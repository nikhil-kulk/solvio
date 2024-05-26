use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;
use common::types::PointOffsetType;
use rstest::rstest;
use tempfile::Builder;

use crate::common::rocksdb_wrapper::{open_db, DB_VECTOR_CF};
use crate::data_types::vectors::{MultiDenseVector, QueryVector, TypedMultiDenseVectorRef};
use crate::fixtures::payload_context_fixture::FixtureIdTracker;
use crate::id_tracker::IdTrackerSS;
use crate::types::{Distance, MultiVectorConfig};
use crate::vector_storage::multi_dense::appendable_mmap_multi_dense_vector_storage::open_appendable_memmap_multi_vector_storage;
use crate::vector_storage::multi_dense::simple_multi_dense_vector_storage::open_simple_multi_dense_vector_storage;
use crate::vector_storage::{new_raw_scorer, MultiVectorStorage, VectorStorage, VectorStorageEnum};

#[derive(Clone, Copy)]
enum MultiDenseStorageType {
    SimpleRamFloat,
    AppendableMmapFloat,
}

fn multi_points_fixtures() -> Vec<MultiDenseVector> {
    let mut multis: Vec<MultiDenseVector> = Vec::new();
    for i in 0..5 {
        let value = i as f32;
        let vectors = vec![
            vec![value, 0.0, value, value],
            vec![value, 0.0, value, 0.0],
            vec![value, value, value, value],
            vec![value, value, 0.0, value],
            vec![value, 0.0, 0.0, value],
        ];
        let multi = MultiDenseVector::try_from(vectors).unwrap();
        multis.push(multi);
    }
    multis
}

