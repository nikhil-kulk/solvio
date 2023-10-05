use std::path::{Path, PathBuf};

use segment::common::version::StorageVersion as _;
use tokio::fs;

use super::Collection;
use crate::collection::CollectionVersion;
use crate::common::file_utils::FileCleaner;
use crate::config::CollectionConfig;
use crate::operations::snapshot_ops::{self, SnapshotDescription};
use crate::operations::types::{CollectionError, CollectionResult, NodeType};
use crate::shards::local_shard::LocalShard;
use crate::shards::remote_shard::RemoteShard;
use crate::shards::replica_set::ShardReplicaSet;
use crate::shards::shard::{PeerId, ShardId};
use crate::shards::shard_config::{self, ShardConfig};
use crate::shards::shard_versioning;

impl Collection {
    pub async fn list_snapshots(&self) -> CollectionResult<Vec<SnapshotDescription>> {
        snapshot_ops::list_snapshots_in_directory(&self.snapshots_path).await
    }

    /// Creates a snapshot of the collection.
    ///
    /// The snapshot is created in three steps:
    /// 1. Create a temporary directory and create a snapshot of each shard in it.
    /// 2. Archive the temporary directory into a single file.
    /// 3. Move the archive to the final location.
    ///
    /// # Arguments
    ///
    /// * `global_temp_dir`: directory used to host snapshots while they are being created
    /// * `this_peer_id`: current peer id
    ///
    /// returns: Result<SnapshotDescription, CollectionError>
    pub async fn create_snapshot(
        &self,
        global_temp_dir: &Path,
        this_peer_id: PeerId,
    ) -> CollectionResult<SnapshotDescription> {
        let snapshot_name = format!(
            "{}-{}-{}.snapshot",
            self.name(),
            this_peer_id,
            chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S")
        );

        // Final location of snapshot
        let snapshot_path = self.snapshots_path.join(&snapshot_name);
        log::info!(
            "Creating collection snapshot {} into {:?}",
            snapshot_name,
            snapshot_path
        );

        // Dedicated temporary directory for this snapshot (deleted on drop)
        let snapshot_temp_target_dir = tempfile::Builder::new()
            .prefix(&format!("{snapshot_name}-target-"))
            .tempdir_in(global_temp_dir)?;

        let snapshot_temp_target_dir_path = snapshot_temp_target_dir.path().to_path_buf();
        // Create snapshot of each shard
        {
            let snapshot_temp_temp_dir = tempfile::Builder::new()
                .prefix(&format!("{snapshot_name}-temp-"))
                .tempdir_in(global_temp_dir)?;
            let shards_holder = self.shards_holder.read().await;
            // Create snapshot of each shard
            for (shard_id, replica_set) in shards_holder.get_shards() {
                let shard_snapshot_path = shard_versioning::versioned_shard_path(
                    &snapshot_temp_target_dir_path,
                    *shard_id,
                    0,
                );
                fs::create_dir_all(&shard_snapshot_path).await?;
                // If node is listener, we can save whatever currently is in the storage
                let save_wal = self.shared_storage_config.node_type != NodeType::Listener;
                replica_set
                    .create_snapshot(
                        snapshot_temp_temp_dir.path(),
                        &shard_snapshot_path,
                        save_wal,
                    )
                    .await?;
            }
        }

        // Save collection config and version
        CollectionVersion::save(&snapshot_temp_target_dir_path)?;
        self.collection_config
            .read()
            .await
            .save(&snapshot_temp_target_dir_path)?;

        // Dedicated temporary file for archiving this snapshot (deleted on drop)
        let mut snapshot_temp_arc_file = tempfile::Builder::new()
            .prefix(&format!("{snapshot_name}-arc-"))
            .tempfile_in(global_temp_dir)?;

        // Archive snapshot folder into a single file
        log::debug!("Archiving snapshot {:?}", &snapshot_temp_target_dir_path);
        let archiving = tokio::task::spawn_blocking(move || -> CollectionResult<_> {
            let mut builder = tar::Builder::new(snapshot_temp_arc_file.as_file_mut());
            // archive recursively collection directory `snapshot_path_with_arc_extension` into `snapshot_path`
            builder.append_dir_all(".", &snapshot_temp_target_dir_path)?;
            builder.finish()?;
            drop(builder);
            // return ownership of the file
            Ok(snapshot_temp_arc_file)
        });
        snapshot_temp_arc_file = archiving.await??;

        // Move snapshot to permanent location.
        // We can't move right away, because snapshot folder can be on another mounting point.
        // We can't copy to the target location directly, because copy is not atomic.
        // So we copy to the final location with a temporary name and then rename atomically.
        let snapshot_path_tmp_move = snapshot_path.with_extension("tmp");

        // Ensure that the temporary file is deleted on error
        let _file_cleaner = FileCleaner::new(&snapshot_path_tmp_move);
        fs::copy(&snapshot_temp_arc_file.path(), &snapshot_path_tmp_move).await?;
        fs::rename(&snapshot_path_tmp_move, &snapshot_path).await?;

        log::info!(
            "Collection snapshot {} completed into {:?}",
            snapshot_name,
            snapshot_path
        );
        snapshot_ops::get_snapshot_description(&snapshot_path).await
    }

