use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use atomic_refcell::AtomicRefCell;
use common::budget::ResourcePermit;
use common::flags::FeatureFlags;
use io::storage_version::StorageVersion;
use log::info;
use parking_lot::Mutex;
#[cfg(feature = "rocksdb")]
use parking_lot::RwLock;
use rand::Rng;
#[cfg(feature = "rocksdb")]
use rocksdb::DB;
use serde::Deserialize;
use uuid::Uuid;

use super::rocksdb_builder::RocksDbBuilder;
use crate::common::operation_error::{OperationError, OperationResult, check_process_stopped};
use crate::data_types::vectors::DEFAULT_VECTOR_NAME;
use crate::id_tracker::immutable_id_tracker::ImmutableIdTracker;
use crate::id_tracker::mutable_id_tracker::MutableIdTracker;
#[cfg(feature = "rocksdb")]
use crate::id_tracker::simple_id_tracker::SimpleIdTracker;
use crate::id_tracker::{IdTracker, IdTrackerEnum, IdTrackerSS};
use crate::index::VectorIndexEnum;
use crate::index::hnsw_index::gpu::gpu_devices_manager::LockedGpuDevice;
use crate::index::hnsw_index::hnsw::{HNSWIndex, HnswIndexOpenArgs};
use crate::index::plain_vector_index::PlainVectorIndex;
use crate::index::sparse_index::sparse_index_config::SparseIndexType;
use crate::index::sparse_index::sparse_vector_index::{
    self, SparseVectorIndex, SparseVectorIndexOpenArgs,
};
use crate::index::struct_payload_index::StructPayloadIndex;
use crate::payload_storage::mmap_payload_storage::MmapPayloadStorage;
use crate::payload_storage::on_disk_payload_storage::OnDiskPayloadStorage;
use crate::payload_storage::payload_storage_enum::PayloadStorageEnum;
use crate::payload_storage::simple_payload_storage::SimplePayloadStorage;
use crate::segment::{SEGMENT_STATE_FILE, Segment, SegmentVersion, VectorData};
use crate::types::{
    Distance, Indexes, PayloadStorageType, SegmentConfig, SegmentState, SegmentType, SeqNumberType,
    SparseVectorStorageType, VectorDataConfig, VectorName, VectorStorageDatatype,
    VectorStorageType,
};
use crate::vector_storage::dense::appendable_dense_vector_storage::{
    open_appendable_in_ram_vector_storage, open_appendable_in_ram_vector_storage_byte,
    open_appendable_in_ram_vector_storage_half, open_appendable_memmap_vector_storage,
    open_appendable_memmap_vector_storage_byte, open_appendable_memmap_vector_storage_half,
};
use crate::vector_storage::dense::memmap_dense_vector_storage::{
    open_memmap_vector_storage, open_memmap_vector_storage_byte, open_memmap_vector_storage_half,
};
#[cfg(feature = "rocksdb")]
use crate::vector_storage::dense::simple_dense_vector_storage::{
    open_simple_dense_byte_vector_storage, open_simple_dense_half_vector_storage,
    open_simple_dense_vector_storage,
};
use crate::vector_storage::multi_dense::appendable_mmap_multi_dense_vector_storage::{
    open_appendable_in_ram_multi_vector_storage, open_appendable_in_ram_multi_vector_storage_byte,
    open_appendable_in_ram_multi_vector_storage_half, open_appendable_memmap_multi_vector_storage,
    open_appendable_memmap_multi_vector_storage_byte,
    open_appendable_memmap_multi_vector_storage_half,
};
#[cfg(feature = "rocksdb")]
use crate::vector_storage::multi_dense::simple_multi_dense_vector_storage::{
    open_simple_multi_dense_vector_storage, open_simple_multi_dense_vector_storage_byte,
    open_simple_multi_dense_vector_storage_half,
};
use crate::vector_storage::quantized::quantized_vectors::QuantizedVectors;
use crate::vector_storage::sparse::mmap_sparse_vector_storage::MmapSparseVectorStorage;
use crate::vector_storage::{VectorStorage, VectorStorageEnum};

