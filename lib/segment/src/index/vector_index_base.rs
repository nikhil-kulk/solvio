use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use common::cpu::CpuPermit;
use common::types::{PointOffsetType, ScoredPointOffset, TelemetryDetail};
use sparse::index::inverted_index::inverted_index_compressed_immutable_ram::InvertedIndexImmutableRam;
use sparse::index::inverted_index::inverted_index_compressed_mmap::InvertedIndexMmap;
use sparse::index::inverted_index::inverted_index_ram::InvertedIndexRam;

use super::hnsw_index::graph_links::{GraphLinksMmap, GraphLinksRam};
use super::hnsw_index::hnsw::HNSWIndex;
use super::plain_payload_index::PlainIndex;
use super::sparse_index::sparse_vector_index::SparseVectorIndex;
use crate::common::operation_error::OperationResult;
use crate::data_types::query_context::VectorQueryContext;
use crate::data_types::vectors::{QueryVector, VectorRef};
use crate::telemetry::VectorIndexSearchesTelemetry;
use crate::types::{Filter, SearchParams};

/// Trait for vector searching
pub trait VectorIndex {
    /// Return list of Ids with fitting
    fn search(
        &self,
        vectors: &[&QueryVector],
        filter: Option<&Filter>,
        top: usize,
        params: Option<&SearchParams>,
        query_context: &VectorQueryContext,
    ) -> OperationResult<Vec<Vec<ScoredPointOffset>>>;

    /// Force internal index rebuild.
    fn build_index(&mut self, permit: Arc<CpuPermit>, stopped: &AtomicBool) -> OperationResult<()> {
        self.build_index_with_progress(permit, stopped, || ())
    }

    /// Force internal index rebuild.
    fn build_index_with_progress(
        &mut self,
        permit: Arc<CpuPermit>,
        stopped: &AtomicBool,
        tick_progress: impl FnMut(),
    ) -> OperationResult<()>;

    fn get_telemetry_data(&self, detail: TelemetryDetail) -> VectorIndexSearchesTelemetry;

    fn files(&self) -> Vec<PathBuf>;

    /// The number of indexed vectors, currently accessible
    fn indexed_vector_count(&self) -> usize;

    /// Update index for a single vector
    fn update_vector(&mut self, id: PointOffsetType, vector: VectorRef) -> OperationResult<()>;
}

pub enum VectorIndexEnum {
    Plain(PlainIndex),
    HnswRam(HNSWIndex<GraphLinksRam>),
    HnswMmap(HNSWIndex<GraphLinksMmap>),
    SparseRam(SparseVectorIndex<InvertedIndexRam>),
    SparseImmutableRam(SparseVectorIndex<InvertedIndexImmutableRam<f32>>),
    SparseMmap(SparseVectorIndex<InvertedIndexMmap<f32>>),
}

impl VectorIndexEnum {
    pub fn is_index(&self) -> bool {
        match self {
            Self::Plain(_) => false,
            Self::HnswRam(_) => true,
            Self::HnswMmap(_) => true,
            Self::SparseRam(_) => true,
            Self::SparseImmutableRam(_) => true,
            Self::SparseMmap(_) => true,
        }
    }
}

impl VectorIndex for VectorIndexEnum {
    fn search(
        &self,
        vectors: &[&QueryVector],
        filter: Option<&Filter>,
        top: usize,
        params: Option<&SearchParams>,
        query_context: &VectorQueryContext,
    ) -> OperationResult<Vec<Vec<ScoredPointOffset>>> {
        match self {
            VectorIndexEnum::Plain(index) => {
                index.search(vectors, filter, top, params, query_context)
            }
            VectorIndexEnum::HnswRam(index) => {
                index.search(vectors, filter, top, params, query_context)
            }
            VectorIndexEnum::HnswMmap(index) => {
                index.search(vectors, filter, top, params, query_context)
            }
            VectorIndexEnum::SparseRam(index) => {
                index.search(vectors, filter, top, params, query_context)
            }
            VectorIndexEnum::SparseImmutableRam(index) => {
                index.search(vectors, filter, top, params, query_context)
            }
            VectorIndexEnum::SparseMmap(index) => {
                index.search(vectors, filter, top, params, query_context)
            }
        }
    }

    fn build_index_with_progress(
        &mut self,
        permit: Arc<CpuPermit>,
        stopped: &AtomicBool,
        tick_progress: impl FnMut(),
    ) -> OperationResult<()> {
        match self {
            VectorIndexEnum::Plain(index) => {
                index.build_index_with_progress(permit, stopped, tick_progress)
            }
            VectorIndexEnum::HnswRam(index) => {
                index.build_index_with_progress(permit, stopped, tick_progress)
            }
            VectorIndexEnum::HnswMmap(index) => {
                index.build_index_with_progress(permit, stopped, tick_progress)
            }
            VectorIndexEnum::SparseRam(index) => {
                index.build_index_with_progress(permit, stopped, tick_progress)
            }
            VectorIndexEnum::SparseImmutableRam(index) => {
                index.build_index_with_progress(permit, stopped, tick_progress)
            }
            VectorIndexEnum::SparseMmap(index) => {
                index.build_index_with_progress(permit, stopped, tick_progress)
            }
        }
    }

    fn get_telemetry_data(&self, detail: TelemetryDetail) -> VectorIndexSearchesTelemetry {
        match self {
            VectorIndexEnum::Plain(index) => index.get_telemetry_data(detail),
            VectorIndexEnum::HnswRam(index) => index.get_telemetry_data(detail),
            VectorIndexEnum::HnswMmap(index) => index.get_telemetry_data(detail),
            VectorIndexEnum::SparseRam(index) => index.get_telemetry_data(detail),
            VectorIndexEnum::SparseImmutableRam(index) => index.get_telemetry_data(detail),
            VectorIndexEnum::SparseMmap(index) => index.get_telemetry_data(detail),
        }
    }

    fn files(&self) -> Vec<PathBuf> {
        match self {
            VectorIndexEnum::Plain(index) => index.files(),
            VectorIndexEnum::HnswRam(index) => index.files(),
            VectorIndexEnum::HnswMmap(index) => index.files(),
            VectorIndexEnum::SparseRam(index) => index.files(),
            VectorIndexEnum::SparseImmutableRam(index) => index.files(),
            VectorIndexEnum::SparseMmap(index) => index.files(),
        }
    }

    fn indexed_vector_count(&self) -> usize {
        match self {
            Self::Plain(index) => index.indexed_vector_count(),
            Self::HnswRam(index) => index.indexed_vector_count(),
            Self::HnswMmap(index) => index.indexed_vector_count(),
            Self::SparseRam(index) => index.indexed_vector_count(),
            Self::SparseImmutableRam(index) => index.indexed_vector_count(),
            Self::SparseMmap(index) => index.indexed_vector_count(),
        }
    }

    fn update_vector(&mut self, id: PointOffsetType, vector: VectorRef) -> OperationResult<()> {
        match self {
            Self::Plain(index) => index.update_vector(id, vector),
            Self::HnswRam(index) => index.update_vector(id, vector),
            Self::HnswMmap(index) => index.update_vector(id, vector),
            Self::SparseRam(index) => index.update_vector(id, vector),
            Self::SparseImmutableRam(index) => index.update_vector(id, vector),
            Self::SparseMmap(index) => index.update_vector(id, vector),
        }
    }
}
