// TODO: Remove when we release the next version and integrate the immutable id tracker
#![allow(dead_code)]
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::iter;
use std::mem::size_of_val;
use std::path::{Path, PathBuf};

use bitvec::prelude::BitSlice;
use bitvec::vec::BitVec;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use common::types::PointOffsetType;
use itertools::Itertools;
use memory::mmap_ops::{create_and_ensure_length, open_write_mmap};
use rand::distributions::Distribution;
use uuid::Uuid;

use crate::common::mmap_bitslice_buffered_update_wrapper::MmapBitSliceBufferedUpdateWrapper;
use crate::common::mmap_slice_buffered_update_wrapper::MmapSliceBufferedUpdateWrapper;
use crate::common::mmap_type::{MmapBitSlice, MmapSlice};
use crate::common::operation_error::{OperationError, OperationResult};
use crate::common::Flusher;
use crate::id_tracker::IdTracker;
use crate::types::{ExtendedPointId, PointIdType, SeqNumberType};

pub const DELETED_FILE_NAME: &str = "id_tracker.deleted";
pub const MAPPINGS_FILE_NAME: &str = "id_tracker.mappings";
pub const VERSION_MAPPING_FILE_NAME: &str = "id_tracker.versions";

#[derive(Debug)]
pub struct ImmutableIdTracker {
    path: PathBuf,

    deleted: BitVec,
    deleted_wrapper: MmapBitSliceBufferedUpdateWrapper,

    internal_to_version: Vec<SeqNumberType>,
    internal_to_version_wrapper: MmapSliceBufferedUpdateWrapper<SeqNumberType>,

    mappings: PointMappings,
}

#[derive(Clone, PartialEq, Debug)]
pub struct PointMappings {
    pub(crate) internal_to_external: Vec<PointIdType>,

    // Having two separate maps allows us iterating only over one type at a time without having to filter.
    pub(crate) external_to_internal_num: BTreeMap<u64, PointOffsetType>,
    pub(crate) external_to_internal_uuid: BTreeMap<Uuid, PointOffsetType>,
}

/// Used endianness for storing PointMapping-files.
type FileEndianess = LittleEndian;

impl PointMappings {
    const EXTERNAL_ID_NUMBER_BYTE: u8 = 0;
    const EXTERNAL_ID_UUID_BYTE: u8 = 1;

    /// Loads a `PointMappings` from the given reader. Applies an optional filter of deleted items
    /// to prevent allocating unneeded data.
    pub fn load<R: Read>(mut reader: R, filter: Option<&BitSlice>) -> OperationResult<Self> {
        // Deserialize the header
        let len = reader.read_u64::<FileEndianess>()? as usize;

        let mut internal_to_external = Vec::with_capacity(len);
        let mut external_to_internal_num: BTreeMap<u64, PointOffsetType> = BTreeMap::new();
        let mut external_to_internal_uuid: BTreeMap<Uuid, PointOffsetType> = BTreeMap::new();

        // Deserialize the list entries
        for i in 0..len {
            let (internal_id, external_id) = Self::read_entry(&mut reader)?;

            // Need to push this regardless of point deletion as the vecs index represents the internal id
            // which would become wrong if we leave out entries.
            internal_to_external.push(external_id);

            let deleted = filter
                .as_ref()
                .and_then(|deleted| deleted.get(i).as_deref().copied())
                .unwrap_or_default();

            if deleted {
                continue;
            }

            match external_id {
                ExtendedPointId::NumId(num) => {
                    external_to_internal_num.insert(num, internal_id);
                }
                ExtendedPointId::Uuid(uuid) => {
                    external_to_internal_uuid.insert(uuid, internal_id);
                }
            }
        }

        // Check that the file has ben fully read.
        #[cfg(debug_assertions)] // Only for dev builds
        {
            let mut buf = vec![];
            let read_bytes = reader.read_to_end(&mut buf).unwrap();
            assert_eq!(buf.len(), 0);
            assert_eq!(read_bytes, 0);
        }

        Ok(PointMappings {
            internal_to_external,
            external_to_internal_num,
            external_to_internal_uuid,
        })
    }