pub const PAYLOAD_INDEX_PATH: &str = "payload_index";
pub const VECTOR_STORAGE_PATH: &str = "vector_storage";
pub const VECTOR_INDEX_PATH: &str = "vector_index";

fn sp<T>(t: T) -> Arc<AtomicRefCell<T>> {
    Arc::new(AtomicRefCell::new(t))
}

pub fn get_vector_name_with_prefix(prefix: &str, vector_name: &VectorName) -> String {
    if !vector_name.is_empty() {
        format!("{prefix}-{vector_name}")
    } else {
        prefix.to_owned()
    }
}

pub fn get_vector_storage_path(segment_path: &Path, vector_name: &VectorName) -> PathBuf {
    segment_path.join(get_vector_name_with_prefix(
        VECTOR_STORAGE_PATH,
        vector_name,
    ))
}

pub fn get_vector_index_path(segment_path: &Path, vector_name: &VectorName) -> PathBuf {
    segment_path.join(get_vector_name_with_prefix(VECTOR_INDEX_PATH, vector_name))
}

pub(crate) fn open_vector_storage(
    #[cfg(feature = "rocksdb")] db_builder: &mut RocksDbBuilder,
    vector_config: &VectorDataConfig,
    #[cfg(feature = "rocksdb")] stopped: &AtomicBool,
    vector_storage_path: &Path,
    #[cfg(feature = "rocksdb")] vector_name: &VectorName,
) -> OperationResult<VectorStorageEnum> {
    let storage_element_type = vector_config.datatype.unwrap_or_default();

    match vector_config.storage_type {
        // In memory - RocksDB disabled
        #[cfg(not(feature = "rocksdb"))]
        VectorStorageType::Memory => Err(OperationError::service_error(
            "Failed to load 'Memory' storage type, RocksDB disabled in this Solvio version",
        )),

        // In memory - RocksDB enabled
        #[cfg(feature = "rocksdb")]
        VectorStorageType::Memory => {
            use crate::common::rocksdb_wrapper::DB_VECTOR_CF;

            let db_column_name = get_vector_name_with_prefix(DB_VECTOR_CF, vector_name);

            if let Some(multi_vec_config) = &vector_config.multivector_config {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_simple_multi_dense_vector_storage(
                        db_builder.require()?,
                        &db_column_name,
                        vector_config.size,
                        vector_config.distance,
                        *multi_vec_config,
                        stopped,
                    ),
                    VectorStorageDatatype::Uint8 => open_simple_multi_dense_vector_storage_byte(
                        db_builder.require()?,
                        &db_column_name,
                        vector_config.size,
                        vector_config.distance,
                        *multi_vec_config,
                        stopped,
                    ),
                    VectorStorageDatatype::Float16 => open_simple_multi_dense_vector_storage_half(
                        db_builder.require()?,
                        &db_column_name,
                        vector_config.size,
                        vector_config.distance,
                        *multi_vec_config,
                        stopped,
                    ),
                }
            } else {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_simple_dense_vector_storage(
                        db_builder.require()?,
                        &db_column_name,
                        vector_config.size,
                        vector_config.distance,
                        stopped,
                    ),
                    VectorStorageDatatype::Uint8 => open_simple_dense_byte_vector_storage(
                        db_builder.require()?,
                        &db_column_name,
                        vector_config.size,
                        vector_config.distance,
                        stopped,
                    ),
                    VectorStorageDatatype::Float16 => open_simple_dense_half_vector_storage(
                        db_builder.require()?,
                        &db_column_name,
                        vector_config.size,
                        vector_config.distance,
                        stopped,
                    ),
                }
            }
        }
        // Mmap on disk, not appendable
        VectorStorageType::Mmap => {
            if let Some(multi_vec_config) = &vector_config.multivector_config {
                // there are no mmap multi vector storages, appendable only
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_appendable_memmap_multi_vector_storage(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                        *multi_vec_config,
                    ),
                    VectorStorageDatatype::Uint8 => {
                        open_appendable_memmap_multi_vector_storage_byte(
                            vector_storage_path,
                            vector_config.size,
                            vector_config.distance,
                            *multi_vec_config,
                        )
                    }
                    VectorStorageDatatype::Float16 => {
                        open_appendable_memmap_multi_vector_storage_half(
                            vector_storage_path,
                            vector_config.size,
                            vector_config.distance,
                            *multi_vec_config,
                        )
                    }
                }
            } else {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_memmap_vector_storage(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                    VectorStorageDatatype::Uint8 => open_memmap_vector_storage_byte(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                    VectorStorageDatatype::Float16 => open_memmap_vector_storage_half(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                }
            }
        }
        // Chunked mmap on disk, appendable
        VectorStorageType::ChunkedMmap => {
            if let Some(multi_vec_config) = &vector_config.multivector_config {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_appendable_memmap_multi_vector_storage(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                        *multi_vec_config,
                    ),
                    VectorStorageDatatype::Uint8 => {
                        open_appendable_memmap_multi_vector_storage_byte(
                            vector_storage_path,
                            vector_config.size,
                            vector_config.distance,
                            *multi_vec_config,
                        )
                    }
                    VectorStorageDatatype::Float16 => {
                        open_appendable_memmap_multi_vector_storage_half(
                            vector_storage_path,
                            vector_config.size,
                            vector_config.distance,
                            *multi_vec_config,
                        )
                    }
                }
            } else {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_appendable_memmap_vector_storage(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                    VectorStorageDatatype::Uint8 => open_appendable_memmap_vector_storage_byte(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                    VectorStorageDatatype::Float16 => open_appendable_memmap_vector_storage_half(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                }
            }
        }
        VectorStorageType::InRamChunkedMmap => {
            if let Some(multi_vec_config) = &vector_config.multivector_config {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_appendable_in_ram_multi_vector_storage(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                        *multi_vec_config,
                    ),
                    VectorStorageDatatype::Uint8 => {
                        open_appendable_in_ram_multi_vector_storage_byte(
                            vector_storage_path,
                            vector_config.size,
                            vector_config.distance,
                            *multi_vec_config,
                        )
                    }
                    VectorStorageDatatype::Float16 => {
                        open_appendable_in_ram_multi_vector_storage_half(
                            vector_storage_path,
                            vector_config.size,
                            vector_config.distance,
                            *multi_vec_config,
                        )
                    }
                }
            } else {
                match storage_element_type {
                    VectorStorageDatatype::Float32 => open_appendable_in_ram_vector_storage(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                    VectorStorageDatatype::Uint8 => open_appendable_in_ram_vector_storage_byte(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                    VectorStorageDatatype::Float16 => open_appendable_in_ram_vector_storage_half(
                        vector_storage_path,
                        vector_config.size,
                        vector_config.distance,
                    ),
                }
            }
        }
    }
}

pub(crate) fn create_payload_storage(
    db_builder: &mut RocksDbBuilder,
    segment_path: &Path,
    config: &SegmentConfig,
) -> OperationResult<PayloadStorageEnum> {
    let payload_storage = match config.payload_storage_type {
        PayloadStorageType::InMemory => {
            PayloadStorageEnum::from(SimplePayloadStorage::open(db_builder.require()?)?)
        }
        PayloadStorageType::OnDisk => {
            PayloadStorageEnum::from(OnDiskPayloadStorage::open(db_builder.require()?)?)
        }
        PayloadStorageType::Mmap => {
            PayloadStorageEnum::from(MmapPayloadStorage::open_or_create(segment_path)?)
        }
    };
    Ok(payload_storage)
}

pub(crate) fn create_mutable_id_tracker(segment_path: &Path) -> OperationResult<MutableIdTracker> {
    MutableIdTracker::open(segment_path)
}

#[cfg(feature = "rocksdb")]
pub(crate) fn create_rocksdb_id_tracker(
    database: Arc<RwLock<DB>>,
) -> OperationResult<SimpleIdTracker> {
    SimpleIdTracker::open(database)
}

pub(crate) fn create_immutable_id_tracker(
    segment_path: &Path,
) -> OperationResult<ImmutableIdTracker> {
    ImmutableIdTracker::open(segment_path)
}

pub(crate) fn get_payload_index_path(segment_path: &Path) -> PathBuf {
    segment_path.join(PAYLOAD_INDEX_PATH)
}

pub(crate) struct VectorIndexOpenArgs<'a> {
    pub path: &'a Path,
    pub id_tracker: Arc<AtomicRefCell<IdTrackerSS>>,
    pub vector_storage: Arc<AtomicRefCell<VectorStorageEnum>>,
    pub payload_index: Arc<AtomicRefCell<StructPayloadIndex>>,
    pub quantized_vectors: Arc<AtomicRefCell<Option<QuantizedVectors>>>,
}

