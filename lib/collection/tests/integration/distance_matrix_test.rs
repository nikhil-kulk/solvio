use collection::operations::point_ops::{Batch, WriteOrdering};
use collection::operations::shard_selector_internal::ShardSelectorInternal;
use itertools::Itertools;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};
use segment::data_types::vectors::BatchVectorStructInternal;
use tempfile::Builder;

use crate::common::simple_collection_fixture;

const SEED: u64 = 42;

#[tokio::test(flavor = "multi_thread")]
async fn distance_matrix_empty() {
    let collection_dir = Builder::new().prefix("storage").tempdir().unwrap();

    // empty collection
    let collection = simple_collection_fixture(collection_dir.path(), 1).await;

    let sample_size = 100;
    let limit_per_sample = 10;
    let matrix = collection
        .distance_matrix(
            sample_size,
            limit_per_sample,
            None,
            "".to_string(), // default vector name
            ShardSelectorInternal::All,
            None,
            None,
        )
        .await
        .unwrap();

    // assert all empty
    assert!(matrix.sample_ids.is_empty());
    assert!(matrix.nearest.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn distance_matrix_anonymous_vector() {
    let collection_dir = Builder::new().prefix("storage").tempdir().unwrap();

    let collection = simple_collection_fixture(collection_dir.path(), 1).await;

    let point_count = 2000;
    let ids = (0..point_count).map_into().collect();
    let mut rng = SmallRng::seed_from_u64(SEED);

    let vectors = (0..point_count)
        .map(|_| rng.gen::<[f32; 4]>().to_vec())
        .collect_vec();

    let upsert_points = collection::operations::CollectionUpdateOperations::PointOperation(
        Batch {
            ids,
            vectors: BatchVectorStructInternal::from(vectors).into(),
            payloads: None,
        }
        .into(),
    );

    collection
        .update_from_client_simple(upsert_points, true, WriteOrdering::default())
        .await
        .unwrap();

    let sample_size = 100;
    let limit_per_sample = 10;
    let matrix = collection
        .distance_matrix(
            sample_size,
            limit_per_sample,
            None,
            "".to_string(), // default vector name
            ShardSelectorInternal::All,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(matrix.sample_ids.len(), sample_size);
    // no duplicate sample ids
    assert_eq!(
        matrix
            .sample_ids
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len(),
        sample_size
    );

    assert_eq!(matrix.nearest.len(), sample_size);
    for nearest in matrix.nearest {
        assert_eq!(nearest.len(), limit_per_sample);
        // assert each row sorted by scores
        nearest.iter().tuple_windows().for_each(|(prev, next)| {
            assert!(prev.score >= next.score);
        });
    }
}
