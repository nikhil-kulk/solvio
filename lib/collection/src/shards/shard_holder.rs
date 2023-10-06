use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tar::Builder as TarBuilder;
use tokio::runtime::Handle;
use tokio::sync::RwLock;

use crate::common::file_utils::move_file;
use crate::config::CollectionConfig;
use crate::hash_ring::HashRing;
use crate::operations::shared_storage_config::SharedStorageConfig;
use crate::operations::snapshot_ops::{
    get_snapshot_description, list_snapshots_in_directory, SnapshotDescription,
};
use crate::operations::types::{CollectionError, CollectionResult, ShardTransferInfo};
use crate::operations::{OperationToShard, SplitByShard};
use crate::save_on_disk::SaveOnDisk;
use crate::shards::channel_service::ChannelService;
use crate::shards::local_shard::LocalShard;
use crate::shards::replica_set::{ChangePeerState, ReplicaState, ShardReplicaSet}; // TODO rename ReplicaShard to ReplicaSetShard
use crate::shards::shard::{PeerId, ShardId};
use crate::shards::shard_config::{ShardConfig, ShardType};
use crate::shards::shard_versioning::latest_shard_paths;
use crate::shards::transfer::shard_transfer::{ShardTransfer, ShardTransferKey};
use crate::shards::CollectionId;

const SHARD_TRANSFERS_FILE: &str = "shard_transfers";

pub struct ShardHolder {
    shards: HashMap<ShardId, ShardReplicaSet>,
    pub(crate) shard_transfers: SaveOnDisk<HashSet<ShardTransfer>>,
    ring: HashRing<ShardId>,
}

pub type LockedShardHolder = RwLock<ShardHolder>;

impl ShardHolder {
    pub fn new(collection_path: &Path, hashring: HashRing<ShardId>) -> CollectionResult<Self> {
        let shard_transfers = SaveOnDisk::load_or_init(collection_path.join(SHARD_TRANSFERS_FILE))?;
        Ok(Self {
            shards: HashMap::new(),
            shard_transfers,
            ring: hashring,
        })
    }

    pub fn add_shard(&mut self, shard_id: ShardId, shard: ShardReplicaSet) {
        self.shards.insert(shard_id, shard);
        self.ring.add(shard_id);
    }

    pub fn remove_shard(&mut self, shard_id: ShardId) -> Option<ShardReplicaSet> {
        let shard = self.shards.remove(&shard_id);
        self.ring.remove(&shard_id);
        shard
    }

    /// Take shard
    ///
    /// remove shard and return ownership
    pub fn take_shard(&mut self, shard_id: ShardId) -> Option<ShardReplicaSet> {
        self.shards.remove(&shard_id)
    }

    /// Replace shard
    ///
    /// return old shard
    pub fn replace_shard(
        &mut self,
        shard_id: ShardId,
        shard: ShardReplicaSet,
    ) -> Option<ShardReplicaSet> {
        self.shards.insert(shard_id, shard)
    }

    pub fn contains_shard(&self, shard_id: &ShardId) -> bool {
        self.shards.contains_key(shard_id)
    }

    pub fn get_shard(&self, shard_id: &ShardId) -> Option<&ShardReplicaSet> {
        self.shards.get(shard_id)
    }

    pub fn get_mut_shard(&mut self, shard_id: &ShardId) -> Option<&mut ShardReplicaSet> {
        self.shards.get_mut(shard_id)
    }

    pub fn get_shards(&self) -> impl Iterator<Item = (&ShardId, &ShardReplicaSet)> {
        self.shards.iter()
    }

    pub fn all_shards(&self) -> impl Iterator<Item = &ShardReplicaSet> {
        self.shards.values()
    }

    pub fn split_by_shard<O: SplitByShard + Clone>(
        &self,
        operation: O,
    ) -> Vec<(&ShardReplicaSet, O)> {
        let operation_to_shard = operation.split_by_shard(&self.ring);
        let shard_ops: Vec<_> = match operation_to_shard {
            OperationToShard::ByShard(by_shard) => by_shard
                .into_iter()
                .map(|(shard_id, operation)| (self.shards.get(&shard_id).unwrap(), operation))
                .collect(),
            OperationToShard::ToAll(operation) => self
                .all_shards()
                .map(|shard| (shard, operation.clone()))
                .collect(),
        };
        shard_ops
    }

    pub fn register_start_shard_transfer(&self, transfer: ShardTransfer) -> CollectionResult<bool> {
        Ok(self
            .shard_transfers
            .write(|transfers| transfers.insert(transfer))?)
    }

    pub fn register_finish_transfer(&self, key: &ShardTransferKey) -> CollectionResult<bool> {
        Ok(self.shard_transfers.write(|transfers| {
            let before_remove = transfers.len();
            transfers.retain(|transfer| !key.check(transfer));
            before_remove != transfers.len() // `true` if something was removed
        })?)
    }