fn do_test_delete_points(storage: Arc<AtomicRefCell<VectorStorageEnum>>) {
    let points = multi_points_fixtures();

    let delete_mask = [false, false, true, true, false];

    let id_tracker: Arc<AtomicRefCell<IdTrackerSS>> =
        Arc::new(AtomicRefCell::new(FixtureIdTracker::new(points.len())));

    let borrowed_id_tracker = id_tracker.borrow_mut();
    let mut borrowed_storage = storage.borrow_mut();

    // Insert all points
    for (i, vec) in points.iter().enumerate() {
        borrowed_storage
            .insert_vector(i as PointOffsetType, vec.into())
            .unwrap();
    }
    // Check that all points are inserted
    for (i, vec) in points.iter().enumerate() {
        let stored_vec = borrowed_storage.get_vector(i as PointOffsetType);
        let multi_dense: TypedMultiDenseVectorRef<_> = stored_vec.as_vec_ref().try_into().unwrap();
        assert_eq!(multi_dense.to_owned(), vec.clone());
    }
    // Check that all points are inserted #2
    {
        let orig_iter = points.iter().flat_map(|multivec| multivec.multi_vectors());
        match &borrowed_storage as &VectorStorageEnum {
            VectorStorageEnum::DenseSimple(_) => unreachable!(),
            VectorStorageEnum::DenseSimpleByte(_) => unreachable!(),
            VectorStorageEnum::DenseSimpleHalf(_) => unreachable!(),
            VectorStorageEnum::DenseMemmap(_) => unreachable!(),
            VectorStorageEnum::DenseMemmapByte(_) => unreachable!(),
            VectorStorageEnum::DenseMemmapHalf(_) => unreachable!(),
            VectorStorageEnum::DenseAppendableMemmap(_) => unreachable!(),
            VectorStorageEnum::DenseAppendableMemmapByte(_) => unreachable!(),
            VectorStorageEnum::DenseAppendableMemmapHalf(_) => unreachable!(),
            VectorStorageEnum::SparseSimple(_) => unreachable!(),
            VectorStorageEnum::MultiDenseSimple(v) => {
                for (orig, vec) in orig_iter.zip(v.iterate_inner_vectors()) {
                    assert_eq!(orig, vec);
                }
            }
            VectorStorageEnum::MultiDenseSimpleByte(_) => unreachable!(),
            VectorStorageEnum::MultiDenseSimpleHalf(_) => unreachable!(),
            VectorStorageEnum::MultiDenseAppendableMemmap(v) => {
                for (orig, vec) in orig_iter.zip(v.iterate_inner_vectors()) {
                    assert_eq!(orig, vec);
                }
            }
            VectorStorageEnum::MultiDenseAppendableMemmapByte(_) => unreachable!(),
            VectorStorageEnum::MultiDenseAppendableMemmapHalf(_) => unreachable!(),
        };
    }

    // Delete select number of points
    delete_mask
        .into_iter()
        .enumerate()
        .filter(|(_, d)| *d)
        .for_each(|(i, _)| {
            borrowed_storage
                .delete_vector(i as PointOffsetType)
                .unwrap();
        });
    assert_eq!(
        borrowed_storage.deleted_vector_count(),
        2,
        "2 vectors must be deleted"
    );
    let vector: Vec<Vec<f32>> = vec![vec![0.0, 1.0, 1.1, 1.0]];
    let query = QueryVector::Nearest(vector.try_into().unwrap());
    let closest = new_raw_scorer(
        query,
        &borrowed_storage,
        borrowed_id_tracker.deleted_point_bitslice(),
    )
    .unwrap()
    .peek_top_iter(&mut [0, 1, 2, 3, 4].iter().cloned(), 5);
    assert_eq!(closest.len(), 3, "must have 3 vectors, 2 are deleted");
    assert_eq!(closest[0].idx, 4);
    assert_eq!(closest[1].idx, 1);
    assert_eq!(closest[2].idx, 0);

    // Delete 1, redelete 2
    borrowed_storage
        .delete_vector(1 as PointOffsetType)
        .unwrap();
    borrowed_storage
        .delete_vector(2 as PointOffsetType)
        .unwrap();
    assert_eq!(
        borrowed_storage.deleted_vector_count(),
        3,
        "3 vectors must be deleted"
    );

    let vector: Vec<Vec<f32>> = vec![vec![1.0, 0.0, 0.0, 0.0]];
    let query = QueryVector::Nearest(vector.try_into().unwrap());
    let closest = new_raw_scorer(
        query,
        &borrowed_storage,
        borrowed_id_tracker.deleted_point_bitslice(),
    )
    .unwrap()
    .peek_top_iter(&mut [0, 1, 2, 3, 4].iter().cloned(), 5);
    assert_eq!(closest.len(), 2, "must have 2 vectors, 3 are deleted");
    assert_eq!(closest[0].idx, 4);
    assert_eq!(closest[1].idx, 0);

    // Delete all
    borrowed_storage
        .delete_vector(0 as PointOffsetType)
        .unwrap();
    borrowed_storage
        .delete_vector(4 as PointOffsetType)
        .unwrap();
    assert_eq!(
        borrowed_storage.deleted_vector_count(),
        5,
        "all vectors must be deleted"
    );

    let vector: Vec<Vec<f32>> = vec![vec![1.0, 0.0, 0.0, 0.0]];
    let query = QueryVector::Nearest(vector.try_into().unwrap());
    let closest = new_raw_scorer(
        query,
        &borrowed_storage,
        borrowed_id_tracker.deleted_point_bitslice(),
    )
    .unwrap()
    .peek_top_all(5);
    assert!(closest.is_empty(), "must have no results, all deleted");
}

