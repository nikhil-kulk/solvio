use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::num::NonZeroU32;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use futures::future::{join_all, try_join_all};
use itertools::Itertools;
use segment::common::version::StorageVersion;
use segment::data_types::vectors::{NamedVector, VectorElementType, DEFAULT_VECTOR_NAME};
use segment::spaces::tools::{peek_top_largest_iterable, peek_top_smallest_iterable};
use segment::types::{
    Condition, ExtendedPointId, Filter, HasIdCondition, Order, ScoredPoint, WithPayload,
    WithPayloadInterface, WithVector,
};
use semver::Version;
use tar::Builder as TarBuilder;
use tokio::fs::{copy, create_dir_all, remove_dir_all, remove_file, rename};
use tokio::runtime::Handle;
use tokio::sync::{Mutex, RwLock};

use crate::collection_state::{ShardInfo, State};
use crate::config::CollectionConfig;
use crate::hash_ring::HashRing;
use crate::operations::config_diff::{CollectionParamsDiff, DiffConfig, OptimizersConfigDiff};
use crate::operations::snapshot_ops::{
    get_snapshot_description, list_snapshots_in_directory, SnapshotDescription,
};
use crate::operations::types::{
    CollectionClusterInfo, CollectionError, CollectionInfo, CollectionResult, CountRequest,
    CountResult, LocalShardInfo, PointRequest, RecommendRequest, RecommendRequestBatch, Record,
    RemoteShardInfo, ScrollRequest, ScrollResult, SearchRequest, SearchRequestBatch,
    ShardTransferInfo, UpdateResult, UsingVector,
};
use crate::operations::{CollectionUpdateOperations, Validate};
use crate::optimizers_builder::OptimizersConfig;
use crate::shards::channel_service::ChannelService;
use crate::shards::collection_shard_distribution::CollectionShardDistribution;
use crate::shards::local_shard::LocalShard;
use crate::shards::remote_shard::RemoteShard;
use crate::shards::replica_set::{
    Change, OnPeerFailure, ReplicaState, ShardReplicaSet as ReplicaSetShard,
}; // TODO rename ReplicaShard to ReplicaSetShard
use crate::shards::shard::{PeerId, ShardId};
use crate::shards::shard_config::{self, ShardConfig};
use crate::shards::shard_holder::{LockedShardHolder, ShardHolder};
use crate::shards::shard_trait::ShardOperation;
use crate::shards::shard_versioning::versioned_shard_path;
use crate::shards::transfer::shard_transfer::{
    change_remote_shard_route, finalize_partial_shard, handle_transferred_shard_proxy,
    revert_proxy_shard_to_local, spawn_transfer_task, ShardTransfer, ShardTransferKey,
};
use crate::shards::transfer::transfer_tasks_pool::{TaskResult, TransferTasksPool};
use crate::shards::{replica_set, CollectionId, HASH_RING_SHARD_SCALE};
use crate::telemetry::CollectionTelemetry;

pub type RequestShardTransfer = Arc<dyn Fn(ShardTransfer) + Send + Sync>;

struct CollectionVersion;

