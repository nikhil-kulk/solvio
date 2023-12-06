use std::collections::HashSet;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use common::types::{PointOffsetType, ScoredPointOffset};
use itertools::Itertools;
use sparse::common::sparse_vector::SparseVector;
use sparse::index::inverted_index::inverted_index_ram::InvertedIndexRam;
use sparse::index::inverted_index::InvertedIndex;
use sparse::index::search_context::SearchContext;

use crate::common::operation_error::{check_process_stopped, OperationResult};
use crate::common::operation_time_statistics::ScopeDurationMeasurer;
use crate::data_types::vectors::{QueryVector, VectorRef};
use crate::id_tracker::IdTrackerSS;
use crate::index::field_index::CardinalityEstimation;
use crate::index::query_estimator::adjust_to_available_vectors;
use crate::index::sparse_index::sparse_index_config::SparseIndexConfig;
use crate::index::sparse_index::sparse_search_telemetry::SparseSearchesTelemetry;
use crate::index::struct_payload_index::StructPayloadIndex;
use crate::index::{PayloadIndex, VectorIndex};
use crate::telemetry::VectorIndexSearchesTelemetry;
use crate::types::{Filter, SearchParams, DEFAULT_SPARSE_FULL_SCAN_THRESHOLD};
use crate::vector_storage::quantized::quantized_vectors::QuantizedVectors;
use crate::vector_storage::{
    check_deleted_condition, new_stoppable_raw_scorer, VectorStorage, VectorStorageEnum,
};

pub struct SparseVectorIndex<TInvertedIndex: InvertedIndex> {
    pub config: SparseIndexConfig,
    pub id_tracker: Arc<AtomicRefCell<IdTrackerSS>>,
    pub vector_storage: Arc<AtomicRefCell<VectorStorageEnum>>,
    pub payload_index: Arc<AtomicRefCell<StructPayloadIndex>>,
    path: PathBuf,
    pub inverted_index: TInvertedIndex,
    searches_telemetry: SparseSearchesTelemetry,
}

impl<TInvertedIndex: InvertedIndex> SparseVectorIndex<TInvertedIndex> {
    /// Open a sparse vector index at a given path
    pub fn open(
        config: SparseIndexConfig,
        id_tracker: Arc<AtomicRefCell<IdTrackerSS>>,
        vector_storage: Arc<AtomicRefCell<VectorStorageEnum>>,
        payload_index: Arc<AtomicRefCell<StructPayloadIndex>>,
        path: &Path,
    ) -> OperationResult<Self> {
        // create directory if it does not exist
        create_dir_all(path)?;

        // load config
        let config_path = SparseIndexConfig::get_config_path(path);
        let config = if config_path.exists() {
            SparseIndexConfig::load(&config_path)?
        } else {
            // create new files if they do not exist
            TInvertedIndex::from_ram_index(InvertedIndexRam::empty(), path)?;
            // use provided config if no config file exists
            config
        };

        let searches_telemetry = SparseSearchesTelemetry::new();
        let inverted_index = if let Some(inverted_index) = TInvertedIndex::open(path)? {
            inverted_index
        } else {
            Self::build_inverted_index(
                id_tracker.clone(),
                vector_storage.clone(),
                path,
                &AtomicBool::new(false),
            )?
        };

        let path = path.to_path_buf();
        let index = Self {
            config,
            id_tracker,
            vector_storage,
            payload_index,
            path,
            inverted_index,
            searches_telemetry,
        };
        Ok(index)
    }

    fn save_config(&self) -> OperationResult<()> {
        let config_path = SparseIndexConfig::get_config_path(&self.path);
        self.config.save(&config_path)
    }

    fn build_inverted_index(
        id_tracker: Arc<AtomicRefCell<IdTrackerSS>>,
        vector_storage: Arc<AtomicRefCell<VectorStorageEnum>>,
        path: &Path,
        stopped: &AtomicBool,
    ) -> OperationResult<TInvertedIndex> {
        let borrowed_vector_storage = vector_storage.borrow();
        let borrowed_id_tracker = id_tracker.borrow();
        let deleted_bitslice = borrowed_vector_storage.deleted_vector_bitslice();
        let mut ram_index = InvertedIndexRam::empty();
        let mut index_point_count: usize = 0;
        for id in borrowed_id_tracker.iter_ids_excluding(deleted_bitslice) {
            check_process_stopped(stopped)?;
            let vector = borrowed_vector_storage.get_vector(id);
            let vector: &SparseVector = vector.as_vec_ref().try_into()?;
            // do not index empty vectors
            if vector.is_empty() {
                continue;
            }
            ram_index.upsert(id, vector.to_owned());
            index_point_count += 1;
        }
        // the underlying upsert operation does not guarantee that the indexed vector count is correct
        // so we set the indexed vector count to the number of points we have seen
        ram_index.vector_count = index_point_count;
        // TODO(sparse) this operation loads the entire index into memory which can cause OOM on large storage
        Ok(TInvertedIndex::from_ram_index(ram_index, path)?)
    }