pub struct VectorIndexBuildArgs<'a, R: Rng + ?Sized> {
    pub permit: Arc<ResourcePermit>,
    /// Vector indices from other segments, used to speed up index building.
    /// May or may not contain the same vectors.
    pub old_indices: &'a [Arc<AtomicRefCell<VectorIndexEnum>>],
    pub gpu_device: Option<&'a LockedGpuDevice<'a>>,
    pub rng: &'a mut R,
    pub stopped: &'a AtomicBool,
    pub feature_flags: FeatureFlags,
}

pub(crate) fn open_vector_index(
    vector_config: &VectorDataConfig,
    open_args: VectorIndexOpenArgs,
) -> OperationResult<VectorIndexEnum> {
    let VectorIndexOpenArgs {
        path,
        id_tracker,
        vector_storage,
        payload_index,
        quantized_vectors,
    } = open_args;
    Ok(match &vector_config.index {
        Indexes::Plain {} => VectorIndexEnum::Plain(PlainVectorIndex::new(
            id_tracker,
            vector_storage,
            payload_index,
        )),
        Indexes::Hnsw(hnsw_config) => VectorIndexEnum::Hnsw(HNSWIndex::open(HnswIndexOpenArgs {
            path,
            id_tracker,
            vector_storage,
            quantized_vectors,
            payload_index,
            hnsw_config: hnsw_config.clone(),
        })?),
    })
}