impl StorageVersion for CollectionVersion {
    fn current() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

/// Collection's data is split into several shards.
pub struct Collection {
    pub(crate) id: CollectionId,
    pub(crate) shards_holder: Arc<LockedShardHolder>,
    pub(crate) config: Arc<RwLock<CollectionConfig>>,
    /// Tracks whether `before_drop` fn has been called.
    before_drop_called: bool,
    this_peer_id: PeerId,
    path: PathBuf,
    snapshots_path: PathBuf,
    channel_service: ChannelService,
    transfer_tasks: Mutex<TransferTasksPool>,
    request_shard_transfer_cb: RequestShardTransfer,
    #[allow(dead_code)] //Might be useful in case of repartition implementation
    notify_peer_failure_cb: OnPeerFailure,
    init_time: Duration,
}

impl Collection {
    pub fn name(&self) -> String {
        self.id.clone()
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn new(
        name: CollectionId,
        this_peer_id: PeerId,
        path: &Path,
        snapshots_path: &Path,
        config: &CollectionConfig,
        shard_distribution: CollectionShardDistribution,
        channel_service: ChannelService,
        on_replica_failure: replica_set::OnPeerFailure,
        request_shard_transfer: RequestShardTransfer,
    ) -> Result<Self, CollectionError> {
        let start_time = std::time::Instant::now();

        let mut shard_holder = ShardHolder::new(path, HashRing::fair(HASH_RING_SHARD_SCALE))?;

        let shared_config = Arc::new(RwLock::new(config.clone()));
        for (shard_id, mut peers) in shard_distribution.shards {
            let is_local = peers.contains(&this_peer_id);
            peers.remove(&this_peer_id);

            let replica_set = ReplicaSetShard::build(
                shard_id,
                name.clone(),
                this_peer_id,
                is_local,
                peers,
                on_replica_failure.clone(),
                path,
                shared_config.clone(),
                channel_service.clone(),
            )
            .await;

            let replica_set = match replica_set {
                Ok(replica_set) => replica_set,
                Err(err) => {
                    shard_holder.before_drop().await;
                    return Err(err);
                }
            };
            shard_holder.add_shard(shard_id, replica_set);
        }

        let locked_shard_holder = Arc::new(LockedShardHolder::new(shard_holder));

        // Once the config is persisted - the collection is considered to be successfully created.
        CollectionVersion::save(path)?;
        config.save(path)?;

        Ok(Self {
            id: name.clone(),
            shards_holder: locked_shard_holder,
            config: shared_config,
            before_drop_called: false,
            this_peer_id,
            path: path.to_owned(),
            snapshots_path: snapshots_path.to_owned(),
            channel_service,
            transfer_tasks: Default::default(),
            request_shard_transfer_cb: request_shard_transfer.clone(),
            notify_peer_failure_cb: on_replica_failure.clone(),
            init_time: start_time.elapsed(),
        })
    }

    /// Check if stored version have consequent version.
    /// If major version is different, then it is not compatible.
    /// If the difference in consecutive versions is greater than 1 in patch,
    /// then the collection is not compatible with the current version.
    ///
    /// Example:
    ///   0.4.0 -> 0.4.1 = true
    ///   0.4.0 -> 0.4.2 = false
    ///   0.4.0 -> 0.5.0 = false
    ///   0.4.0 -> 0.5.1 = false
    pub fn can_upgrade_storage(stored: &Version, app: &Version) -> bool {
        if stored.major != app.major {
            return false;
        }
        if stored.minor != app.minor {
            return false;
        }
        if stored.patch + 1 < app.patch {
            return false;
        }
        true
    }

    pub async fn load(
        collection_id: CollectionId,
        this_peer_id: PeerId,
        path: &Path,
        snapshots_path: &Path,
        channel_service: ChannelService,
        on_replica_failure: replica_set::OnPeerFailure,
        request_shard_transfer: RequestShardTransfer,
    ) -> Self {
        let start_time = std::time::Instant::now();
        let stored_version = CollectionVersion::load(path)
            .expect("Can't read collection version")
            .parse()
            .expect("Failed to parse stored collection version as semver");

        let app_version: Version = CollectionVersion::current()
            .parse()
            .expect("Failed to parse current collection version as semver");

        if stored_version > app_version {
            panic!("Collection version is greater than application version");
        }

        if stored_version != app_version {
            if Self::can_upgrade_storage(&stored_version, &app_version) {
                log::info!("Migrating collection {stored_version} -> {app_version}");
                CollectionVersion::save(path)
                    .unwrap_or_else(|err| panic!("Can't save collection version {}", err));
            } else {
                log::error!("Cannot upgrade version {stored_version} to {app_version}.");
                panic!("Cannot upgrade version {stored_version} to {app_version}. Try to use older version of Solvio first.");
            }
        }

        let config = CollectionConfig::load(path).unwrap_or_else(|err| {
            panic!(
                "Can't read collection config due to {}\nat {}",
                err,
                path.to_str().unwrap()
            )
        });

        let ring = HashRing::fair(HASH_RING_SHARD_SCALE);
        let mut shard_holder = ShardHolder::new(path, ring).expect("Can not create shard holder");

        let shared_config = Arc::new(RwLock::new(config.clone()));

        shard_holder
            .load_shards(
                path,
                &collection_id,
                shared_config.clone(),
                channel_service.clone(),
                on_replica_failure.clone(),
                this_peer_id,
            )
            .await;

        let locked_shard_holder = Arc::new(LockedShardHolder::new(shard_holder));

        Self {
            id: collection_id.clone(),
            shards_holder: locked_shard_holder,
            config: shared_config,
            before_drop_called: false,
            this_peer_id,
            path: path.to_owned(),
            snapshots_path: snapshots_path.to_owned(),
            channel_service,
            transfer_tasks: Mutex::new(TransferTasksPool::default()),
            request_shard_transfer_cb: request_shard_transfer.clone(),
            notify_peer_failure_cb: on_replica_failure,
            init_time: start_time.elapsed(),
        }
    }

    pub async fn set_shard_replica_state(
        &self,
        shard_id: ShardId,
        peer_id: PeerId,
        state: ReplicaState,
    ) -> CollectionResult<()> {
        let shard_holder = self.shards_holder.read().await;
        let replica_set =
            shard_holder
                .get_shard(&shard_id)
                .ok_or_else(|| CollectionError::NotFound {
                    what: format!("Shard {shard_id}"),
                })?;
        replica_set.set_replica_state(&peer_id, state)?;

        // Try to request shard transfer if replicas on the current peer are dead
        if state == ReplicaState::Dead && self.this_peer_id == peer_id {
            let transfer_from = replica_set
                .peers()
                .into_iter()
                .find(|(_, state)| state == &ReplicaState::Active)
                .map(|(peer_id, _)| peer_id);
            if let Some(transfer_from) = transfer_from {
                self.request_shard_transfer(ShardTransfer {
                    shard_id,
                    from: transfer_from,
                    to: self.this_peer_id,
                    sync: true,
                })
            } else {
                log::warn!("No alive replicas to recover shard {shard_id}");
            }
        }

        Ok(())
    }

    pub async fn contains_shard(&self, shard_id: ShardId) -> bool {
        let shard_holder_read = self.shards_holder.read().await;
        shard_holder_read.contains_shard(&shard_id)
    }

    /// Returns true if shard it explicitly local, false otherwise.
    pub async fn is_shard_local(&self, shard_id: &ShardId) -> Option<bool> {
        let shard_holder_read = self.shards_holder.read().await;
        if let Some(shard) = shard_holder_read.get_shard(shard_id) {
            Some(shard.is_local().await)
        } else {
            None
        }
    }

    pub async fn check_transfer_exists(&self, transfer_key: &ShardTransferKey) -> bool {
        let shard_holder_read = self.shards_holder.read().await;
        let transfers = shard_holder_read
            .shard_transfers
            .read()
            .iter()
            .any(|transfer| transfer_key.check(transfer));
        transfers
    }

    pub async fn get_outgoing_transfers(&self, current_peer_id: &PeerId) -> Vec<ShardTransfer> {
        let shard_holder = self.shards_holder.read().await;
        let transfers = shard_holder
            .shard_transfers
            .read()
            .iter()
            .filter(|transfer| transfer.from == *current_peer_id)
            .cloned()
            .collect();
        transfers
    }

    async fn send_shard<OF, OE>(&self, transfer: ShardTransfer, on_finish: OF, on_error: OE)
    where
        OF: Future<Output = ()> + Send + 'static,
        OE: Future<Output = ()> + Send + 'static,
    {
        let mut active_transfer_tasks = self.transfer_tasks.lock().await;
        let task_result = active_transfer_tasks.stop_if_exists(&transfer.key()).await;

        debug_assert_eq!(task_result, TaskResult::NotFound);

        let shard_holder = self.shards_holder.clone();
        let collection_id = self.id.clone();
        let channel_service = self.channel_service.clone();

        let transfer_task = spawn_transfer_task(
            shard_holder,
            transfer.clone(),
            collection_id,
            channel_service,
            on_finish,
            on_error,
        );

        active_transfer_tasks.add_task(&transfer, transfer_task);
    }

    pub async fn start_shard_transfer<T, F>(
        &self,
        shard_transfer: ShardTransfer,
        on_finish: T,
        on_error: F,
    ) -> CollectionResult<bool>
    where
        T: Future<Output = ()> + Send + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        let shard_id = shard_transfer.shard_id;
        let do_transfer = {
            let shards_holder = self.shards_holder.read().await;
            let _was_not_transferred =
                shards_holder.register_start_shard_transfer(shard_transfer.clone())?;
            let replica_set_opt = shards_holder.get_shard(&shard_id);

            // Check if current node owns the shard which should be transferred
            // and therefor able to transfer
            let replica_set = if let Some(replica_set) = replica_set_opt {
                replica_set
            } else {
                // Service error, because it means the validation was incorrect
                return Err(CollectionError::service_error(format!(
                    "Shard {shard_id} doesn't exist"
                )));
            };
            // Set learning replica state on all peers
            // This should disable queries to learning replica even if it was active
            replica_set.set_replica_state(&shard_transfer.to, ReplicaState::Partial)?;
            replica_set.is_local().await && replica_set.this_peer_id() == shard_transfer.from
        };
        if do_transfer {
            self.send_shard(shard_transfer, on_finish, on_error).await;
        }
        Ok(do_transfer)
    }