    /// Returns the maximum number of results that can be returned by the index for a given sparse vector
    /// Warning: the cost of this function grows with the number of dimensions in the query vector
    pub fn max_result_count(&self, query_vector: &SparseVector) -> usize {
        let mut unique_record_ids = HashSet::new();
        for dim_id in query_vector.indices.iter() {
            if let Some(posting_list) = self.inverted_index.get(dim_id) {
                for element in posting_list.elements.iter() {
                    unique_record_ids.insert(element.record_id);
                }
            }
        }
        unique_record_ids.len()
    }

    fn get_query_cardinality(&self, filter: &Filter) -> CardinalityEstimation {
        let vector_storage = self.vector_storage.borrow();
        let id_tracker = self.id_tracker.borrow();
        let payload_index = self.payload_index.borrow();
        let available_vector_count = vector_storage.available_vector_count();
        let query_point_cardinality = payload_index.estimate_cardinality(filter);
        adjust_to_available_vectors(
            query_point_cardinality,
            available_vector_count,
            id_tracker.available_point_count(),
        )
    }

    // Search using raw scorer
    fn search_scored(
        &self,
        query_vector: &QueryVector,
        filter: Option<&Filter>,
        top: usize,
        is_stopped: &AtomicBool,
        prefiltered_points: &mut Option<Vec<PointOffsetType>>,
    ) -> OperationResult<Vec<ScoredPointOffset>> {
        let vector_storage = self.vector_storage.borrow();
        let id_tracker = self.id_tracker.borrow();
        let raw_scorer = new_stoppable_raw_scorer(
            query_vector.clone(),
            &vector_storage,
            id_tracker.deleted_point_bitslice(),
            is_stopped,
        )?;
        match filter {
            Some(filter) => {
                let payload_index = self.payload_index.borrow();
                let mut filtered_points = match prefiltered_points {
                    Some(filtered_points) => filtered_points.iter().copied(),
                    None => {
                        let filtered_points = payload_index.query_points(filter);
                        *prefiltered_points = Some(filtered_points);
                        prefiltered_points.as_ref().unwrap().iter().copied()
                    }
                };
                Ok(raw_scorer.peek_top_iter(&mut filtered_points, top))
            }
            None => Ok(raw_scorer.peek_top_all(top)),
        }
    }

    pub fn search_plain(
        &self,
        sparse_vector: &SparseVector,
        filter: &Filter,
        top: usize,
        is_stopped: &AtomicBool,
        prefiltered_points: &mut Option<Vec<PointOffsetType>>,
    ) -> OperationResult<Vec<ScoredPointOffset>> {
        let vector_storage = self.vector_storage.borrow();
        let id_tracker = self.id_tracker.borrow();
        let payload_index = self.payload_index.borrow();

        let deleted_point_bitslice = id_tracker.deleted_point_bitslice();
        let deleted_vectors = vector_storage.deleted_vector_bitslice();

        let ids = match prefiltered_points {
            Some(filtered_points) => filtered_points.iter(),
            None => {
                let filtered_points = payload_index.query_points(filter);
                *prefiltered_points = Some(filtered_points);
                prefiltered_points.as_ref().unwrap().iter()
            }
        }
        .copied()
        .filter(|&idx| check_deleted_condition(idx, deleted_vectors, deleted_point_bitslice))
        .collect_vec();

        let mut search_context = SearchContext::new(
            sparse_vector.to_owned(),
            top,
            &self.inverted_index,
            is_stopped,
        );
        Ok(search_context.plain_search(&ids))
    }

    // search using sparse vector inverted index
    fn search_sparse(
        &self,
        sparse_vector: &SparseVector,
        filter: Option<&Filter>,
        top: usize,
        is_stopped: &AtomicBool,
    ) -> OperationResult<Vec<ScoredPointOffset>> {
        let vector_storage = self.vector_storage.borrow();
        let id_tracker = self.id_tracker.borrow();
        let deleted_point_bitslice = id_tracker.deleted_point_bitslice();
        let deleted_vectors = vector_storage.deleted_vector_bitslice();

        let not_deleted_condition = |idx: PointOffsetType| -> bool {
            check_deleted_condition(idx, deleted_vectors, deleted_point_bitslice)
        };
        let mut search_context = SearchContext::new(
            sparse_vector.to_owned(),
            top,
            &self.inverted_index,
            is_stopped,
        );

        match filter {
            Some(filter) => {
                let payload_index = self.payload_index.borrow();
                let filter_context = payload_index.filter_context(filter);
                let matches_filter_condition = |idx: PointOffsetType| -> bool {
                    not_deleted_condition(idx) && filter_context.check(idx)
                };
                Ok(search_context.search(&matches_filter_condition))
            }
            None => Ok(search_context.search(&not_deleted_condition)),
        }
    }