pub(crate) fn build_vector_index<R: Rng + ?Sized>(
    vector_config: &VectorDataConfig,
    open_args: VectorIndexOpenArgs,
    build_args: VectorIndexBuildArgs<R>,
) -> OperationResult<VectorIndexEnum> {
    let VectorIndexOpenArgs {
        path,
        id_tracker,
        vector_storage,
        payload_index,
        quantized_vectors,
    } = open_args;
    Ok(match &vector_config.index {
        Indexes::Plain {} => VectorIndexEnum::Plain(PlainVectorIndex::new(
            id_tracker,
            vector_storage,
            payload_index,
        )),
        Indexes::Hnsw(hnsw_config) => VectorIndexEnum::Hnsw(HNSWIndex::build(
            HnswIndexOpenArgs {
                path,
                id_tracker,
                vector_storage,
                quantized_vectors,
                payload_index,
                hnsw_config: hnsw_config.clone(),
            },
            build_args,
        )?),
    })
}

#[cfg(feature = "testing")]
pub fn create_sparse_vector_index_test(
    args: SparseVectorIndexOpenArgs<impl FnMut()>,
) -> OperationResult<VectorIndexEnum> {
    create_sparse_vector_index(args)
}

pub(crate) fn create_sparse_vector_index(
    args: SparseVectorIndexOpenArgs<impl FnMut()>,
) -> OperationResult<VectorIndexEnum> {
    let vector_index = match (
        args.config.index_type,
        args.config.datatype.unwrap_or_default(),
        sparse_vector_index::USE_COMPRESSED,
    ) {
        (_, a @ (VectorStorageDatatype::Float16 | VectorStorageDatatype::Uint8), false) => {
            Err(OperationError::ValidationError {
                description: format!("{a:?} datatype is not supported"),
            })?
        }

        (SparseIndexType::MutableRam, _, _) => {
            VectorIndexEnum::SparseRam(SparseVectorIndex::open(args)?)
        }

        // Non-compressed
        (SparseIndexType::ImmutableRam, VectorStorageDatatype::Float32, false) => {
            VectorIndexEnum::SparseImmutableRam(SparseVectorIndex::open(args)?)
        }
        (SparseIndexType::Mmap, VectorStorageDatatype::Float32, false) => {
            VectorIndexEnum::SparseMmap(SparseVectorIndex::open(args)?)
        }

        // Compressed
        (SparseIndexType::ImmutableRam, VectorStorageDatatype::Float32, true) => {
            VectorIndexEnum::SparseCompressedImmutableRamF32(SparseVectorIndex::open(args)?)
        }
        (SparseIndexType::Mmap, VectorStorageDatatype::Float32, true) => {
            VectorIndexEnum::SparseCompressedMmapF32(SparseVectorIndex::open(args)?)
        }
        (SparseIndexType::ImmutableRam, VectorStorageDatatype::Float16, true) => {
            VectorIndexEnum::SparseCompressedImmutableRamF16(SparseVectorIndex::open(args)?)
        }
        (SparseIndexType::Mmap, VectorStorageDatatype::Float16, true) => {
            VectorIndexEnum::SparseCompressedMmapF16(SparseVectorIndex::open(args)?)
        }
        (SparseIndexType::ImmutableRam, VectorStorageDatatype::Uint8, true) => {
            VectorIndexEnum::SparseCompressedImmutableRamU8(SparseVectorIndex::open(args)?)
        }
        (SparseIndexType::Mmap, VectorStorageDatatype::Uint8, true) => {
            VectorIndexEnum::SparseCompressedMmapU8(SparseVectorIndex::open(args)?)
        }
    };

    Ok(vector_index)
}