    /// Handles finishing of the shard transfer.
    ///
    /// Returns true if state was changed, false otherwise.
    pub async fn finish_shard_transfer(&self, transfer: ShardTransfer) -> CollectionResult<()> {
        let finish_was_registered = self
            .shards_holder
            .read()
            .await
            .register_finish_transfer(&transfer.key())?;
        let transfer_finished = self
            .transfer_tasks
            .lock()
            .await
            .stop_if_exists(&transfer.key())
            .await
            .is_finished();
        log::debug!("finish_was_registered: {}", finish_was_registered);
        log::debug!("transfer_finished: {}", transfer_finished);

        let shards_holder_guard = self.shards_holder.read().await;

        // Should happen on transfer side
        // Unwrap forward proxy into local shard, or replace it with remote shard
        // depending on the `sync` flag.
        if self.this_peer_id == transfer.from {
            let proxy_promoted = handle_transferred_shard_proxy(
                &shards_holder_guard,
                transfer.shard_id,
                transfer.to,
                transfer.sync,
            )
            .await?;
            log::debug!("proxy_promoted: {}", proxy_promoted);
        }

        // Should happen on receiving side
        // Promote partial shard to active shard
        if self.this_peer_id == transfer.to {
            let shard_promoted =
                finalize_partial_shard(&shards_holder_guard, transfer.shard_id).await?;
            log::debug!("shard_promoted: {}", shard_promoted);
        }

        // Should happen on a third-party side
        // Change direction of the remote shards or add a new remote shard
        if self.this_peer_id != transfer.from {
            let remote_shard_rerouted = change_remote_shard_route(
                &shards_holder_guard,
                transfer.shard_id,
                transfer.from,
                transfer.to,
                transfer.sync,
            )
            .await?;
            log::debug!("remote_shard_rerouted: {}", remote_shard_rerouted);
        }
        Ok(())
    }

    /// Handles abort of the transfer
    ///
    /// 1. Unregister the transfer
    /// 2. Stop transfer task
    /// 3. Unwrap the proxy
    /// 4. Remove temp shard
    pub async fn abort_shard_transfer(
        &self,
        transfer_key: ShardTransferKey,
    ) -> CollectionResult<()> {
        let _finish_was_registered = self
            .shards_holder
            .read()
            .await
            .register_finish_transfer(&transfer_key)?;
        let _transfer_finished = self
            .transfer_tasks
            .lock()
            .await
            .stop_if_exists(&transfer_key)
            .await
            .is_finished();

        let shard_holder_guard = self.shards_holder.read().await;

        let replica_set =
            if let Some(replica_set) = shard_holder_guard.get_shard(&transfer_key.shard_id) {
                replica_set
            } else {
                return Err(CollectionError::bad_request(format!(
                    "Shard {} doesn't exist",
                    transfer_key.shard_id
                )));
            };

        replica_set.remove_peer(transfer_key.to).await?;

        if self.this_peer_id == transfer_key.from {
            revert_proxy_shard_to_local(&shard_holder_guard, transfer_key.shard_id).await?;
        }

        Ok(())
    }

    /// Initiate local partial shard
    ///
    /// Drops existing temporary shards for `shard_id`.
    pub async fn initiate_local_partial_shard(&self, shard_id: ShardId) -> CollectionResult<()> {
        let shards_holder = self.shards_holder.read().await;
        let replica_set = match shards_holder.get_shard(&shard_id) {
            None => {
                return Err(CollectionError::service_error(format!(
                    "Shard {shard_id} doesn't exist, repartition is not supported yet"
                )))
            }
            Some(replica_set) => replica_set,
        };

        if !replica_set.has_local_shard().await {
            // create local shard
            let shard = LocalShard::build(
                shard_id,
                self.name(),
                &replica_set.shard_path,
                self.config.clone(),
            )
            .await?;

            replica_set
                .set_local(shard, Some(ReplicaState::Partial))
                .await?;
        } else {
            if !replica_set.is_local().await {
                // We have proxy or something, we need to unwrap it
                log::warn!("Unwrapping proxy shard {}", shard_id);
                replica_set.un_proxify_local().await?
            }
            replica_set.set_replica_state(&replica_set.this_peer_id(), ReplicaState::Partial)?;
        }
        Ok(())
    }