    fn search_nearest_query(
        &self,
        vector: &SparseVector,
        filter: Option<&Filter>,
        top: usize,
        is_stopped: &AtomicBool,
        prefiltered_points: &mut Option<Vec<PointOffsetType>>,
    ) -> OperationResult<Vec<ScoredPointOffset>> {
        let mut vector = vector.clone();
        vector.sort_by_indices();

        match filter {
            Some(filter) => {
                // if cardinality is small - use plain search
                let query_cardinality = self.get_query_cardinality(filter);
                let threshold = self
                    .config
                    .full_scan_threshold
                    .unwrap_or(DEFAULT_SPARSE_FULL_SCAN_THRESHOLD);
                if query_cardinality.max < threshold {
                    let _timer =
                        ScopeDurationMeasurer::new(&self.searches_telemetry.small_cardinality);
                    self.search_plain(&vector, filter, top, is_stopped, prefiltered_points)
                } else {
                    let _timer =
                        ScopeDurationMeasurer::new(&self.searches_telemetry.filtered_sparse);
                    self.search_sparse(&vector, Some(filter), top, is_stopped)
                }
            }
            None => {
                let _timer = ScopeDurationMeasurer::new(&self.searches_telemetry.unfiltered_sparse);
                self.search_sparse(&vector, filter, top, is_stopped)
            }
        }
    }

    pub fn search_query(
        &self,
        query_vector: &QueryVector,
        filter: Option<&Filter>,
        top: usize,
        is_stopped: &AtomicBool,
        prefiltered_points: &mut Option<Vec<PointOffsetType>>,
    ) -> OperationResult<Vec<ScoredPointOffset>> {
        if top == 0 {
            return Ok(vec![]);
        }

        match query_vector {
            QueryVector::Nearest(vector) => self.search_nearest_query(
                vector.try_into()?,
                filter,
                top,
                is_stopped,
                prefiltered_points,
            ),
            QueryVector::Recommend(_) | QueryVector::Discovery(_) | QueryVector::Context(_) => {
                let _timer = if filter.is_some() {
                    ScopeDurationMeasurer::new(&self.searches_telemetry.filtered_plain)
                } else {
                    ScopeDurationMeasurer::new(&self.searches_telemetry.unfiltered_plain)
                };
                self.search_scored(query_vector, filter, top, is_stopped, prefiltered_points)
            }
        }
    }
}

impl<TInvertedIndex: InvertedIndex> VectorIndex for SparseVectorIndex<TInvertedIndex> {
    fn search(
        &self,
        vectors: &[&QueryVector],
        filter: Option<&Filter>,
        top: usize,
        _params: Option<&SearchParams>,
        is_stopped: &AtomicBool,
    ) -> OperationResult<Vec<Vec<ScoredPointOffset>>> {
        let mut results = Vec::with_capacity(vectors.len());
        let mut prefiltered_points = None;
        for vector in vectors {
            check_process_stopped(is_stopped)?;
            let search_results =
                self.search_query(vector, filter, top, is_stopped, &mut prefiltered_points)?;
            results.push(search_results);
        }
        Ok(results)
    }

    fn build_index(&mut self, stopped: &AtomicBool) -> OperationResult<()> {
        self.inverted_index = Self::build_inverted_index(
            self.id_tracker.clone(),
            self.vector_storage.clone(),
            &self.path,
            stopped,
        )?;

        // save config to mark successful build
        self.save_config()?;
        Ok(())
    }

    fn get_telemetry_data(&self) -> VectorIndexSearchesTelemetry {
        let tm = &self.searches_telemetry;
        tm.into()
    }

    fn files(&self) -> Vec<PathBuf> {
        self.inverted_index.files()
    }

    fn indexed_vector_count(&self) -> usize {
        self.inverted_index.vector_count()
    }

    fn update_vector(&mut self, id: PointOffsetType, vector: VectorRef) -> OperationResult<()> {
        let vector: &SparseVector = vector.try_into()?;
        // do not upsert empty vectors into the index
        if !vector.is_empty() {
            self.inverted_index.upsert(id, vector.clone());
        }
        Ok(())
    }

    fn set_quantized_vectors(
        &mut self,
        quantized_vectors: Option<Arc<AtomicRefCell<QuantizedVectors>>>,
    ) {
        debug_assert!(
            quantized_vectors.is_none(),
            "Sparse index does not support quantization"
        );
    }
}