pub(crate) fn create_sparse_vector_storage(
    #[cfg(feature = "rocksdb")] db_builder: &mut RocksDbBuilder,
    path: &Path,
    #[cfg(feature = "rocksdb")] vector_name: &VectorName,
    storage_type: &SparseVectorStorageType,
    #[cfg(feature = "rocksdb")] stopped: &AtomicBool,
) -> OperationResult<VectorStorageEnum> {
    match storage_type {
        #[cfg(feature = "rocksdb")]
        SparseVectorStorageType::OnDisk => {
            use crate::common::rocksdb_wrapper::DB_VECTOR_CF;
            use crate::vector_storage::sparse::simple_sparse_vector_storage::open_simple_sparse_vector_storage;

            let db_column_name = get_vector_name_with_prefix(DB_VECTOR_CF, vector_name);
            open_simple_sparse_vector_storage(db_builder.require()?, &db_column_name, stopped)
        }
        SparseVectorStorageType::Mmap => {
            let mmap_storage = MmapSparseVectorStorage::open_or_create(path)?;
            Ok(VectorStorageEnum::SparseMmap(mmap_storage))
        }
    }
}

fn create_segment(
    version: Option<SeqNumberType>,
    segment_path: &Path,
    config: &SegmentConfig,
    stopped: &AtomicBool,
) -> OperationResult<Segment> {
    let mut db_builder = RocksDbBuilder::new(segment_path, config)?;

    let payload_storage = sp(create_payload_storage(
        &mut db_builder,
        segment_path,
        config,
    )?);

    let appendable_flag = config.is_appendable();

    let use_mutable_id_tracker =
        appendable_flag || !ImmutableIdTracker::mappings_file_path(segment_path).is_file();
    let id_tracker = create_segment_id_tracker(
        use_mutable_id_tracker,
        segment_path,
        #[cfg(feature = "rocksdb")]
        &mut db_builder,
    )?;

    let mut vector_storages = HashMap::new();

    for (vector_name, vector_config) in &config.vector_data {
        let vector_storage_path = get_vector_storage_path(segment_path, vector_name);

        // Select suitable vector storage type based on configuration
        let vector_storage = sp(open_vector_storage(
            #[cfg(feature = "rocksdb")]
            &mut db_builder,
            vector_config,
            #[cfg(feature = "rocksdb")]
            stopped,
            &vector_storage_path,
            #[cfg(feature = "rocksdb")]
            vector_name,
        )?);

        vector_storages.insert(vector_name.to_owned(), vector_storage);
    }

    for (vector_name, sparse_config) in config.sparse_vector_data.iter() {
        let vector_storage_path = get_vector_storage_path(segment_path, vector_name);

        // Select suitable sparse vector storage type based on configuration
        let vector_storage = sp(create_sparse_vector_storage(
            #[cfg(feature = "rocksdb")]
            &mut db_builder,
            &vector_storage_path,
            #[cfg(feature = "rocksdb")]
            vector_name,
            &sparse_config.storage_type,
            #[cfg(feature = "rocksdb")]
            stopped,
        )?);

        vector_storages.insert(vector_name.to_owned(), vector_storage);
    }

    let payload_index_path = get_payload_index_path(segment_path);
    let payload_index: Arc<AtomicRefCell<StructPayloadIndex>> = sp(StructPayloadIndex::open(
        payload_storage.clone(),
        id_tracker.clone(),
        vector_storages.clone(),
        &payload_index_path,
        appendable_flag,
    )?);

    let mut vector_data = HashMap::new();
    for (vector_name, vector_config) in &config.vector_data {
        let vector_storage_path = get_vector_storage_path(segment_path, vector_name);
        let vector_storage = vector_storages.remove(vector_name).unwrap();

        let vector_index_path = get_vector_index_path(segment_path, vector_name);
        // Warn when number of points between ID tracker and storage differs
        let point_count = id_tracker.borrow().total_point_count();
        let vector_count = vector_storage.borrow().total_vector_count();
        if vector_count != point_count {
            log::debug!(
                "Mismatch of point and vector counts ({point_count} != {vector_count}, storage: {})",
                vector_storage_path.display(),
            );
        }

        let quantized_vectors = sp(if config.quantization_config(vector_name).is_some() {
            let quantized_data_path = vector_storage_path;
            if QuantizedVectors::config_exists(&quantized_data_path) {
                let quantized_vectors =
                    QuantizedVectors::load(&vector_storage.borrow(), &quantized_data_path)?;
                Some(quantized_vectors)
            } else {
                None
            }
        } else {
            None
        });

        let vector_index: Arc<AtomicRefCell<VectorIndexEnum>> = sp(open_vector_index(
            vector_config,
            VectorIndexOpenArgs {
                path: &vector_index_path,
                id_tracker: id_tracker.clone(),
                vector_storage: vector_storage.clone(),
                payload_index: payload_index.clone(),
                quantized_vectors: quantized_vectors.clone(),
            },
        )?);

        check_process_stopped(stopped)?;

        vector_data.insert(
            vector_name.to_owned(),
            VectorData {
                vector_index,
                vector_storage,
                quantized_vectors,
            },
        );
    }

    for (vector_name, sparse_vector_config) in &config.sparse_vector_data {
        let vector_storage_path = get_vector_storage_path(segment_path, vector_name);
        let vector_index_path = get_vector_index_path(segment_path, vector_name);
        let vector_storage = vector_storages.remove(vector_name).unwrap();

        // Warn when number of points between ID tracker and storage differs
        let point_count = id_tracker.borrow().total_point_count();
        let vector_count = vector_storage.borrow().total_vector_count();
        if vector_count != point_count {
            log::debug!(
                "Mismatch of point and vector counts ({point_count} != {vector_count}, storage: {})",
                vector_storage_path.display(),
            );
        }

        let vector_index = sp(create_sparse_vector_index(SparseVectorIndexOpenArgs {
            config: sparse_vector_config.index,
            id_tracker: id_tracker.clone(),
            vector_storage: vector_storage.clone(),
            payload_index: payload_index.clone(),
            path: &vector_index_path,
            stopped,
            tick_progress: || (),
        })?);

        check_process_stopped(stopped)?;

        vector_data.insert(
            vector_name.to_owned(),
            VectorData {
                vector_storage,
                vector_index,
                quantized_vectors: sp(None),
            },
        );
    }

    let segment_type = if config.is_any_vector_indexed() {
        SegmentType::Indexed
    } else {
        SegmentType::Plain
    };

    Ok(Segment {
        version,
        persisted_version: Arc::new(Mutex::new(version)),
        current_path: segment_path.to_owned(),
        id_tracker,
        vector_data,
        segment_type,
        appendable_flag,
        payload_index,
        payload_storage,
        segment_config: config.clone(),
        error_status: None,
        database: db_builder.build(),
        flush_thread: Mutex::new(None),
    })
}