    /// Handle collection updates from peers.
    ///
    /// Shard transfer aware.
    pub async fn update_from_peer(
        &self,
        operation: CollectionUpdateOperations,
        shard_selection: ShardId,
        wait: bool,
    ) -> CollectionResult<UpdateResult> {
        let shard_holder_guard = self.shards_holder.read().await;

        let res = match shard_holder_guard.get_shard(&shard_selection) {
            None => None,
            Some(target_shard) => target_shard.update_local(operation.clone(), wait).await?,
        };

        if let Some(res) = res {
            Ok(res)
        } else {
            Err(CollectionError::service_error(format!(
                "No target shard {} found for update",
                shard_selection
            )))
        }
    }

    pub async fn update_from_client(
        &self,
        operation: CollectionUpdateOperations,
        wait: bool,
    ) -> CollectionResult<UpdateResult> {
        operation.validate()?;

        let mut results = {
            let shards_holder = self.shards_holder.read().await;
            let shard_to_op = shards_holder.split_by_shard(operation);

            if shard_to_op.is_empty() {
                return Err(CollectionError::bad_request(
                    "Empty update request".to_string(),
                ));
            }

            let shard_requests = shard_to_op
                .into_iter()
                .map(move |(shard, operation)| shard.update(operation, wait));
            join_all(shard_requests).await
        };

        let with_error = results
            .iter()
            .filter(|result| matches!(result, Err(_)))
            .count();

        // one request per shard
        let result_len = results.len();

        if with_error > 0 {
            let first_err = results
                .into_iter()
                .find(|result| matches!(result, Err(_)))
                .unwrap();
            // inconsistent if only a subset of the requests fail - one request per shard.
            if with_error < result_len {
                first_err.map_err(|err| {
                    // compute final status code based on the first error
                    // e.g. a partially successful batch update failing because of bad input is a client error
                    CollectionError::InconsistentShardFailure {
                        shards_total: result_len as u32, // report only the number of shards that took part in the update
                        shards_failed: with_error as u32,
                        first_err: Box::new(err),
                    }
                })
            } else {
                // all requests per shard failed - propagate first error (assume there are all the same)
                first_err
            }
        } else {
            // At least one result is always present.
            results.pop().unwrap()
        }
    }