fn do_test_update_from_delete_points(storage: Arc<AtomicRefCell<VectorStorageEnum>>) {
    let points = multi_points_fixtures();

    let delete_mask = [false, false, true, true, false];

    let id_tracker: Arc<AtomicRefCell<IdTrackerSS>> =
        Arc::new(AtomicRefCell::new(FixtureIdTracker::new(points.len())));
    let borrowed_id_tracker = id_tracker.borrow_mut();
    let mut borrowed_storage = storage.borrow_mut();

    {
        let dir2 = Builder::new().prefix("db_dir").tempdir().unwrap();
        let db = open_db(dir2.path(), &[DB_VECTOR_CF]).unwrap();
        let storage2 = open_simple_multi_dense_vector_storage(
            db,
            DB_VECTOR_CF,
            4,
            Distance::Dot,
            MultiVectorConfig::default(),
            &AtomicBool::new(false),
        )
        .unwrap();
        {
            let mut borrowed_storage2 = storage2.borrow_mut();
            points.iter().enumerate().for_each(|(i, vec)| {
                borrowed_storage2
                    .insert_vector(i as PointOffsetType, vec.into())
                    .unwrap();
                if delete_mask[i] {
                    borrowed_storage2
                        .delete_vector(i as PointOffsetType)
                        .unwrap();
                }
            });
        }
        borrowed_storage
            .update_from(
                &storage2.borrow(),
                &mut Box::new(0..points.len() as u32),
                &Default::default(),
            )
            .unwrap();
    }

    assert_eq!(
        borrowed_storage.deleted_vector_count(),
        2,
        "2 vectors must be deleted from other storage"
    );

    let vector: Vec<Vec<f32>> = vec![vec![0.0, 1.0, 1.1, 1.0]];

    let query = QueryVector::Nearest(vector.try_into().unwrap());

    let closest = new_raw_scorer(
        query,
        &borrowed_storage,
        borrowed_id_tracker.deleted_point_bitslice(),
    )
    .unwrap()
    .peek_top_iter(&mut [0, 1, 2, 3, 4].iter().cloned(), 5);
    assert_eq!(closest.len(), 3, "must have 3 vectors, 2 are deleted");
    assert_eq!(closest[0].idx, 4);
    assert_eq!(closest[1].idx, 1);
    assert_eq!(closest[2].idx, 0);

    // Delete all
    borrowed_storage
        .delete_vector(0 as PointOffsetType)
        .unwrap();
    borrowed_storage
        .delete_vector(1 as PointOffsetType)
        .unwrap();
    borrowed_storage
        .delete_vector(4 as PointOffsetType)
        .unwrap();
    assert_eq!(
        borrowed_storage.deleted_vector_count(),
        5,
        "all vectors must be deleted"
    );
}

fn create_vector_storage(
    storage_type: MultiDenseStorageType,
    path: &Path,
) -> Arc<AtomicRefCell<VectorStorageEnum>> {
    match storage_type {
        MultiDenseStorageType::SimpleRamFloat => {
            let db = open_db(path, &[DB_VECTOR_CF]).unwrap();
            open_simple_multi_dense_vector_storage(
                db,
                DB_VECTOR_CF,
                4,
                Distance::Dot,
                MultiVectorConfig::default(),
                &AtomicBool::new(false),
            )
            .unwrap()
        }
        MultiDenseStorageType::AppendableMmapFloat => open_appendable_memmap_multi_vector_storage(
            path,
            4,
            Distance::Dot,
            MultiVectorConfig::default(),
        )
        .unwrap(),
    }
}

#[rstest]
fn test_delete_points_in_multi_dense_vector_storage(
    #[values(
        MultiDenseStorageType::SimpleRamFloat,
        MultiDenseStorageType::AppendableMmapFloat
    )]
    storage_type: MultiDenseStorageType,
) {
    let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();

    let total_vector_count = {
        let storage = create_vector_storage(storage_type, dir.path());
        do_test_delete_points(storage.clone());
        let count = storage.borrow().total_vector_count();
        storage.borrow().flusher()().unwrap();
        count
    };
    let storage = create_vector_storage(storage_type, dir.path());
    assert_eq!(
        storage.borrow().total_vector_count(),
        total_vector_count,
        "total vector count must be the same"
    );
}

#[rstest]
fn test_update_from_delete_points_multi_dense_vector_storage(
    #[values(
        MultiDenseStorageType::SimpleRamFloat,
        MultiDenseStorageType::AppendableMmapFloat
    )]
    storage_type: MultiDenseStorageType,
) {
    let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
    let total_vector_count = {
        let storage = create_vector_storage(storage_type, dir.path());
        do_test_update_from_delete_points(storage.clone());
        let count = storage.borrow().total_vector_count();
        storage.borrow().flusher()().unwrap();
        count
    };
    let storage = create_vector_storage(storage_type, dir.path());
    assert_eq!(
        storage.borrow().total_vector_count(),
        total_vector_count,
        "total vector count must be the same"
    );
}