fn create_segment_id_tracker(
    mutable_id_tracker: bool,
    segment_path: &Path,
    #[cfg(feature = "rocksdb")] db_builder: &mut RocksDbBuilder,
) -> OperationResult<Arc<AtomicRefCell<IdTrackerEnum>>> {
    if !mutable_id_tracker {
        return Ok(sp(IdTrackerEnum::ImmutableIdTracker(
            create_immutable_id_tracker(segment_path)?,
        )));
    }

    // Determine whether we use the new (file based) or old (RocksDB) mutable ID tracker
    // Decide based on the feature flag and state on disk
    #[cfg(feature = "rocksdb")]
    {
        use crate::common::rocksdb_wrapper::DB_MAPPING_CF;

        let use_rocksdb_mutable_tracker = if let Some(db) = db_builder.read() {
            // New ID tracker is enabled by default, but we still use the old tracker if we have
            // any mappings stored in RocksDB
            //
            // TODO(1.15 or later): remove this check and use new mutable ID tracker unconditionally
            if let Some(cf) = db.cf_handle(DB_MAPPING_CF) {
                let count = db
                    .property_int_value_cf(cf, rocksdb::properties::ESTIMATE_NUM_KEYS)
                    .map_err(|err| {
                        OperationError::service_error(format!(
                            "Failed to get estimated number of keys from RocksDB: {err}"
                        ))
                    })?
                    .unwrap_or_default();

                count > 0
            } else {
                false
            }
        } else {
            false
        };

        if use_rocksdb_mutable_tracker {
            return Ok(sp(IdTrackerEnum::RocksDbIdTracker(
                create_rocksdb_id_tracker(db_builder.require()?)?,
            )));
        }
    }

    Ok(sp(IdTrackerEnum::MutableIdTracker(
        create_mutable_id_tracker(segment_path)?,
    )))
}