    /// Loads a single entry from a reader. Expects the reader to be aligned so, that the next read
    /// byte is the first byte of a new entry.
    /// This function reads exact one entry which means after calling this function, the reader
    /// will be at the start of the next entry.
    fn read_entry<R: Read>(mut reader: R) -> OperationResult<(PointOffsetType, ExtendedPointId)> {
        let point_id_type = reader.read_u8()?;

        let external_id = if point_id_type == Self::EXTERNAL_ID_NUMBER_BYTE {
            let num = reader.read_u64::<FileEndianess>()?;
            PointIdType::NumId(num)
        } else if point_id_type == Self::EXTERNAL_ID_UUID_BYTE {
            let uuid_u128 = reader.read_u128::<FileEndianess>()?;
            PointIdType::Uuid(Uuid::from_u128_le(uuid_u128))
        } else {
            return Err(OperationError::InconsistentStorage {
                description: "Invalid byte read when deserializing Immutable id tracker"
                    .to_string(),
            });
        };

        let internal_id = reader.read_u32::<FileEndianess>()? as PointOffsetType;
        Ok((internal_id, external_id))
    }

    /// Serializes the `PointMappings` into the given writer using the file format specified below.
    ///
    /// ## File format
    /// In general the format looks like this:
    /// +---------------------------+-----------------+
    /// | Header (list length: u64) | List of entries |
    /// +---------------------------+-----------------+
    ///
    /// A single list entry:
    /// +-----------------+-----------------------+------------------+
    /// | PointIdType: u8 | Number/UUID: u64/u128 | Internal ID: u32 |
    /// +-----------------+-----------------------+------------------+
    /// A single entry is thus either 1+8+4=13 or 1+16+4=21 bytes in size depending
    /// on the PointIdType.

