use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use atomic_refcell::AtomicRefCell;

use crate::common::Flusher;
use crate::data_types::vectors::VectorElementType;
use crate::entry::entry_point::{check_process_stopped, OperationResult};
use crate::spaces::metric::Metric;
use crate::spaces::simple::{CosineMetric, DotProductMetric, EuclidMetric};
use crate::spaces::tools::peek_top_largest_iterable;
use crate::types::{Distance, PointOffsetType, ScoreType};
use crate::vector_storage::mmap_vectors::MmapVectors;
use crate::vector_storage::{RawScorer, ScoredPointOffset, VectorStorage, VectorStorageSS};

fn vf_to_u8<T>(v: &[T]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * size_of::<T>()) }
}

/// An scored iterator over search result.
/// Keeps iteration context, which allows to use this iterator in external functions safely
pub struct MemmapRawScorer<'a, TMetric: Metric> {
    query: Vec<VectorElementType>,
    metric: std::marker::PhantomData<TMetric>,
    mmap_store: &'a MmapVectors,
}

impl<TMetric> RawScorer for MemmapRawScorer<'_, TMetric>
where
    TMetric: Metric,
{
    fn score_points(&self, points: &[PointOffsetType], scores: &mut [ScoredPointOffset]) -> usize {
        let mut size: usize = 0;
        // Use `read_deleted_map` instead of `deleted` to prevent multiple locks
        let deleted_map = self.mmap_store.read_deleted_map();
        for point in points {
            if MmapVectors::check_deleted(&deleted_map, *point).unwrap_or(true) {
                continue;
            }
            let other_vector = self.mmap_store.raw_vector(*point).unwrap();
            scores[size] = ScoredPointOffset {
                idx: *point,
                score: TMetric::similarity(&self.query, other_vector),
            };

            size += 1;
            if size == scores.len() {
                return size;
            }
        }
        size
    }

    fn check_point(&self, point: PointOffsetType) -> bool {
        (point < self.mmap_store.num_vectors as PointOffsetType)
            && !self.mmap_store.deleted(point).unwrap_or(true)
    }

    fn score_point(&self, point: PointOffsetType) -> ScoreType {
        let other_vector = self.mmap_store.raw_vector(point).unwrap();
        TMetric::similarity(&self.query, other_vector)
    }

    fn score_internal(&self, point_a: PointOffsetType, point_b: PointOffsetType) -> ScoreType {
        let vector_a = self.mmap_store.raw_vector(point_a).unwrap();
        let vector_b = self.mmap_store.raw_vector(point_b).unwrap();
        TMetric::similarity(vector_a, vector_b)
    }
}

/// Stores all vectors in mem-mapped file
///
/// It is not possible to insert new vectors into mem-mapped storage,
/// but possible to mark some vectors as removed
///
/// Mem-mapped storage can only be constructed from another storage
pub struct MemmapVectorStorage<TMetric: Metric> {
    vectors_path: PathBuf,
    deleted_path: PathBuf,
    mmap_store: Option<MmapVectors>,
    metric: PhantomData<TMetric>,
}

pub fn open_memmap_vector_storage(
    path: &Path,
    dim: usize,
    distance: Distance,
) -> OperationResult<Arc<AtomicRefCell<VectorStorageSS>>> {
    create_dir_all(path)?;

    let vectors_path = path.join("matrix.dat");
    let deleted_path = path.join("deleted.dat");

    let mmap_store = MmapVectors::open(&vectors_path, &deleted_path, dim)?;

    match distance {
        Distance::Cosine => Ok(Arc::new(AtomicRefCell::new(MemmapVectorStorage::<
            CosineMetric,
        > {
            vectors_path,
            deleted_path,
            mmap_store: Some(mmap_store),
            metric: PhantomData,
        }))),
        Distance::Euclid => Ok(Arc::new(AtomicRefCell::new(MemmapVectorStorage::<
            EuclidMetric,
        > {
            vectors_path,
            deleted_path,
            mmap_store: Some(mmap_store),
            metric: PhantomData,
        }))),
        Distance::Dot => Ok(Arc::new(AtomicRefCell::new(MemmapVectorStorage::<
            DotProductMetric,
        > {
            vectors_path,
            deleted_path,
            mmap_store: Some(mmap_store),
            metric: PhantomData,
        }))),
    }
}

