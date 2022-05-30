use collection::config::{CollectionConfig, CollectionParams, WalConfig};
use collection::operations::types::CollectionError;
use collection::optimizers_builder::OptimizersConfig;
use collection::{ChannelService, Collection, CollectionId, CollectionShardDistribution};
use segment::types::Distance;
use std::num::NonZeroU32;
use std::path::Path;

/// Test collections for this upper bound of shards.
/// Testing with more shards is problematic due to `number of open files problem`
/// See https://github.com/solvio/solvio/issues/379
#[allow(dead_code)]
pub const N_SHARDS: u32 = 3;

pub const TEST_OPTIMIZERS_CONFIG: OptimizersConfig = OptimizersConfig {
    deleted_threshold: 0.9,
    vacuum_min_vector_number: 1000,
    default_segment_number: 2,
    max_segment_size: 100_000,
    memmap_threshold: 100_000,
    indexing_threshold: 50_000,
    payload_indexing_threshold: 20_000,
    flush_interval_sec: 30,
    max_optimization_threads: 2,
};

#[allow(dead_code)]
pub async fn simple_collection_fixture(collection_path: &Path, shard_number: u32) -> Collection {
    let wal_config = WalConfig {
        wal_capacity_mb: 1,
        wal_segments_ahead: 0,
    };

    let collection_params = CollectionParams {
        vector_size: 4,
        distance: Distance::Dot,
        shard_number: NonZeroU32::new(shard_number).expect("Shard number can not be zero"),
    };

    let collection_config = CollectionConfig {
        params: collection_params,
        optimizer_config: TEST_OPTIMIZERS_CONFIG.clone(),
        wal_config,
        hnsw_config: Default::default(),
    };

    // Default to a collection with all the shards local
    new_local_collection("test".to_string(), collection_path, &collection_config)
        .await
        .unwrap()
}

/// Default to a collection with all the shards local
pub async fn new_local_collection(
    id: CollectionId,
    path: &Path,
    config: &CollectionConfig,
) -> Result<Collection, CollectionError> {
    Collection::new(
        id,
        path,
        config,
        CollectionShardDistribution::AllLocal,
        ChannelService::default(),
    )
    .await
}

/// Default to a collection with all the shards local
pub async fn load_local_collection(id: CollectionId, path: &Path) -> Collection {
    Collection::load(id, path, ChannelService::default()).await
}