    pub fn store<W: Write>(&self, mut writer: W) -> OperationResult<()> {
        // Serialize the header (=length).
        writer.write_u64::<FileEndianess>(self.internal_to_external.len() as u64)?;

        // Serialize all entries
        for external_id in self.internal_to_external.iter() {
            self.write_entry(&mut writer, external_id)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn write_entry<W: Write>(
        &self,
        mut writer: W,
        external_id: &PointIdType,
    ) -> OperationResult<()> {
        // Serializing External ID
        match external_id {
            PointIdType::NumId(num) => {
                // Byte to distinguish between Number and UUID
                writer.write_u8(Self::EXTERNAL_ID_NUMBER_BYTE)?;

                // The PointID's number
                writer.write_u64::<FileEndianess>(*num)?;
            }
            PointIdType::Uuid(uuid) => {
                // Byte to distinguish between Number and UUID
                writer.write_u8(Self::EXTERNAL_ID_UUID_BYTE)?;

                // The PointID's UUID
                writer.write_u128::<FileEndianess>(uuid.to_u128_le())?;
            }
        }

        let internal_id = match external_id {
            PointIdType::NumId(n) => self.external_to_internal_num.get(n),
            PointIdType::Uuid(u) => self.external_to_internal_uuid.get(u),
        }
        .ok_or(OperationError::PointIdError {
            missed_point_id: *external_id,
        })?;

        // Serializing Internal ID
        writer.write_u32::<FileEndianess>(*internal_id)?;

        Ok(())
    }
}

impl ImmutableIdTracker {
    pub fn open(segment_path: &Path) -> OperationResult<Self> {
        let deleted_raw = open_write_mmap(&Self::deleted_file_path(segment_path))?;
        let deleted_mmap = MmapBitSlice::try_from(deleted_raw, 0)?;
        let deleted_bitvec = deleted_mmap.to_bitvec();
        let deleted_wrapper = MmapBitSliceBufferedUpdateWrapper::new(deleted_mmap);

        let internal_to_version_map =
            open_write_mmap(&Self::version_mapping_file_path(segment_path))?;
        let internal_to_version_mapslice: MmapSlice<SeqNumberType> =
            unsafe { MmapSlice::try_from(internal_to_version_map)? };
        let internal_to_version = internal_to_version_mapslice.to_vec();
        let internal_to_version_wrapper =
            MmapSliceBufferedUpdateWrapper::new(internal_to_version_mapslice);

        let reader = BufReader::new(File::open(Self::mappings_file_path(segment_path))?);
        let mappings = PointMappings::load(reader, Some(&deleted_bitvec))?;

        Ok(Self {
            path: segment_path.to_path_buf(),
            deleted: deleted_bitvec,
            deleted_wrapper,
            internal_to_version_wrapper,
            internal_to_version,
            mappings,
        })
    }

    pub(super) fn new(
        path: &Path,
        deleted: &BitSlice,
        internal_to_version: &[SeqNumberType],
        mappings: PointMappings,
    ) -> OperationResult<Self> {
        // Create mmap file for deleted bitvec
        let deleted_filepath = Self::deleted_file_path(path);
        {
            let deleted_size = bitmap_mmap_size(deleted);
            create_and_ensure_length(&deleted_filepath, deleted_size)?;
        }

        let deleted_mmap = open_write_mmap(&deleted_filepath)?;
        let mut deleted_new = MmapBitSlice::try_from(deleted_mmap, 0)?;
        deleted_new[..deleted.len()].copy_from_bitslice(deleted);
        let deleted_wrapper = MmapBitSliceBufferedUpdateWrapper::new(deleted_new);

        // Create mmap file for internal-to-version list
        let version_filepath = Self::version_mapping_file_path(path);
        {
            let version_size = size_of_val(internal_to_version);
            create_and_ensure_length(&version_filepath, version_size)?;
        }
        let mut internal_to_version_wrapper =
            unsafe { MmapSlice::try_from(open_write_mmap(&version_filepath)?)? };
        internal_to_version_wrapper.copy_from_slice(internal_to_version);
        let internal_to_version = internal_to_version_wrapper.to_vec();
        let internal_to_version_wrapper =
            MmapSliceBufferedUpdateWrapper::new(internal_to_version_wrapper);

        // Write mappings to disk.
        let writer = BufWriter::new(File::create(Self::mappings_file_path(path))?);
        mappings.store(writer)?;

        Ok(Self {
            path: path.to_path_buf(),
            deleted: deleted.to_bitvec(),
            deleted_wrapper,
            internal_to_version_wrapper,
            internal_to_version,
            mappings,
        })
    }

    fn deleted_file_path(base: &Path) -> PathBuf {
        base.join(DELETED_FILE_NAME)
    }

    fn version_mapping_file_path(base: &Path) -> PathBuf {
        base.join(VERSION_MAPPING_FILE_NAME)
    }

    pub(crate) fn mappings_file_path(base: &Path) -> PathBuf {
        base.join(MAPPINGS_FILE_NAME)
    }
}

/// Returns the required mmap filesize for a `BitSlice`.
fn bitmap_mmap_size(deleted: &BitSlice) -> usize {
    let usize_bytes = std::mem::size_of::<usize>();
    let num_bytes = deleted.len().div_ceil(8); // used bytes
    num_bytes.div_ceil(usize_bytes) * usize_bytes // Make it a multiple of usize-width.
}

impl IdTracker for ImmutableIdTracker {
    fn internal_version(&self, internal_id: PointOffsetType) -> Option<SeqNumberType> {
        self.internal_to_version.get(internal_id as usize).copied()
    }

    fn set_internal_version(
        &mut self,
        internal_id: PointOffsetType,
        version: SeqNumberType,
    ) -> OperationResult<()> {
        if self.external_id(internal_id).is_some() {
            if let Some(old_version) = self.internal_to_version.get_mut(internal_id as usize) {
                *old_version = version;
                self.internal_to_version_wrapper
                    .set(internal_id as usize, version);
            }
        }

        Ok(())
    }

    fn internal_id(&self, external_id: PointIdType) -> Option<PointOffsetType> {
        match external_id {
            PointIdType::NumId(num) => self.mappings.external_to_internal_num.get(&num).copied(),
            PointIdType::Uuid(uuid) => self.mappings.external_to_internal_uuid.get(&uuid).copied(),
        }
    }

    fn external_id(&self, internal_id: PointOffsetType) -> Option<PointIdType> {
        if *self.deleted.get(internal_id as usize)? {
            return None;
        }

        self.mappings
            .internal_to_external
            .get(internal_id as usize)
            .map(|i| i.into())
    }

    fn set_link(
        &mut self,
        _external_id: PointIdType,
        _internal_id: PointOffsetType,
    ) -> OperationResult<()> {
        panic!("Trying to call a mutating function (`set_link`) of an immutable id tracker");
    }

    fn drop(&mut self, external_id: PointIdType) -> OperationResult<()> {
        let internal_id = match external_id {
            // We "temporarily" remove existing points from the BTreeMaps without writing them to disk
            // because we remove deleted points of a previous load directly when loading.
            PointIdType::NumId(num) => self.mappings.external_to_internal_num.remove(&num),
            PointIdType::Uuid(uuid) => self.mappings.external_to_internal_uuid.remove(&uuid),
        };

        if let Some(internal_id) = internal_id {
            self.deleted.set(internal_id as usize, true);
            self.deleted_wrapper.set(internal_id as usize, true);
        }

        Ok(())
    }

    fn iter_external(&self) -> Box<dyn Iterator<Item = PointIdType> + '_> {
        let iter_num = self
            .mappings
            .external_to_internal_num
            .keys()
            .map(|i| PointIdType::NumId(*i));

        let iter_uuid = self
            .mappings
            .external_to_internal_uuid
            .keys()
            .map(|i| PointIdType::Uuid(*i));
        // order is important here, we want to iterate over the u64 ids first
        Box::new(iter_num.chain(iter_uuid))
    }

    fn iter_internal(&self) -> Box<dyn Iterator<Item = PointOffsetType> + '_> {
        Box::new(
            (0..self.mappings.internal_to_external.len() as PointOffsetType)
                .filter(move |i| !self.deleted[*i as usize]),
        )
    }