    pub async fn recommend_by(
        &self,
        request: RecommendRequest,
        search_runtime_handle: &Handle,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<ScoredPoint>> {
        // `recommend_by` is a special case of recommend_by_batch with a single batch
        let request_batch = RecommendRequestBatch {
            searches: vec![request],
        };
        let results = self
            .recommend_batch_by(request_batch, search_runtime_handle, shard_selection)
            .await?;
        Ok(results.into_iter().next().unwrap())
    }

    pub async fn recommend_batch_by(
        &self,
        request_batch: RecommendRequestBatch,
        search_runtime_handle: &Handle,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<Vec<ScoredPoint>>> {
        // pack all reference vector ids
        let mut all_reference_vectors_ids = HashSet::new();
        for request in &request_batch.searches {
            if request.positive.is_empty() {
                return Err(CollectionError::BadRequest {
                    description: "At least one positive vector ID required".to_owned(),
                });
            }
            for point_id in request.positive.iter().chain(&request.negative) {
                all_reference_vectors_ids.insert(*point_id);
            }
        }

        // batch vector retrieval
        let all_vectors = self
            .retrieve(
                PointRequest {
                    ids: all_reference_vectors_ids.into_iter().collect(),
                    with_payload: Some(WithPayloadInterface::Bool(true)),
                    with_vector: true.into(),
                },
                shard_selection,
            )
            .await?;

        let mut searches = Vec::with_capacity(request_batch.searches.len());

        for request in request_batch.searches {
            let vector_name = match request.using {
                None => DEFAULT_VECTOR_NAME.to_owned(),
                Some(UsingVector::Name(name)) => name,
            };

            //let rec_vectors = rec.get
            let mut all_vectors_map = HashMap::new();

            for rec in all_vectors.iter() {
                let vector = rec.get_vector_by_name(&vector_name);
                if let Some(vector) = vector {
                    all_vectors_map.insert(rec.id, vector);
                } else {
                    return Err(CollectionError::BadRequest {
                        description: format!(
                            "Vector '{}' not found, expected one of {:?}",
                            vector_name,
                            rec.vector_names()
                        ),
                    });
                }
            }

            let reference_vectors_ids = request
                .positive
                .iter()
                .chain(&request.negative)
                .cloned()
                .collect_vec();

            for &point_id in &reference_vectors_ids {
                if !all_vectors_map.contains_key(&point_id) {
                    return Err(CollectionError::PointNotFound {
                        missed_point_id: point_id,
                    });
                }
            }

            let avg_positive = avg_vectors(
                request
                    .positive
                    .iter()
                    .map(|vid| *all_vectors_map.get(vid).unwrap()),
            );

            let search_vector = if request.negative.is_empty() {
                avg_positive
            } else {
                let avg_negative = avg_vectors(
                    request
                        .negative
                        .iter()
                        .map(|vid| *all_vectors_map.get(vid).unwrap()),
                );

                avg_positive
                    .iter()
                    .cloned()
                    .zip(avg_negative.iter().cloned())
                    .map(|(pos, neg)| pos + pos - neg)
                    .collect()
            };

            let search_request = SearchRequest {
                vector: NamedVector {
                    name: vector_name,
                    vector: search_vector,
                }
                .into(),
                filter: Some(Filter {
                    should: None,
                    must: request
                        .filter
                        .clone()
                        .map(|filter| vec![Condition::Filter(filter)]),
                    must_not: Some(vec![Condition::HasId(HasIdCondition {
                        has_id: reference_vectors_ids.iter().cloned().collect(),
                    })]),
                }),
                with_payload: request.with_payload.clone(),
                with_vector: request.with_vector,
                params: request.params,
                limit: request.limit,
                score_threshold: request.score_threshold,
                offset: request.offset,
            };
            searches.push(search_request)
        }

        let search_batch_request = SearchRequestBatch { searches };

        self.search_batch(search_batch_request, search_runtime_handle, shard_selection)
            .await
    }

    pub async fn search_batch(
        &self,
        request: SearchRequestBatch,
        search_runtime_handle: &Handle,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<Vec<ScoredPoint>>> {
        // A factor which determines if we need to use the 2-step search or not
        // Should be adjusted based on usage statistics.
        const PAYLOAD_TRANSFERS_FACTOR_THRESHOLD: usize = 10;

        let is_payload_required = request.searches.iter().all(|s| {
            s.with_payload
                .clone()
                .map(|p| p.is_required())
                .unwrap_or_default()
        });
        let with_vectors = request.searches.iter().all(|s| {
            s.with_vector
                .as_ref()
                .map(|wv| wv.is_some())
                .unwrap_or(false)
        });

        let metadata_required = is_payload_required || with_vectors;

        let sum_limits: usize = request.searches.iter().map(|s| s.limit).sum();
        let sum_offsets: usize = request.searches.iter().map(|s| s.offset).sum();

        // Number of records we need to retrieve to fill the search result.
        let require_transfers = self.shards_holder.read().await.len() * (sum_limits + sum_offsets);
        // Actually used number of records.
        let used_transfers = sum_limits;

        let is_required_transfer_large_enough =
            require_transfers > used_transfers * PAYLOAD_TRANSFERS_FACTOR_THRESHOLD;

        if metadata_required && is_required_transfer_large_enough {
            // If there is a significant offset, we need to retrieve the whole result
            // set without payload first and then retrieve the payload.
            // It is required to do this because the payload might be too large to send over the
            // network.
            let mut without_payload_requests = Vec::with_capacity(request.searches.len());
            for search in &request.searches {
                let mut without_payload_request = search.clone();
                without_payload_request.with_payload = None;
                without_payload_request.with_vector = None;
                without_payload_requests.push(without_payload_request);
            }
            let without_payload_batch = SearchRequestBatch {
                searches: without_payload_requests,
            };
            let without_payload_results = self
                ._search_batch(
                    without_payload_batch,
                    search_runtime_handle,
                    shard_selection,
                )
                .await?;
            let filled_results = without_payload_results
                .into_iter()
                .zip(request.clone().searches.into_iter())
                .map(|(without_payload_result, req)| {
                    self.fill_search_result_with_payload(
                        without_payload_result,
                        req.with_payload.clone(),
                        req.with_vector.unwrap_or_default(),
                        shard_selection,
                    )
                });
            try_join_all(filled_results).await
        } else {
            let result = self
                ._search_batch(request, search_runtime_handle, shard_selection)
                .await?;
            Ok(result)
        }
    }

    pub async fn _search_batch(
        &self,
        request: SearchRequestBatch,
        search_runtime_handle: &Handle,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<Vec<ScoredPoint>>> {
        let batch_size = request.searches.len();
        let request = Arc::new(request);

        // query all shards concurrently
        let mut all_searches_res = {
            let shard_holder = self.shards_holder.read().await;
            let target_shards = shard_holder.target_shard(shard_selection)?;
            let all_searches = target_shards
                .iter()
                .map(|shard| shard.search(request.clone(), search_runtime_handle));
            try_join_all(all_searches).await?
        };

        // merge results from shards in order
        let mut merged_results: Vec<Vec<ScoredPoint>> = vec![vec![]; batch_size];
        for shard_searches_results in all_searches_res.iter_mut() {
            for (index, shard_searches_result) in shard_searches_results.iter_mut().enumerate() {
                merged_results[index].append(shard_searches_result)
            }
        }
        let collection_params = self.config.read().await.params.clone();
        let top_results: Vec<_> = merged_results
            .into_iter()
            .zip(request.searches.iter())
            .map(|(res, request)| {
                let distance = collection_params
                    .get_vector_params(request.vector.get_name())?
                    .distance;
                let mut top_res = match distance.distance_order() {
                    Order::LargeBetter => {
                        peek_top_largest_iterable(res, request.limit + request.offset)
                    }
                    Order::SmallBetter => {
                        peek_top_smallest_iterable(res, request.limit + request.offset)
                    }
                };
                // Remove `offset` from top result only for client requests
                // to avoid applying `offset` twice in distributed mode.
                if shard_selection.is_none() && request.offset > 0 {
                    if top_res.len() >= request.offset {
                        // Panics if the end point > length of the vector.
                        top_res.drain(..request.offset);
                    } else {
                        top_res.clear()
                    }
                }
                Ok(top_res)
            })
            .collect::<CollectionResult<Vec<_>>>()?;

        Ok(top_results)
    }

    async fn fill_search_result_with_payload(
        &self,
        search_result: Vec<ScoredPoint>,
        with_payload: Option<WithPayloadInterface>,
        with_vector: WithVector,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<ScoredPoint>> {
        let retrieve_request = PointRequest {
            ids: search_result.iter().map(|x| x.id).collect(),
            with_payload,
            with_vector,
        };
        let retrieved_records = self.retrieve(retrieve_request, shard_selection).await?;
        let mut records_map: HashMap<ExtendedPointId, Record> = retrieved_records
            .into_iter()
            .map(|rec| (rec.id, rec))
            .collect();
        let enriched_result = search_result
            .into_iter()
            .filter_map(|mut scored_point| {
                // Points might get deleted between search and retrieve.
                // But it's not a problem, because we don't want to return deleted points.
                // So we just filter out them.
                records_map.remove(&scored_point.id).map(|record| {
                    scored_point.payload = record.payload;
                    scored_point.vector = record.vector;
                    scored_point
                })
            })
            .collect();
        Ok(enriched_result)
    }

    pub async fn search(
        &self,
        request: SearchRequest,
        search_runtime_handle: &Handle,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<ScoredPoint>> {
        // search is a special case of search_batch with a single batch
        let request_batch = SearchRequestBatch {
            searches: vec![request],
        };
        let results = self
            ._search_batch(request_batch, search_runtime_handle, shard_selection)
            .await?;
        Ok(results.into_iter().next().unwrap())
    }

    pub async fn scroll_by(
        &self,
        request: ScrollRequest,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<ScrollResult> {
        let default_request = ScrollRequest::default();

        let offset = request.offset;
        let limit = request
            .limit
            .unwrap_or_else(|| default_request.limit.unwrap());
        let with_payload_interface = request
            .with_payload
            .clone()
            .unwrap_or_else(|| default_request.with_payload.clone().unwrap());
        let with_vector = request.with_vector;

        if limit == 0 {
            return Err(CollectionError::BadRequest {
                description: "Limit cannot be 0".to_string(),
            });
        }

        // Needed to return next page offset.
        let limit = limit + 1;
        let retrieved_points: Vec<_> = {
            let shards_holder = self.shards_holder.read().await;
            let target_shards = shards_holder.target_shard(shard_selection)?;
            let scroll_futures = target_shards.into_iter().map(|shard| {
                shard.scroll_by(
                    offset,
                    limit,
                    &with_payload_interface,
                    &with_vector,
                    request.filter.as_ref(),
                )
            });

            try_join_all(scroll_futures).await?
        };
        let mut points: Vec<_> = retrieved_points
            .into_iter()
            .flatten()
            .sorted_by_key(|point| point.id)
            .take(limit)
            .collect();

        let next_page_offset = if points.len() < limit {
            // This was the last page
            None
        } else {
            // remove extra point, it would be a first point of the next page
            Some(points.pop().unwrap().id)
        };
        Ok(ScrollResult {
            points,
            next_page_offset,
        })
    }

    pub async fn count(
        &self,
        request: CountRequest,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<CountResult> {
        let request = Arc::new(request);

        let counts: Vec<_> = {
            let shards_holder = self.shards_holder.read().await;
            let target_shards = shards_holder.target_shard(shard_selection)?;
            let count_futures = target_shards
                .into_iter()
                .map(|shard| shard.count(request.clone()));
            try_join_all(count_futures).await?.into_iter().collect()
        };

        let total_count = counts.iter().map(|x| x.count).sum::<usize>();
        let aggregated_count = CountResult { count: total_count };
        Ok(aggregated_count)
    }

    pub async fn retrieve(
        &self,
        request: PointRequest,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<Record>> {
        let with_payload_interface = request
            .with_payload
            .as_ref()
            .unwrap_or(&WithPayloadInterface::Bool(false));
        let with_payload = WithPayload::from(with_payload_interface);
        let request = Arc::new(request);
        let all_shard_collection_results = {
            let shard_holder = self.shards_holder.read().await;
            let target_shards = shard_holder.target_shard(shard_selection)?;
            let retrieve_futures = target_shards
                .into_iter()
                .map(|shard| shard.retrieve(request.clone(), &with_payload, &request.with_vector));
            try_join_all(retrieve_futures).await?
        };
        let points = all_shard_collection_results.into_iter().flatten().collect();
        Ok(points)
    }

    pub async fn update_params_from_diff(
        &self,
        params_diff: CollectionParamsDiff,
    ) -> CollectionResult<()> {
        let mut config = self.config.write().await;
        config.params = params_diff.update(&config.params)?;
        Ok(())
    }

    pub fn request_shard_transfer(&self, shard_transfer: ShardTransfer) {
        self.request_shard_transfer_cb.deref()(shard_transfer)
    }

    /// Handle replica changes
    ///
    /// add and remove replicas from replica set
    pub async fn handle_replica_changes(
        &self,
        replica_changes: Vec<Change>,
    ) -> CollectionResult<()> {
        if replica_changes.is_empty() {
            return Ok(());
        }
        let read_shard_holder = self.shards_holder.read().await;

        for change in replica_changes {
            match change {
                Change::Remove(shard_id, peer_id) => {
                    let replica_set_opt = read_shard_holder.get_shard(&shard_id);
                    let replica_set = if let Some(replica_set) = replica_set_opt {
                        replica_set
                    } else {
                        return Err(CollectionError::BadRequest {
                            description: format!("Shard {} of {} not found", shard_id, self.name()),
                        });
                    };

                    let peers = replica_set.peers();

                    if !peers.contains_key(&peer_id) {
                        return Err(CollectionError::BadRequest {
                            description: format!(
                                "Peer {} has no replica of shard {}",
                                peer_id, shard_id
                            ),
                        });
                    }

                    if peers.len() == 1 {
                        return Err(CollectionError::BadRequest {
                            description: format!(
                                "Shard {} must have at least one replica",
                                shard_id
                            ),
                        });
                    }

                    replica_set.remove_peer(peer_id).await?;
                }
            }
        }
        Ok(())
    }

    /// Updates shard optimization params:
    /// - Saves new params on disk
    /// - Stops existing optimization loop
    /// - Runs new optimizers with new params
    pub async fn update_optimizer_params_from_diff(
        &self,
        optimizer_config_diff: OptimizersConfigDiff,
    ) -> CollectionResult<()> {
        {
            let mut config = self.config.write().await;
            config.optimizer_config =
                DiffConfig::update(optimizer_config_diff, &config.optimizer_config)?;
        }
        {
            let shard_holder = self.shards_holder.read().await;
            for replica_set in shard_holder.all_shards() {
                replica_set.on_optimizer_config_update().await?;
            }
        }
        self.config.read().await.save(&self.path)?;
        Ok(())
    }

    /// Updates shard optimization params:
    /// - Saves new params on disk
    /// - Stops existing optimization loop
    /// - Runs new optimizers with new params
    pub async fn update_optimizer_params(
        &self,
        optimizer_config: OptimizersConfig,
    ) -> CollectionResult<()> {
        {
            let mut config = self.config.write().await;
            config.optimizer_config = optimizer_config;
        }
        {
            let shard_holder = self.shards_holder.read().await;
            for replica_set in shard_holder.all_shards() {
                replica_set.on_optimizer_config_update().await?;
            }
        }
        self.config.read().await.save(&self.path)?;
        Ok(())
    }

    pub async fn info(&self, shard_selection: Option<ShardId>) -> CollectionResult<CollectionInfo> {
        let (all_shard_collection_results, mut info) = {
            let shards_holder = self.shards_holder.read().await;

            let target_shards = shards_holder.target_shard(shard_selection)?;

            let first_shard =
                *target_shards
                    .first()
                    .ok_or_else(|| CollectionError::ServiceError {
                        error: "There are no shards for selected collection".to_string(),
                    })?;

            let info = first_shard.info().await?;
            let info_futures = target_shards.into_iter().skip(1).map(|shard| shard.info());

            (try_join_all(info_futures).await?, info)
        };

        all_shard_collection_results
            .into_iter()
            .for_each(|mut shard_info| {
                info.status = max(info.status, shard_info.status);
                info.optimizer_status =
                    max(info.optimizer_status.clone(), shard_info.optimizer_status);
                info.vectors_count += shard_info.vectors_count;
                info.indexed_vectors_count += shard_info.indexed_vectors_count;
                info.points_count += shard_info.points_count;
                info.segments_count += shard_info.segments_count;
                info.payload_schema
                    .extend(shard_info.payload_schema.drain());
            });
        Ok(info)
    }

    pub async fn cluster_info(&self, peer_id: PeerId) -> CollectionResult<CollectionClusterInfo> {
        let shards_holder = self.shards_holder.read().await;
        let shard_count = shards_holder.len();
        let mut local_shards = Vec::new();
        let mut remote_shards = Vec::new();
        let mut shard_transfers = Vec::new();
        let count_request = Arc::new(CountRequest {
            filter: None,
            exact: true,
        });
        // extract shards info
        for (shard_id, replica_set) in shards_holder.get_shards() {
            let shard_id = *shard_id;
            let peers = replica_set.peers();

            if replica_set.has_local_shard().await {
                let state = peers
                    .get(&replica_set.this_peer_id())
                    .copied()
                    .unwrap_or(ReplicaState::Dead);
                let count_result = replica_set.count(count_request.clone()).await?;
                let points_count = count_result.count;
                local_shards.push(LocalShardInfo {
                    shard_id,
                    points_count,
                    state,
                })
            }
            for (peer_id, state) in replica_set.peers().into_iter() {
                if peer_id == replica_set.this_peer_id() {
                    continue;
                }
                remote_shards.push(RemoteShardInfo {
                    shard_id,
                    peer_id,
                    state,
                });
            }
        }
        // extract shard transfers info
        for shard_transfer in shards_holder.shard_transfers.read().iter() {
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

        // sort by shard_id
        local_shards.sort_by_key(|k| k.shard_id);
        remote_shards.sort_by_key(|k| k.shard_id);
        shard_transfers.sort_by_key(|k| k.shard_id);

        let info = CollectionClusterInfo {
            peer_id,
            shard_count,
            local_shards,
            remote_shards,
            shard_transfers,
        };
        Ok(info)
    }

    pub async fn before_drop(&mut self) {
        self.shards_holder.write().await.before_drop().await;
        self.before_drop_called = true
    }

    pub async fn state(&self) -> State {
        let shards_holder = self.shards_holder.read().await;
        let transfers = shards_holder.shard_transfers.read().clone();
        State {
            config: self.config.read().await.clone(),
            shards: shards_holder
                .get_shards()
                .map(|(shard_id, replicas)| {
                    let shard_info = ShardInfo {
                        replicas: replicas.peers(),
                    };
                    (*shard_id, shard_info)
                })
                .collect(),
            transfers,
        }
    }

    pub async fn apply_state(
        &self,
        state: State,
        this_peer_id: PeerId,
        abort_transfer: impl FnMut(ShardTransfer),
    ) -> CollectionResult<()> {
        state.apply(this_peer_id, self, abort_transfer).await
    }

    pub async fn get_telemetry_data(&self) -> CollectionTelemetry {
        let shards_telemetry = {
            let mut shards_telemetry = Vec::new();
            let shards_holder = self.shards_holder.read().await;
            for shard in shards_holder.all_shards() {
                shards_telemetry.push(shard.get_telemetry_data().await)
            }
            shards_telemetry
        };

        CollectionTelemetry {
            id: self.name(),
            init_time_ms: self.init_time.as_millis() as u64,
            config: self.config.read().await.clone(),
            shards: shards_telemetry,
        }
    }

    pub async fn list_snapshots(&self) -> CollectionResult<Vec<SnapshotDescription>> {
        list_snapshots_in_directory(&self.snapshots_path).await
    }

    pub async fn get_snapshot_path(&self, snapshot_name: &str) -> CollectionResult<PathBuf> {
        let snapshot_path = self.snapshots_path.join(snapshot_name);
        if !snapshot_path.exists() {
            return Err(CollectionError::NotFound {
                what: format!("Snapshot {}", snapshot_name),
            });
        }
        Ok(snapshot_path)
    }

    pub async fn create_snapshot(&self, temp_dir: &Path) -> CollectionResult<SnapshotDescription> {
        let snapshot_name = format!(
            "{}-{}.snapshot",
            self.name(),
            chrono::Utc::now().format("%Y-%m-%d-%H-%M-%S")
        );
        let snapshot_path = self.snapshots_path.join(&snapshot_name);

        let snapshot_path_tmp = snapshot_path.with_extension("tmp");

        let snapshot_path_with_tmp_extension = temp_dir.join(&snapshot_name).with_extension("tmp");
        let snapshot_path_with_arc_extension = temp_dir.join(snapshot_name).with_extension("arc");

        create_dir_all(&snapshot_path_with_tmp_extension).await?;

        {
            let shards_holder = self.shards_holder.read().await;
            // Create snapshot of each shard
            for (shard_id, replica_set) in shards_holder.get_shards() {
                let shard_snapshot_path =
                    versioned_shard_path(&snapshot_path_with_tmp_extension, *shard_id, 0);
                create_dir_all(&shard_snapshot_path).await?;
                replica_set.create_snapshot(&shard_snapshot_path).await?;
            }
        }

        CollectionVersion::save(&snapshot_path_with_tmp_extension)?;
        self.config
            .read()
            .await
            .save(&snapshot_path_with_tmp_extension)?;

        // have to use std here, cause TarBuilder is not async
        let file = std::fs::File::create(&snapshot_path_with_arc_extension)?;
        let mut builder = TarBuilder::new(file);
        // archive recursively collection directory `snapshot_path_with_arc_extension` into `snapshot_path`
        builder.append_dir_all(".", &snapshot_path_with_tmp_extension)?;
        builder.finish()?;

        // remove temporary snapshot directory
        remove_dir_all(&snapshot_path_with_tmp_extension).await?;

        // move snapshot to permanent location
        // We can't move right away, because snapshot folder can be on another mounting point.
        // We can't copy to the target location directly, cause copy is not atomic.
        copy(&snapshot_path_with_arc_extension, &snapshot_path_tmp).await?;
        rename(&snapshot_path_tmp, &snapshot_path).await?;
        remove_file(snapshot_path_with_arc_extension).await?;

        get_snapshot_description(&snapshot_path).await
    }

    pub fn restore_snapshot(snapshot_path: &Path, target_dir: &Path) -> CollectionResult<()> {
        // decompress archive
        let archive_file = std::fs::File::open(snapshot_path).unwrap();
        let mut ar = tar::Archive::new(archive_file);
        ar.unpack(target_dir)?;

        let config = CollectionConfig::load(target_dir)?;
        let configured_shards = config.params.shard_number.get();

        for shard_id in 0..configured_shards {
            let shard_path = versioned_shard_path(target_dir, shard_id, 0);
            let shard_config_opt = ShardConfig::load(&shard_path)?;
            if let Some(shard_config) = shard_config_opt {
                match shard_config.r#type {
                    shard_config::ShardType::Local => LocalShard::restore_snapshot(&shard_path)?,
                    shard_config::ShardType::Remote { .. } => {
                        RemoteShard::restore_snapshot(&shard_path)
                    }
                    shard_config::ShardType::Temporary => {}
                    shard_config::ShardType::ReplicaSet { .. } => {
                        ReplicaSetShard::restore_snapshot(&shard_path)?
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

    pub async fn suggest_shard_replica_changes(
        &self,
        new_repl_factor: NonZeroU32,
        _all_peers: HashSet<PeerId>,
    ) -> CollectionResult<HashSet<replica_set::Change>> {
        let changes: HashSet<Change> = HashSet::new();

        let shard_holder = self.shards_holder.read().await;
        let mut shard_to_peers: HashMap<ShardId, HashSet<PeerId>> = HashMap::new();

        for (shard_id, replica_set) in shard_holder.get_shards() {
            let peers = replica_set.peers();
            shard_to_peers.insert(*shard_id, peers.keys().copied().collect());
        }

        // ToDo: functions to use:
        // * suggest_transfer_source
        // * suggest_peer_to_add_replica
        // * suggest_peer_to_remove_replica

        for (_shard_id, replica_set) in shard_holder.get_shards() {
            let peers = replica_set.peers();
            let current_number_of_replicas = peers.len();
            let required_number_of_replicas = new_repl_factor.get() as usize;
            if current_number_of_replicas < required_number_of_replicas {
                // We need to add replicas
                // ToDo add replicas
                log::warn!("Automatic replica addition is not implemented yet");
            }
            if current_number_of_replicas > required_number_of_replicas {
                // We need to remove replicas
                // ToDo remove replicas
                log::warn!("Automatic replica removal is not implemented yet");
            }
        }

        Ok(changes)
    }

    pub async fn remove_shards_at_peer(&self, peer_id: PeerId) -> CollectionResult<()> {
        let shard_holder = self.shards_holder.read().await;

        for (_shard_id, replica_set) in shard_holder.get_shards() {
            replica_set.remove_peer(peer_id).await?;
        }
        Ok(())
    }
}

impl Drop for Collection {
    fn drop(&mut self) {
        if !self.before_drop_called {
            // Panic is used to get fast feedback in unit and integration tests
            // in cases where `before_drop` was not added.
            if cfg!(test) {
                panic!("Collection `before_drop` was not called.")
            } else {
                log::error!("Collection `before_drop` was not called.")
            }
        }
    }
}

fn avg_vectors<'a>(
    vectors: impl Iterator<Item = &'a Vec<VectorElementType>>,
) -> Vec<VectorElementType> {
    let mut count: usize = 0;
    let mut avg_vector: Vec<VectorElementType> = vec![];
    for vector in vectors {
        count += 1;
        for i in 0..vector.len() {
            if i >= avg_vector.len() {
                avg_vector.push(vector[i])
            } else {
                avg_vector[i] += vector[i];
            }
        }
    }

    for item in &mut avg_vector {
        *item /= count as VectorElementType;
    }

    avg_vector
}