pub fn load_segment(path: &Path, stopped: &AtomicBool) -> OperationResult<Option<Segment>> {
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext == "deleted")
        .unwrap_or(false)
    {
        log::warn!("Segment is marked as deleted, skipping: {}", path.display());
        // Skip deleted segments
        return Ok(None);
    }

    let Some(stored_version) = SegmentVersion::load(path)? else {
        // Assume segment was not properly saved.
        // Server might have crashed before saving the segment fully.
        log::warn!(
            "Segment version file not found, skipping: {}",
            path.display()
        );
        return Ok(None);
    };

    let app_version = SegmentVersion::current();

    if stored_version != app_version {
        info!("Migrating segment {stored_version} -> {app_version}");

        if stored_version > app_version {
            return Err(OperationError::service_error(format!(
                "Data version {stored_version} is newer than application version {app_version}. \
                Please upgrade the application. Compatibility is not guaranteed."
            )));
        }

        if stored_version.major == 0 && stored_version.minor < 3 {
            return Err(OperationError::service_error(format!(
                "Segment version({stored_version}) is not compatible with current version({app_version})"
            )));
        }

        if stored_version.major == 0 && stored_version.minor == 3 {
            let segment_state = load_segment_state_v3(path)?;
            Segment::save_state(&segment_state, path)?;
        } else if stored_version.major == 0 && stored_version.minor <= 5 {
            let segment_state = load_segment_state_v5(path)?;
            Segment::save_state(&segment_state, path)?;
        }

        SegmentVersion::save(path)?
    }

    let segment_state = Segment::load_state(path)?;

    let segment = create_segment(segment_state.version, path, &segment_state.config, stopped)?;

    Ok(Some(segment))
}