    pub fn get_shard_transfer_info(&self) -> Vec<ShardTransferInfo> {
        let mut shard_transfers = vec![];
        for shard_transfer in self.shard_transfers.read().iter() {
            let shard_id = shard_transfer.shard_id;
            let to = shard_transfer.to;
            let from = shard_transfer.from;
            let sync = shard_transfer.sync;
            shard_transfers.push(ShardTransferInfo {
                shard_id,
                from,
                to,
                sync,
            })
        }
        shard_transfers.sort_by_key(|k| k.shard_id);
        shard_transfers
    }

    pub fn get_related_transfers(
        &self,
        shard_id: &ShardId,
        peer_id: &PeerId,
    ) -> Vec<ShardTransfer> {
        self.shard_transfers
            .read()
            .iter()
            .filter(|transfer| transfer.shard_id == *shard_id)
            .filter(|transfer| transfer.from == *peer_id || transfer.to == *peer_id)
            .cloned()
            .collect()
    }

    pub fn target_shard(
        &self,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<&ShardReplicaSet>> {
        match shard_selection {
            None => Ok(self.all_shards().collect()),
            Some(shard_selection) => {
                let shard_opt = self.get_shard(&shard_selection);
                let shards = match shard_opt {
                    None => vec![],
                    Some(shard) => vec![shard],
                };
                Ok(shards)
            }
        }
    }

    pub fn len(&self) -> usize {
        self.shards.len()
    }

    pub fn is_empty(&self) -> bool {
        self.shards.is_empty()
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn load_shards(
        &mut self,
        collection_path: &Path,
        collection_id: &CollectionId,
        collection_config: Arc<RwLock<CollectionConfig>>,
        shared_storage_config: Arc<SharedStorageConfig>,
        channel_service: ChannelService,
        on_peer_failure: ChangePeerState,
        this_peer_id: PeerId,
        update_runtime: Handle,
        search_runtime: Handle,
    ) {
        let shard_number = collection_config.read().await.params.shard_number.get();
        // ToDo: remove after version 0.11.0
        for shard_id in 0..shard_number {
            for (path, _shard_version, shard_type) in
                latest_shard_paths(collection_path, shard_id).await.unwrap()
            {
                let replica_set = ShardReplicaSet::load(
                    shard_id,
                    collection_id.clone(),
                    &path,
                    collection_config.clone(),
                    shared_storage_config.clone(),
                    channel_service.clone(),
                    on_peer_failure.clone(),
                    this_peer_id,
                    update_runtime.clone(),
                    search_runtime.clone(),
                )
                .await;

                let mut require_migration = true;
                match shard_type {
                    ShardType::Local => {
                        // deprecated
                        let local_shard = LocalShard::load(
                            shard_id,
                            collection_id.clone(),
                            &path,
                            collection_config.clone(),
                            shared_storage_config.clone(),
                            update_runtime.clone(),
                        )
                        .await
                        .unwrap();
                        replica_set
                            .set_local(local_shard, Some(ReplicaState::Active))
                            .await
                            .unwrap();
                    }
                    ShardType::Remote { peer_id } => {
                        // deprecated
                        replica_set
                            .add_remote(peer_id, ReplicaState::Active)
                            .await
                            .unwrap();
                    }
                    ShardType::Temporary => {
                        // deprecated
                        let temp_shard = LocalShard::load(
                            shard_id,
                            collection_id.clone(),
                            &path,
                            collection_config.clone(),
                            shared_storage_config.clone(),
                            update_runtime.clone(),
                        )
                        .await
                        .unwrap();

                        replica_set
                            .set_local(temp_shard, Some(ReplicaState::Partial))
                            .await
                            .unwrap();
                    }
                    ShardType::ReplicaSet => {
                        require_migration = false;
                        // nothing to do, replicate set should be loaded already
                    }
                }
                // Migrate shard config to replica set
                // Override existing shard configuration
                if require_migration {
                    ShardConfig::new_replica_set()
                        .save(&path)
                        .map_err(|e| panic!("Failed to save shard config {path:?}: {e}"))
                        .unwrap();
                }

                // Change local shards stuck in Initializing state to Active
                let local_peer_id = replica_set.this_peer_id();
                let not_distributed = !shared_storage_config.is_distributed;
                let is_local =
                    replica_set.this_peer_id() == local_peer_id && replica_set.is_local().await;
                let is_initializing =
                    replica_set.peer_state(&local_peer_id) == Some(ReplicaState::Initializing);
                if not_distributed && is_local && is_initializing {
                    log::warn!("Local shard {collection_id}:{} stuck in Initializing state, changing to Active", replica_set.shard_id);
                    replica_set
                        .set_replica_state(&local_peer_id, ReplicaState::Active)
                        .expect("Failed to set local shard state");
                }

                self.add_shard(shard_id, replica_set);
            }
        }
    }

    pub async fn assert_shard_exists(&self, shard_id: ShardId) -> CollectionResult<()> {
        match self.get_shard(&shard_id) {
            Some(_) => Ok(()),
            None => Err(shard_not_found_error(shard_id)),
        }
    }

    async fn assert_shard_is_local(&self, shard_id: ShardId) -> CollectionResult<()> {
        let is_local_shard = self
            .is_shard_local(&shard_id)
            .await
            .ok_or_else(|| shard_not_found_error(shard_id))?;

        if is_local_shard {
            Ok(())
        } else {
            Err(CollectionError::bad_input(format!(
                "Shard {shard_id} is not a local shard"
            )))
        }
    }

    /// Returns true if shard it explicitly local, false otherwise.
    pub async fn is_shard_local(&self, shard_id: &ShardId) -> Option<bool> {
        match self.get_shard(shard_id) {
            Some(shard) => Some(shard.is_local().await),
            None => None,
        }
    }

    /// Return a list of local shards, present on this peer
    pub async fn get_local_shards(&self) -> Vec<ShardId> {
        let mut res = Vec::with_capacity(1);
        for (shard_id, replica_set) in self.get_shards() {
            if replica_set.has_local_shard().await {
                res.push(*shard_id);
            }
        }
        res
    }

    pub async fn is_all_active(&self) -> bool {
        self.get_shards().all(|(_, replica_set)| {
            replica_set
                .peers()
                .into_iter()
                .all(|(_, state)| state == ReplicaState::Active)
        })
    }

    pub async fn check_transfer_exists(&self, transfer_key: &ShardTransferKey) -> bool {
        self.shard_transfers
            .read()
            .iter()
            .any(|transfer| transfer_key.check(transfer))
    }

    pub async fn get_transfer(&self, transfer_key: &ShardTransferKey) -> Option<ShardTransfer> {
        self.shard_transfers
            .read()
            .iter()
            .find(|transfer| transfer_key.check(transfer))
            .cloned()
    }

    pub async fn get_transfers<F>(&self, mut predicate: F) -> Vec<ShardTransfer>
    where
        F: FnMut(&ShardTransfer) -> bool,
    {
        self.shard_transfers
            .read()
            .iter()
            .filter(|&transfer| predicate(transfer))
            .cloned()
            .collect()
    }

    pub async fn get_outgoing_transfers(&self, current_peer_id: &PeerId) -> Vec<ShardTransfer> {
        self.get_transfers(|transfer| transfer.from == *current_peer_id)
            .await
    }

    pub async fn list_shard_snapshots(
        &self,
        snapshots_path: &Path,
        shard_id: ShardId,
    ) -> CollectionResult<Vec<SnapshotDescription>> {
        self.assert_shard_is_local(shard_id).await?;

        let snapshots_path = self.snapshots_path_for_shard_unchecked(snapshots_path, shard_id);

        if !snapshots_path.exists() {
            return Ok(Vec::new());
        }

        list_snapshots_in_directory(&snapshots_path).await
    }

    pub async fn create_shard_snapshot(
        &self,
        snapshots_path: &Path,
        collection_name: &str,
        shard_id: ShardId,
        temp_dir: &Path,
    ) -> CollectionResult<SnapshotDescription> {
        let shard = self
            .get_shard(&shard_id)
            .ok_or_else(|| shard_not_found_error(shard_id))?;

        if !shard.is_local().await {
            return Err(CollectionError::bad_input(format!(
                "Shard {shard_id} is not a local shard"
            )));
        }

        let snapshot_file_name = format!(
            "{collection_name}-shard-{shard_id}-{}.snapshot",
            chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S"),
        );

        let snapshot_temp_dir = tempfile::Builder::new()
            .prefix(&format!("{snapshot_file_name}-temp-"))
            .tempdir_in(temp_dir)?;

        let snapshot_target_dir = tempfile::Builder::new()
            .prefix(&format!("{snapshot_file_name}-target-"))
            .tempdir_in(temp_dir)?;

        shard
            .create_snapshot(snapshot_temp_dir.path(), snapshot_target_dir.path(), false)
            .await?;

        if let Err(err) = snapshot_temp_dir.close() {
            log::error!("Failed to remove temporary directory: {err}");
        }

        let mut temp_file = tempfile::Builder::new()
            .prefix(&format!("{snapshot_file_name}-"))
            .tempfile_in(temp_dir)?;

        let task = {
            let snapshot_target_dir = snapshot_target_dir.path().to_path_buf();

            tokio::task::spawn_blocking(move || -> CollectionResult<_> {
                let mut tar = TarBuilder::new(temp_file.as_file_mut());
                tar.append_dir_all(".", &snapshot_target_dir)?;
                tar.finish()?;
                drop(tar);

                Ok(temp_file)
            })
        };

        let task_result = task.await;

        if let Err(err) = snapshot_target_dir.close() {
            log::error!("Failed to remove temporary directory: {err}");
        }

        let temp_file = task_result??;

        let snapshot_path =
            self.shard_snapshot_path_unchecked(snapshots_path, shard_id, snapshot_file_name)?;

        if let Some(snapshot_dir) = snapshot_path.parent() {
            if !snapshot_dir.exists() {
                std::fs::create_dir_all(snapshot_dir)?;
            }
        }

        move_file(temp_file.path(), &snapshot_path).await?;

        get_snapshot_description(&snapshot_path).await
    }

    pub async fn restore_shard_snapshot(
        &self,
        snapshot_path: &Path,
        collection_name: &str,
        shard_id: ShardId,
        this_peer_id: PeerId,
        is_distributed: bool,
        temp_dir: &Path,
    ) -> CollectionResult<()> {
        if !self.contains_shard(&shard_id) {
            return Err(shard_not_found_error(shard_id));
        }

        let snapshot = std::fs::File::open(snapshot_path)?;

        if !temp_dir.exists() {
            std::fs::create_dir_all(temp_dir)?;
        }

        let snapshot_file_name = snapshot_path.file_name().unwrap().to_string_lossy();

        let snapshot_temp_dir = tempfile::Builder::new()
            .prefix(&format!(
                "{collection_name}-shard-{shard_id}-{snapshot_file_name}"
            ))
            .tempdir_in(temp_dir)?;

        let task = {
            let snapshot_temp_dir = snapshot_temp_dir.path().to_path_buf();

            tokio::task::spawn_blocking(move || -> CollectionResult<_> {
                let mut tar = tar::Archive::new(snapshot);
                tar.unpack(&snapshot_temp_dir)?;
                drop(tar);

                ShardReplicaSet::restore_snapshot(
                    &snapshot_temp_dir,
                    this_peer_id,
                    is_distributed,
                )?;

                Ok(())
            })
        };

        task.await??;

        let recovered = self
            .recover_local_shard_from(snapshot_temp_dir.path(), shard_id)
            .await?;

        if !recovered {
            return Err(CollectionError::bad_request(format!(
                "Invalid snapshot {snapshot_file_name}"
            )));
        }

        Ok(())
    }

    pub async fn recover_local_shard_from(
        &self,
        snapshot_shard_path: &Path,
        shard_id: ShardId,
    ) -> CollectionResult<bool> {
        let replica_set = self
            .get_shard(&shard_id)
            .ok_or_else(|| shard_not_found_error(shard_id))?;

        // TODO:
        //   Check that shard snapshot is compatible with the collection
        //   (see `VectorsConfig::check_compatible_with_segment_config`)

        replica_set
            .restore_local_replica_from(snapshot_shard_path)
            .await
    }

    pub async fn get_shard_snapshot_path(
        &self,
        snapshots_path: &Path,
        shard_id: ShardId,
        snapshot_file_name: impl AsRef<Path>,
    ) -> CollectionResult<PathBuf> {
        self.assert_shard_is_local(shard_id).await?;
        self.shard_snapshot_path_unchecked(snapshots_path, shard_id, snapshot_file_name)
    }

    fn snapshots_path_for_shard_unchecked(
        &self,
        snapshots_path: &Path,
        shard_id: ShardId,
    ) -> PathBuf {
        snapshots_path.join(format!("shards/{shard_id}"))
    }

    fn shard_snapshot_path_unchecked(
        &self,
        snapshots_path: &Path,
        shard_id: ShardId,
        snapshot_file_name: impl AsRef<Path>,
    ) -> CollectionResult<PathBuf> {
        let snapshots_path = self.snapshots_path_for_shard_unchecked(snapshots_path, shard_id);

        let snapshot_file_name = snapshot_file_name.as_ref();

        if snapshot_file_name.file_name() != Some(snapshot_file_name.as_os_str()) {
            return Err(CollectionError::bad_input(format!(
                "Invalid snapshot file name {}",
                snapshot_file_name.display(),
            )));
        }

        let snapshot_path = snapshots_path.join(snapshot_file_name);

        Ok(snapshot_path)
    }

    pub async fn remove_shards_at_peer(&self, peer_id: PeerId) -> CollectionResult<()> {
        for (_shard_id, replica_set) in self.get_shards() {
            replica_set.remove_peer(peer_id).await?;
        }
        Ok(())
    }
}

pub(crate) fn shard_not_found_error(shard_id: ShardId) -> CollectionError {
    CollectionError::NotFound {
        what: format!("shard {shard_id}"),
    }
}