    fn iter_from(
        &self,
        external_id: Option<PointIdType>,
    ) -> Box<dyn Iterator<Item = (PointIdType, PointOffsetType)> + '_> {
        let full_num_iter = || {
            self.mappings
                .external_to_internal_num
                .iter()
                .map(|(k, v)| (PointIdType::NumId(*k), *v))
        };
        let offset_num_iter = |offset: u64| {
            self.mappings
                .external_to_internal_num
                .range(offset..)
                .map(|(k, v)| (PointIdType::NumId(*k), *v))
        };
        let full_uuid_iter = || {
            self.mappings
                .external_to_internal_uuid
                .iter()
                .map(|(k, v)| (PointIdType::Uuid(*k), *v))
        };
        let offset_uuid_iter = |offset: Uuid| {
            self.mappings
                .external_to_internal_uuid
                .range(offset..)
                .map(|(k, v)| (PointIdType::Uuid(*k), *v))
        };

        match external_id {
            None => {
                let iter_num = full_num_iter();
                let iter_uuid = full_uuid_iter();
                // order is important here, we want to iterate over the u64 ids first
                Box::new(iter_num.chain(iter_uuid))
            }
            Some(offset) => match offset {
                PointIdType::NumId(idx) => {
                    // Because u64 keys are less that uuid key, we can just use the full iterator for uuid
                    let iter_num = offset_num_iter(idx);
                    let iter_uuid = full_uuid_iter();
                    // order is important here, we want to iterate over the u64 ids first
                    Box::new(iter_num.chain(iter_uuid))
                }
                PointIdType::Uuid(uuid) => {
                    // if offset is a uuid, we can only iterate over uuids
                    Box::new(offset_uuid_iter(uuid))
                }
            },
        }
    }

    fn iter_ids(&self) -> Box<dyn Iterator<Item = PointOffsetType> + '_> {
        self.iter_internal()
    }

    fn iter_random(&self) -> Box<dyn Iterator<Item = (PointIdType, PointOffsetType)> + '_> {
        let rng = rand::thread_rng();
        let max_internal = self.mappings.internal_to_external.len();
        if max_internal == 0 {
            return Box::new(iter::empty());
        }
        let uniform = rand::distributions::Uniform::new(0, max_internal);
        let iter = Distribution::sample_iter(uniform, rng)
            // TODO: this is not efficient if `max_internal` is large and we iterate over most of them,
            // but it's good enough for low limits.
            //
            // We could improve it by using a variable-period PRNG to adjust depending on the number of available points.
            .unique()
            .take(max_internal)
            .filter_map(move |i| {
                if self.deleted[i] {
                    None
                } else {
                    Some((self.mappings.internal_to_external[i], i as PointOffsetType))
                }
            });

        Box::new(iter)
    }

    /// Creates a flusher function, that writes the deleted points bitvec to disk.
    fn mapping_flusher(&self) -> Flusher {
        // Only flush deletions because mappings are immutable
        self.deleted_wrapper.flusher()
    }

    /// Creates a flusher function, that writes the points versions to disk.
    fn versions_flusher(&self) -> Flusher {
        self.internal_to_version_wrapper.flusher()
    }

    fn total_point_count(&self) -> usize {
        self.mappings.internal_to_external.len()
    }

    fn available_point_count(&self) -> usize {
        self.mappings.external_to_internal_num.len() + self.mappings.external_to_internal_uuid.len()
    }

    fn deleted_point_count(&self) -> usize {
        self.total_point_count() - self.available_point_count()
    }

    fn deleted_point_bitslice(&self) -> &BitSlice {
        &self.deleted
    }

    fn is_deleted_point(&self, key: PointOffsetType) -> bool {
        let key = key as usize;
        if key >= self.deleted.len() {
            return true;
        }
        self.deleted[key]
    }

    fn name(&self) -> &'static str {
        "immutable id tracker"
    }

    fn cleanup_versions(&mut self) -> OperationResult<()> {
        let mut to_remove = Vec::new();
        for internal_id in self.iter_internal() {
            if self.internal_version(internal_id).is_none() {
                if let Some(external_id) = self.external_id(internal_id) {
                    to_remove.push(external_id);
                } else {
                    debug_assert!(false, "internal id {} has no external id", internal_id);
                }
            }
        }
        for external_id in to_remove {
            self.drop(external_id)?;
            #[cfg(debug_assertions)] // Only for dev builds
            {
                log::debug!("dropped version for point {} without version", external_id);
            }
        }
        Ok(())
    }

    fn files(&self) -> Vec<PathBuf> {
        vec![
            Self::deleted_file_path(&self.path),
            Self::mappings_file_path(&self.path),
            Self::version_mapping_file_path(&self.path),
        ]
    }
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};

    use itertools::Itertools;
    use rand::prelude::*;
    use rand::Rng;
    use tempfile::Builder;

    use super::*;
    use crate::common::rocksdb_wrapper::{open_db, DB_VECTOR_CF};
    use crate::id_tracker::simple_id_tracker::SimpleIdTracker;
    use crate::id_tracker::IdTrackerEnum;

    const RAND_SEED: u64 = 42;

    #[test]
    fn test_iterator() {
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let db = open_db(dir.path(), &[DB_VECTOR_CF]).unwrap();

        let mut id_tracker = SimpleIdTracker::open(db).unwrap();

        id_tracker.set_link(200.into(), 0).unwrap();
        id_tracker.set_link(100.into(), 1).unwrap();
        id_tracker.set_link(150.into(), 2).unwrap();
        id_tracker.set_link(120.into(), 3).unwrap();
        id_tracker.set_link(180.into(), 4).unwrap();
        id_tracker.set_link(110.into(), 5).unwrap();
        id_tracker.set_link(115.into(), 6).unwrap();
        id_tracker.set_link(190.into(), 7).unwrap();
        id_tracker.set_link(177.into(), 8).unwrap();
        id_tracker.set_link(118.into(), 9).unwrap();

        let id_tracker = id_tracker.make_immutable(dir.path()).unwrap();

        let first_four = id_tracker.iter_from(None).take(4).collect_vec();

        assert_eq!(first_four.len(), 4);
        assert_eq!(first_four[0].0, 100.into());

        let last = id_tracker.iter_from(Some(first_four[3].0)).collect_vec();
        assert_eq!(last.len(), 7);
    }

    const TEST_POINTS: &[PointIdType] = &[
        PointIdType::NumId(100),
        PointIdType::Uuid(Uuid::from_u128(123_u128)),
        PointIdType::Uuid(Uuid::from_u128(156_u128)),
        PointIdType::NumId(150),
        PointIdType::NumId(120),
        PointIdType::Uuid(Uuid::from_u128(12_u128)),
        PointIdType::NumId(180),
        PointIdType::NumId(110),
        PointIdType::NumId(115),
        PointIdType::Uuid(Uuid::from_u128(673_u128)),
        PointIdType::NumId(190),
        PointIdType::NumId(177),
        PointIdType::Uuid(Uuid::from_u128(971_u128)),
    ];

    fn make_immutable_tracker(path: &Path) -> ImmutableIdTracker {
        let db = open_db(path, &[DB_VECTOR_CF]).unwrap();

        let mut id_tracker = SimpleIdTracker::open(db).unwrap();

        for (id, value) in TEST_POINTS.iter().enumerate() {
            id_tracker.set_link(*value, id as PointOffsetType).unwrap();
        }

        match id_tracker.make_immutable(path).unwrap() {
            IdTrackerEnum::MutableIdTracker(_) => {
                unreachable!()
            }
            IdTrackerEnum::ImmutableIdTracker(m) => {
                m.mapping_flusher()().unwrap();
                m.versions_flusher()().unwrap();
                m
            }
        }
    }

    #[test]
    fn test_mixed_types_iterator() {
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let id_tracker = make_immutable_tracker(dir.path());

        let sorted_from_tracker = id_tracker.iter_from(None).map(|(k, _)| k).collect_vec();

        let mut values = TEST_POINTS.to_vec();
        values.sort();

        assert_eq!(sorted_from_tracker, values);
    }

    #[test]
    fn test_load_store() {
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let (old_deleted, old_mappings, old_versions) = {
            let id_tracker = make_immutable_tracker(dir.path());
            (
                id_tracker.deleted.to_bitvec(),
                id_tracker.mappings,
                id_tracker.internal_to_version,
            )
        };

        let mut loaded_id_tracker = ImmutableIdTracker::open(dir.path()).unwrap();

        // We may extend the length of deleted bitvec as memory maps need to be aligned to
        // a multiple of `usize-width`.
        assert_eq!(old_deleted, loaded_id_tracker.deleted[..old_deleted.len()]);

        assert_eq!(old_versions, loaded_id_tracker.internal_to_version);

        assert_eq!(old_mappings, loaded_id_tracker.mappings);

        loaded_id_tracker.drop(PointIdType::NumId(180)).unwrap();
    }

    /// Mutates an ID tracker and stores it to disk. Tests whether loading results in the exact same
    /// ID tracker.
    #[test]
    fn test_store_load_mutated() {
        let mut rng = StdRng::seed_from_u64(RAND_SEED);

        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let (dropped_points, custom_version) = {
            let mut id_tracker = make_immutable_tracker(dir.path());

            let mut dropped_points = HashSet::new();
            let mut custom_version = HashMap::new();

            for (index, point) in TEST_POINTS.iter().enumerate() {
                if index % 2 == 0 {
                    continue;
                }

                if index % 3 == 0 {
                    id_tracker.drop(*point).unwrap();
                    dropped_points.insert(*point);
                    continue;
                }

                if index % 5 == 0 {
                    let new_version = rng.next_u64();
                    id_tracker
                        .set_internal_version(index as PointOffsetType, new_version)
                        .unwrap();
                    custom_version.insert(index as PointOffsetType, new_version);
                }
            }

            id_tracker.mapping_flusher()().unwrap();
            id_tracker.versions_flusher()().unwrap();

            (dropped_points, custom_version)
        };

        let id_tracker = ImmutableIdTracker::open(dir.path()).unwrap();
        for (index, point) in TEST_POINTS.iter().enumerate() {
            let internal_id = index as PointOffsetType;

            if dropped_points.contains(point) {
                assert!(id_tracker.is_deleted_point(internal_id));
                assert_eq!(id_tracker.external_id(internal_id), None);
                match point {
                    PointIdType::NumId(num) => {
                        assert!(!id_tracker
                            .mappings
                            .external_to_internal_num
                            .contains_key(num));
                    }
                    PointIdType::Uuid(uuid) => {
                        assert!(!id_tracker
                            .mappings
                            .external_to_internal_uuid
                            .contains_key(uuid));
                    }
                }

                continue;
            }

            // Check version
            let expect_version = custom_version.get(&internal_id).unwrap_or(&0);
            assert_eq!(
                id_tracker.internal_to_version.get(internal_id as usize),
                Some(expect_version)
            );

            // Check that unmodified points still haven't changed.
            assert_eq!(
                id_tracker.external_id(index as PointOffsetType),
                Some(*point)
            );
        }
    }

    #[test]
    fn test_all_points_have_version() {
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let id_tracker = make_immutable_tracker(dir.path());
        for i in id_tracker.iter_ids() {
            assert!(id_tracker.internal_version(i).is_some());
        }
    }

    #[test]
    fn test_point_deletion_correctness() {
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();
        let id_tracker = make_immutable_tracker(dir.path());
        assert_point_deletion_correctness(IdTrackerEnum::ImmutableIdTracker(id_tracker));
    }

    fn assert_point_deletion_correctness(mut id_tracker: IdTrackerEnum) {
        // No deletions yet
        assert_eq!(
            id_tracker.total_point_count(),
            id_tracker.available_point_count()
        );

        let point_to_delete = PointIdType::NumId(100);

        assert!(id_tracker.iter_external().contains(&point_to_delete));

        assert_eq!(id_tracker.internal_id(point_to_delete), Some(0));

        id_tracker.drop(point_to_delete).unwrap();

        assert!(!point_exists(&id_tracker, point_to_delete));

        assert_eq!(
            id_tracker.available_point_count(),
            id_tracker.total_point_count() - 1
        );
    }

    fn point_exists(id_tracker: &IdTrackerEnum, point: PointIdType) -> bool {
        id_tracker.internal_id(point).is_some()
            && id_tracker.iter_external().contains(&point)
            && id_tracker.iter_from(None).any(|i| i.0 == point)
    }

    #[test]
    fn test_point_deletion_persists_reload() {
        let dir = Builder::new().prefix("storage_dir").tempdir().unwrap();

        let point_to_delete = PointIdType::NumId(100);

        let old_mappings = {
            let mut id_tracker = make_immutable_tracker(dir.path());
            let intetrnal_id = id_tracker
                .internal_id(point_to_delete)
                .expect("Point to delete exists.");
            assert!(!id_tracker.is_deleted_point(intetrnal_id));
            id_tracker.drop(point_to_delete).unwrap();
            id_tracker.versions_flusher()().unwrap();
            id_tracker.mapping_flusher()().unwrap();
            id_tracker.mappings
        };

        // Point should still be gone
        let id_tracker = ImmutableIdTracker::open(dir.path()).unwrap();
        assert_eq!(id_tracker.internal_id(point_to_delete), None);

        // Old mappings should be the same as newly loaded one.
        assert_eq!(
            old_mappings.external_to_internal_num,
            id_tracker.mappings.external_to_internal_num
        );
    }

    fn gen_random_point_mappings(size: usize, rand: &mut StdRng) -> PointMappings {
        const UUID_LIKELYNESS: f64 = 0.5;

        let mut external_to_internal_num = BTreeMap::new();
        let mut external_to_internal_uuid = BTreeMap::new();

        let internal_to_external = (0..size)
            .map(|_| {
                if rand.gen_bool(UUID_LIKELYNESS) {
                    PointIdType::Uuid(Uuid::new_v4())
                } else {
                    PointIdType::NumId(rand.next_u64())
                }
            })
            .enumerate()
            .inspect(|(pos, point_type)| match point_type {
                ExtendedPointId::NumId(num) => {
                    external_to_internal_num.insert(*num, *pos as u32);
                }
                ExtendedPointId::Uuid(uuid) => {
                    external_to_internal_uuid.insert(*uuid, *pos as u32);
                }
            })
            .map(|i| i.1)
            .collect();

        PointMappings {
            internal_to_external,
            external_to_internal_num,
            external_to_internal_uuid,
        }
    }

    /// Tests de/serializing of whole `PointMappings`.
    #[test]
    fn test_point_mappings_de_serialization() {
        let mut rng = StdRng::seed_from_u64(RAND_SEED);

        let mut buf = vec![];

        // Test different sized PointMappings, growing exponentially to also test large ones.
        // This way we test up to 2^22=4_194_304 points.
        for size_exp in (0..23u32).step_by(3) {
            buf.clear();

            let size = 2usize.pow(size_exp);

            let mappings = gen_random_point_mappings(size, &mut rng);

            mappings.store(&mut buf).unwrap();

            // 16 is the min byte size of an entry. The exact number is not that important
            // we just want to ensure that the written bytes correlate to the amount of entries.
            assert!(buf.len() >= size * 16);

            let new_mappings = PointMappings::load(&*buf, None).unwrap();

            assert_eq!(new_mappings.internal_to_external.len(), size);
            assert_eq!(mappings, new_mappings);
        }
    }

    /// Verifies that de/serializing works properly for empty `PointMappings`.
    #[test]
    fn test_point_mappings_de_serialization_empty() {
        let mut rng = StdRng::seed_from_u64(RAND_SEED);
        let mappings = gen_random_point_mappings(0, &mut rng);

        let mut buf = vec![];

        mappings.store(&mut buf).unwrap();

        // We still have a header!
        assert!(!buf.is_empty());

        let new_mappings = PointMappings::load(&*buf, None).unwrap();

        assert!(new_mappings.internal_to_external.is_empty());
        assert_eq!(mappings, new_mappings);
    }

    /// Tests de/serializing of only single ID mappings.
    #[test]
    fn test_point_mappings_de_serialization_single() {
        let mut rng = StdRng::seed_from_u64(RAND_SEED);

        const SIZE: usize = 400_000;

        let mappings = gen_random_point_mappings(SIZE, &mut rng);

        for i in 0..SIZE {
            let mut buf = vec![];

            let expected_external = mappings.internal_to_external[i];

            mappings.write_entry(&mut buf, &expected_external).unwrap();

            let (got_internal, got_external) = PointMappings::read_entry(&*buf).unwrap();

            assert_eq!(i as PointOffsetType, got_internal);
            assert_eq!(expected_external, got_external);
        }
    }
}