pub fn new_segment_path(segments_path: &Path) -> PathBuf {
    segments_path.join(Uuid::new_v4().to_string())
}

/// Build segment instance using given configuration.
/// Builder will generate folder for the segment and store all segment information inside it.
///
/// # Arguments
///
/// * `segments_path` - Path to the segments directory. Segment folder will be created in this directory
/// * `config` - Segment configuration
/// * `ready` - Whether the segment is ready after building; will save segment version
///
/// To load a segment, saving the segment version is required. If `ready` is false, the version
/// will not be stored. Then the segment is skipped on restart when trying to load it again. In
/// that case, the segment version must be stored manually to make it ready.
pub fn build_segment(
    segments_path: &Path,
    config: &SegmentConfig,
    ready: bool,
) -> OperationResult<Segment> {
    let segment_path = new_segment_path(segments_path);

    std::fs::create_dir_all(&segment_path)?;

    let segment = create_segment(None, &segment_path, config, &AtomicBool::new(false))?;
    segment.save_current_state()?;

    // Version is the last file to save, as it will be used to check if segment was built correctly.
    // If it is not saved, segment will be skipped.
    if ready {
        SegmentVersion::save(&segment_path)?;
    }

    Ok(segment)
}

/// Load v0.3.* segment data and migrate to current version
#[allow(deprecated)]
fn load_segment_state_v3(segment_path: &Path) -> OperationResult<SegmentState> {
    use crate::compat::{SegmentConfigV5, StorageTypeV5, VectorDataConfigV5};

    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[deprecated]
    pub struct SegmentStateV3 {
        pub version: SeqNumberType,
        pub config: SegmentConfigV3,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[deprecated]
    pub struct SegmentConfigV3 {
        /// Size of a vectors used
        pub vector_size: usize,
        /// Type of distance function used for measuring distance between vectors
        pub distance: Distance,
        /// Type of index used for search
        pub index: Indexes,
        /// Type of vector storage
        pub storage_type: StorageTypeV5,
        /// Defines payload storage type
        #[serde(default)]
        pub payload_storage_type: PayloadStorageType,
    }

    let path = segment_path.join(SEGMENT_STATE_FILE);

    let mut contents = String::new();

    let mut file = File::open(&path)?;
    file.read_to_string(&mut contents)?;

    serde_json::from_str::<SegmentStateV3>(&contents)
        .map(|state| {
            // Construct V5 version, then convert into current
            let vector_data = VectorDataConfigV5 {
                size: state.config.vector_size,
                distance: state.config.distance,
                hnsw_config: None,
                quantization_config: None,
                on_disk: None,
            };
            let segment_config = SegmentConfigV5 {
                vector_data: HashMap::from([(DEFAULT_VECTOR_NAME.to_owned(), vector_data)]),
                index: state.config.index,
                storage_type: state.config.storage_type,
                payload_storage_type: state.config.payload_storage_type,
                quantization_config: None,
            };

            SegmentState {
                version: Some(state.version),
                config: segment_config.into(),
            }
        })
        .map_err(|err| {
            OperationError::service_error(format!(
                "Failed to read segment {}. Error: {}",
                path.to_str().unwrap(),
                err
            ))
        })
}

/// Load v0.5.0 segment data and migrate to current version
#[allow(deprecated)]
fn load_segment_state_v5(segment_path: &Path) -> OperationResult<SegmentState> {
    use crate::compat::SegmentStateV5;

    let path = segment_path.join(SEGMENT_STATE_FILE);

    let mut contents = String::new();

    let mut file = File::open(&path)?;
    file.read_to_string(&mut contents)?;

    serde_json::from_str::<SegmentStateV5>(&contents)
        .map(Into::into)
        .map_err(|err| {
            OperationError::service_error(format!(
                "Failed to read segment {}. Error: {}",
                path.to_str().unwrap(),
                err
            ))
        })
}