    /// Restore collection from snapshot
    ///
    /// This method performs blocking IO.
    pub fn restore_snapshot(
        snapshot_path: &Path,
        target_dir: &Path,
        this_peer_id: PeerId,
        is_distributed: bool,
    ) -> CollectionResult<()> {
        // decompress archive
        let archive_file = std::fs::File::open(snapshot_path)?;
        let mut ar = tar::Archive::new(archive_file);
        ar.unpack(target_dir)?;

        let config = CollectionConfig::load(target_dir)?;
        config.validate_and_warn();
        let configured_shards = config.params.shard_number.get();

        for shard_id in 0..configured_shards {
            let shard_path = shard_versioning::versioned_shard_path(target_dir, shard_id, 0);
            let shard_config_opt = ShardConfig::load(&shard_path)?;
            if let Some(shard_config) = shard_config_opt {
                match shard_config.r#type {
                    shard_config::ShardType::Local => LocalShard::restore_snapshot(&shard_path)?,
                    shard_config::ShardType::Remote { .. } => {
                        RemoteShard::restore_snapshot(&shard_path)
                    }
                    shard_config::ShardType::Temporary => {}
                    shard_config::ShardType::ReplicaSet { .. } => {
                        ShardReplicaSet::restore_snapshot(
                            &shard_path,
                            this_peer_id,
                            is_distributed,
                        )?
                    }
                }
            } else {
                return Err(CollectionError::service_error(format!(
                    "Can't read shard config at {}",
                    shard_path.display()
                )));
            }
        }

        Ok(())
    }

    pub async fn recover_local_shard_from(
        &self,
        snapshot_shard_path: &Path,
        shard_id: ShardId,
    ) -> CollectionResult<bool> {
        // TODO:
        //   Check that shard snapshot is compatible with the collection
        //   (see `VectorsConfig::check_compatible_with_segment_config`)

        self.shards_holder
            .read()
            .await
            .recover_local_shard_from(snapshot_shard_path, shard_id)
            .await
    }

    pub async fn get_snapshot_path(&self, snapshot_name: &str) -> CollectionResult<PathBuf> {
        let snapshot_path = self.snapshots_path.join(snapshot_name);

        let absolute_snapshot_path =
            snapshot_path
                .canonicalize()
                .map_err(|_| CollectionError::NotFound {
                    what: format!("Snapshot {snapshot_name}"),
                })?;

        let absolute_snapshot_dir =
            self.snapshots_path
                .canonicalize()
                .map_err(|_| CollectionError::NotFound {
                    what: format!("Snapshot directory: {}", self.snapshots_path.display()),
                })?;

        if !absolute_snapshot_path.starts_with(absolute_snapshot_dir) {
            return Err(CollectionError::NotFound {
                what: format!("Snapshot {snapshot_name}"),
            });
        }

        if !snapshot_path.exists() {
            return Err(CollectionError::NotFound {
                what: format!("Snapshot {snapshot_name}"),
            });
        }
        Ok(snapshot_path)
    }

    pub async fn list_shard_snapshots(
        &self,
        shard_id: ShardId,
    ) -> CollectionResult<Vec<SnapshotDescription>> {
        self.shards_holder
            .read()
            .await
            .list_shard_snapshots(&self.snapshots_path, shard_id)
            .await
    }

    pub async fn create_shard_snapshot(
        &self,
        shard_id: ShardId,
        temp_dir: &Path,
    ) -> CollectionResult<SnapshotDescription> {
        self.shards_holder
            .read()
            .await
            .create_shard_snapshot(&self.snapshots_path, &self.name(), shard_id, temp_dir)
            .await
    }

    pub async fn restore_shard_snapshot(
        &self,
        shard_id: ShardId,
        snapshot_path: &Path,
        this_peer_id: PeerId,
        is_distributed: bool,
        temp_dir: &Path,
    ) -> CollectionResult<()> {
        self.shards_holder
            .read()
            .await
            .restore_shard_snapshot(
                snapshot_path,
                &self.name(),
                shard_id,
                this_peer_id,
                is_distributed,
                temp_dir,
            )
            .await
    }

    pub async fn assert_shard_exists(&self, shard_id: ShardId) -> CollectionResult<()> {
        self.shards_holder
            .read()
            .await
            .assert_shard_exists(shard_id)
            .await
    }

    pub async fn get_shard_snapshot_path(
        &self,
        shard_id: ShardId,
        snapshot_file_name: impl AsRef<Path>,
    ) -> CollectionResult<PathBuf> {
        self.shards_holder
            .read()
            .await
            .get_shard_snapshot_path(&self.snapshots_path, shard_id, snapshot_file_name)
            .await
    }
}