impl<TMetric> VectorStorage for MemmapVectorStorage<TMetric>
where
    TMetric: Metric,
{
    fn vector_dim(&self) -> usize {
        self.mmap_store.as_ref().unwrap().dim
    }

    fn vector_count(&self) -> usize {
        self.mmap_store
            .as_ref()
            .map(|store| store.num_vectors - store.deleted_count)
            .unwrap()
    }

    fn deleted_count(&self) -> usize {
        self.mmap_store.as_ref().unwrap().deleted_count
    }

    fn total_vector_count(&self) -> usize {
        self.mmap_store.as_ref().unwrap().num_vectors
    }

    fn get_vector(&self, key: PointOffsetType) -> Option<Vec<VectorElementType>> {
        self.mmap_store.as_ref().and_then(|x| x.get_vector(key))
    }

    fn put_vector(&mut self, _vector: Vec<VectorElementType>) -> OperationResult<PointOffsetType> {
        panic!("Can't put vector in mmap storage")
    }

    fn insert_vector(
        &mut self,
        _key: PointOffsetType,
        _vector: Vec<VectorElementType>,
    ) -> OperationResult<()> {
        panic!("Can't directly update vector in mmap storage")
    }

    fn next_id(&self) -> PointOffsetType {
        panic!("There are no available for insertion ids in mmap storage")
    }

    fn update_from(
        &mut self,
        other: &VectorStorageSS,
        stopped: &AtomicBool,
    ) -> OperationResult<Range<PointOffsetType>> {
        let dim = self.vector_dim();

        let start_index = self.mmap_store.as_ref().unwrap().num_vectors as PointOffsetType;
        let mut end_index = start_index;

        self.mmap_store = None;

        {
            let mut file = OpenOptions::new()
                .read(false)
                .write(false)
                .append(true)
                .create(false)
                .open(&self.vectors_path)?;

            for id in other.iter_ids() {
                check_process_stopped(stopped)?;
                let vector = &other.get_vector(id).unwrap();
                let raw_bites = vf_to_u8(vector);
                file.write_all(raw_bites)?;
                end_index += 1;
            }

            file.flush()?;
        }
        {
            let mut file = OpenOptions::new()
                .read(false)
                .write(false)
                .append(true)
                .create(false)
                .open(&self.deleted_path)?;

            let flags: Vec<u8> = vec![0; (end_index - start_index) as usize];
            let flag_bytes = vf_to_u8(&flags);
            file.write_all(flag_bytes)?;
            file.flush()?;
        }

        self.mmap_store = Some(MmapVectors::open(
            &self.vectors_path,
            &self.deleted_path,
            dim,
        )?);

        Ok(start_index..end_index)
    }

    fn delete(&mut self, key: PointOffsetType) -> OperationResult<()> {
        self.mmap_store.as_mut().unwrap().delete(key)
    }

    fn is_deleted(&self, key: PointOffsetType) -> bool {
        self.mmap_store
            .as_ref()
            .unwrap()
            .deleted(key)
            .unwrap_or(false)
    }

    fn iter_ids(&self) -> Box<dyn Iterator<Item = PointOffsetType> + '_> {
        let num_vectors = self.mmap_store.as_ref().unwrap().num_vectors;
        let iter = (0..(num_vectors as PointOffsetType))
            .filter(move |id| !self.mmap_store.as_ref().unwrap().deleted(*id).unwrap());
        Box::new(iter)
    }

    fn flusher(&self) -> Flusher {
        match &self.mmap_store {
            None => Box::new(|| Ok(())),
            Some(x) => x.flusher(),
        }
    }

    fn raw_scorer(&self, vector: Vec<VectorElementType>) -> Box<dyn RawScorer + '_> {
        Box::new(MemmapRawScorer::<TMetric> {
            query: TMetric::preprocess(&vector).unwrap_or(vector),
            metric: PhantomData,
            mmap_store: self.mmap_store.as_ref().unwrap(),
        })
    }

    fn raw_scorer_internal(&self, point_id: PointOffsetType) -> Box<dyn RawScorer + '_> {
        Box::new(MemmapRawScorer::<TMetric> {
            query: self.get_vector(point_id).unwrap(),
            metric: PhantomData,
            mmap_store: self.mmap_store.as_ref().unwrap(),
        })
    }

    fn score_points(
        &self,
        vector: &[VectorElementType],
        points: &mut dyn Iterator<Item = PointOffsetType>,
        top: usize,
    ) -> Vec<ScoredPointOffset> {
        let preprocessed_vector_opt = TMetric::preprocess(vector);
        let preprocessed_vector = preprocessed_vector_opt
            .as_ref()
            .map_or(vector, |x| x as &[_]);
        let scores = points
            .filter(|point| {
                !self
                    .mmap_store
                    .as_ref()
                    .unwrap()
                    .deleted(*point)
                    .unwrap_or(true)
            })
            .map(|point| {
                let other_vector = self.mmap_store.as_ref().unwrap().raw_vector(point).unwrap();
                ScoredPointOffset {
                    idx: point,
                    score: TMetric::similarity(preprocessed_vector, other_vector),
                }
            });
        peek_top_largest_iterable(scores, top)
    }

    fn score_all(&self, vector: &[VectorElementType], top: usize) -> Vec<ScoredPointOffset> {
        let preprocessed_vector_opt = TMetric::preprocess(vector);
        let preprocessed_vector = preprocessed_vector_opt
            .as_ref()
            .map_or(vector, |x| x as &[_]);
        let scores = self.iter_ids().map(|point| {
            let other_vector = self.mmap_store.as_ref().unwrap().raw_vector(point).unwrap();
            ScoredPointOffset {
                idx: point,
                score: TMetric::similarity(preprocessed_vector, other_vector),
            }
        });

        peek_top_largest_iterable(scores, top)
    }

    fn score_internal(
        &self,
        point: PointOffsetType,
        points: &mut dyn Iterator<Item = PointOffsetType>,
        top: usize,
    ) -> Vec<ScoredPointOffset> {
        let vector = self.get_vector(point).unwrap();
        self.score_points(&vector, points, top)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::transmute;

    use tempfile::Builder;

    use super::*;
    use crate::common::rocksdb_wrapper::{open_db, DB_VECTOR_CF};
    use crate::vector_storage::simple_vector_storage::open_simple_vector_storage;

    #[test]
    fn test_basic_persistence() {
        let dist = Distance::Dot;
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();

        let storage = open_memmap_vector_storage(dir.path(), 4, dist).unwrap();
        let mut borrowed_storage = storage.borrow_mut();

        let vec1 = vec![1.0, 0.0, 1.0, 1.0];
        let vec2 = vec![1.0, 0.0, 1.0, 0.0];
        let vec3 = vec![1.0, 1.0, 1.0, 1.0];
        let vec4 = vec![1.0, 1.0, 0.0, 1.0];
        let vec5 = vec![1.0, 0.0, 0.0, 0.0];

        {
            let dir2 = Builder::new().prefix("db_dir").tempdir().unwrap();
            let db = open_db(dir2.path(), &[DB_VECTOR_CF]).unwrap();
            let storage2 = open_simple_vector_storage(db, DB_VECTOR_CF, 4, dist).unwrap();
            {
                let mut borrowed_storage2 = storage2.borrow_mut();
                borrowed_storage2.put_vector(vec1).unwrap();
                borrowed_storage2.put_vector(vec2.clone()).unwrap();
                borrowed_storage2.put_vector(vec3.clone()).unwrap();
            }
            borrowed_storage
                .update_from(&*storage2.borrow(), &Default::default())
                .unwrap();
        }

        assert_eq!(borrowed_storage.vector_count(), 3);

        let vector = borrowed_storage.get_vector(1).unwrap();

        assert_eq!(vec2, vector);

        assert_eq!(borrowed_storage.deleted_count(), 0);

        borrowed_storage.delete(2).unwrap();

        {
            let dir2 = Builder::new().prefix("db_dir").tempdir().unwrap();
            let db = open_db(dir2.path(), &[DB_VECTOR_CF]).unwrap();
            let storage2 = open_simple_vector_storage(db, DB_VECTOR_CF, 4, dist).unwrap();
            {
                let mut borrowed_storage2 = storage2.borrow_mut();
                borrowed_storage2.put_vector(vec4).unwrap();
                borrowed_storage2.put_vector(vec5).unwrap();
            }
            borrowed_storage
                .update_from(&*storage2.borrow(), &Default::default())
                .unwrap();
        }

        assert_eq!(borrowed_storage.vector_count(), 4);

        let stored_ids: Vec<PointOffsetType> = borrowed_storage.iter_ids().collect();

        assert_eq!(stored_ids, [0, 1, 3, 4]);

        let res = borrowed_storage.score_all(&vec3, 2);

        assert_eq!(res.len(), 2);

        assert_ne!(res[0].idx, 2);

        let res = borrowed_storage.score_points(&vec3, &mut vec![0, 1, 2, 3, 4].iter().cloned(), 2);

        assert_eq!(res.len(), 2);
        assert_ne!(res[0].idx, 2);
    }

    #[test]
    fn test_mmap_raw_scorer() {
        let dist = Distance::Dot;
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let storage = open_memmap_vector_storage(dir.path(), 4, dist).unwrap();
        let mut borrowed_storage = storage.borrow_mut();

        let vec1 = vec![1.0, 0.0, 1.0, 1.0];
        let vec2 = vec![1.0, 0.0, 1.0, 0.0];
        let vec3 = vec![1.0, 1.0, 1.0, 1.0];
        let vec4 = vec![1.0, 1.0, 0.0, 1.0];
        let vec5 = vec![1.0, 0.0, 0.0, 0.0];

        {
            let dir2 = Builder::new().prefix("db_dir").tempdir().unwrap();
            let db = open_db(dir2.path(), &[DB_VECTOR_CF]).unwrap();
            let storage2 = open_simple_vector_storage(db, DB_VECTOR_CF, 4, dist).unwrap();
            {
                let mut borrowed_storage2 = storage2.borrow_mut();
                borrowed_storage2.put_vector(vec1).unwrap();
                borrowed_storage2.put_vector(vec2).unwrap();
                borrowed_storage2.put_vector(vec3).unwrap();
                borrowed_storage2.put_vector(vec4).unwrap();
                borrowed_storage2.put_vector(vec5).unwrap();
            }
            borrowed_storage
                .update_from(&*storage2.borrow(), &Default::default())
                .unwrap();
        }

        let query = vec![-1.0, -1.0, -1.0, -1.0];
        let query_points: Vec<PointOffsetType> = vec![0, 2, 4];

        let scorer = borrowed_storage.raw_scorer(query);

        let mut res = vec![ScoredPointOffset { idx: 0, score: 0. }; query_points.len()];
        let res_count = scorer.score_points(&query_points, &mut res);
        res.resize(res_count, ScoredPointOffset { idx: 0, score: 0. });

        assert_eq!(res.len(), 3);
        assert_eq!(res[0].idx, 0);
        assert_eq!(res[1].idx, 2);
        assert_eq!(res[2].idx, 4);

        assert_eq!(res[2].score, -1.0);
    }

    #[test]
    fn test_casts() {
        let data: Vec<VectorElementType> = vec![0.42, 0.069, 333.1, 100500.];

        let raw_data = vf_to_u8(&data);

        eprintln!("raw_data.len() = {:#?}", raw_data.len());

        let arr: &[VectorElementType] = unsafe { transmute(raw_data) };

        let slice = &arr[0..data.len()];

        eprintln!("slice.len() = {:#?}", slice.len());

        for (idx, element) in slice.iter().enumerate() {
            println!("slice[{}]  = {:?}", idx, element);
        }
    }
}
