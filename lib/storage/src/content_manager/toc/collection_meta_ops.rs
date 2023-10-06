use std::collections::HashSet;
use std::path::Path;

use collection::collection_state;
use collection::shards::collection_shard_distribution::CollectionShardDistribution;
use collection::shards::transfer::shard_transfer;
use collection::shards::CollectionId;
use uuid::Uuid;

use super::TableOfContent;
use crate::content_manager::collection_meta_ops::*;
use crate::content_manager::collections_ops::Checker as _;
use crate::content_manager::consensus_ops::ConsensusOperations;
use crate::content_manager::errors::StorageError;

impl TableOfContent {
    pub(super) fn perform_collection_meta_op_sync(
        &self,
        operation: CollectionMetaOperations,
    ) -> Result<bool, StorageError> {
        self.general_runtime
            .block_on(self.perform_collection_meta_op(operation))
    }

    pub async fn perform_collection_meta_op(
        &self,
        operation: CollectionMetaOperations,
    ) -> Result<bool, StorageError> {
        match operation {
            CollectionMetaOperations::CreateCollection(mut operation) => {
                log::debug!("Creating collection {}", operation.collection_name);
                let distribution = match operation.take_distribution() {
                    None => CollectionShardDistribution::all_local(
                        operation.create_collection.shard_number,
                        self.this_peer_id,
                    ),
                    Some(distribution) => distribution.into(),
                };
                self.create_collection(
                    &operation.collection_name,
                    operation.create_collection,
                    distribution,
                )
                .await
            }
            CollectionMetaOperations::UpdateCollection(operation) => {
                log::debug!("Updating collection {}", operation.collection_name);
                self.update_collection(operation).await
            }
            CollectionMetaOperations::DeleteCollection(operation) => {
                log::debug!("Deleting collection {}", operation.0);
                self.delete_collection(&operation.0).await
            }
            CollectionMetaOperations::ChangeAliases(operation) => {
                log::debug!("Changing aliases");
                self.update_aliases(operation).await
            }
            CollectionMetaOperations::TransferShard(collection, operation) => {
                log::debug!("Transfer shard {:?} of {}", operation, collection);

                self.handle_transfer(collection, operation)
                    .await
                    .map(|()| true)
            }
            CollectionMetaOperations::SetShardReplicaState(operation) => {
                log::debug!("Set shard replica state {:?}", operation);
                self.set_shard_replica_state(operation).await.map(|()| true)
            }
            CollectionMetaOperations::Nop { .. } => Ok(true),
        }
    }

    async fn update_collection(
        &self,
        mut operation: UpdateCollectionOperation,
    ) -> Result<bool, StorageError> {
        let replica_changes = operation.take_shard_replica_changes();
        let UpdateCollection {
            vectors,
            hnsw_config,
            params,
            optimizers_config,
            quantization_config,
        } = operation.update_collection;
        let collection = self.get_collection(&operation.collection_name).await?;
        let mut recreate_optimizers = false;

        if let Some(diff) = optimizers_config {
            collection.update_optimizer_params_from_diff(diff).await?;
            recreate_optimizers = true;
        }
        if let Some(diff) = params {
            collection.update_params_from_diff(diff).await?;
            recreate_optimizers = true;
        }
        if let Some(diff) = hnsw_config {
            collection.update_hnsw_config_from_diff(diff).await?;
            recreate_optimizers = true;
        }
        if let Some(diff) = vectors {
            collection.update_vectors_from_diff(&diff).await?;
            recreate_optimizers = true;
        }
        if let Some(diff) = quantization_config {
            collection
                .update_quantization_config_from_diff(diff)
                .await?;
            recreate_optimizers = true;
        }
        if let Some(changes) = replica_changes {
            collection.handle_replica_changes(changes).await?;
        }

        // Recreate optimizers
        if recreate_optimizers {
            collection.recreate_optimizers_blocking().await?;
        }
        Ok(true)
    }

    pub(super) async fn delete_collection(
        &self,
        collection_name: &str,
    ) -> Result<bool, StorageError> {
        if let Some(removed) = self.collections.write().await.remove(collection_name) {
            self.alias_persistence
                .write()
                .await
                .remove_collection(collection_name)?;

            let path = self.get_collection_path(collection_name);
            drop(removed);

            // Move collection to ".deleted" folder to prevent accidental reuse
            let uuid = Uuid::new_v4().to_string();
            let removed_collections_path =
                Path::new(&self.storage_config.storage_path).join(".deleted");
            tokio::fs::create_dir_all(&removed_collections_path).await?;
            let deleted_path = removed_collections_path
                .join(collection_name)
                .with_extension(uuid);
            tokio::fs::rename(path, &deleted_path).await?;

            // At this point collection is removed from memory and moved to ".deleted" folder.
            // Next time we load service the collection will not appear in the list of collections.
            // We can take our time to delete the collection from disk.
            tokio::spawn(async move {
                if let Err(error) = tokio::fs::remove_dir_all(&deleted_path).await {
                    log::error!(
                        "Can't delete collection {} from disk. Error: {}",
                        deleted_path.display(),
                        error
                    );
                }
            });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// performs several alias changes in an atomic fashion
    async fn update_aliases(
        &self,
        operation: ChangeAliasesOperation,
    ) -> Result<bool, StorageError> {
        // Lock all collections for alias changes
        // Prevent search on partially switched collections
        let collection_lock = self.collections.write().await;
        let mut alias_lock = self.alias_persistence.write().await;
        for action in operation.actions {
            match action {
                AliasOperations::CreateAlias(CreateAliasOperation {
                    create_alias:
                        CreateAlias {
                            collection_name,
                            alias_name,
                        },
                }) => {
                    collection_lock
                        .validate_collection_exists(&collection_name)
                        .await?;
                    collection_lock
                        .validate_collection_not_exists(&alias_name)
                        .await?;

                    alias_lock.insert(alias_name, collection_name)?;
                }
                AliasOperations::DeleteAlias(DeleteAliasOperation {
                    delete_alias: DeleteAlias { alias_name },
                }) => {
                    alias_lock.remove(&alias_name)?;
                }
                AliasOperations::RenameAlias(RenameAliasOperation {
                    rename_alias:
                        RenameAlias {
                            old_alias_name,
                            new_alias_name,
                        },
                }) => {
                    alias_lock.rename_alias(&old_alias_name, new_alias_name)?;
                }
            };
        }
        Ok(true)
    }

    async fn handle_transfer(
        &self,
        collection_id: CollectionId,
        transfer_operation: ShardTransferOperations,
    ) -> Result<(), StorageError> {
        let collection = self.get_collection(&collection_id).await?;
        let proposal_sender = if let Some(proposal_sender) = self.consensus_proposal_sender.clone()
        {
            proposal_sender
        } else {
            return Err(StorageError::service_error(
                "Can't handle transfer, this is a single node deployment",
            ));
        };

        match transfer_operation {
            ShardTransferOperations::Start(transfer) => {
                let collection_state::State {
                    config: _,
                    shards,
                    transfers,
                } = collection.state().await;
                let all_peers: HashSet<_> = self
                    .channel_service
                    .id_to_address
                    .read()
                    .keys()
                    .cloned()
                    .collect();
                let shard_state = shards.get(&transfer.shard_id).map(|info| &info.replicas);

                // Valid transfer:
                // All peers: 123, 321, 111, 222, 333
                // Peers: shard_id=1 - [{123: Active}]
                // Transfer: {123 -> 321}, shard_id=1

                // Invalid transfer:
                // All peers: 123, 321, 111, 222, 333
                // Peers: shard_id=1 - [{123: Active}]
                // Transfer: {321 -> 123}, shard_id=1

                shard_transfer::validate_transfer(&transfer, &all_peers, shard_state, &transfers)?;

                let collection_id_clone = collection_id.clone();
                let transfer_clone = transfer.clone();

                let on_finish_sender = proposal_sender.clone();
                let on_finish = async move {
                    let operation =
                        ConsensusOperations::finish_transfer(collection_id_clone, transfer_clone);

                    if let Err(error) = on_finish_sender.send(operation) {
                        log::error!("Can't report transfer progress to consensus: {}", error)
                    };
                };

                let collection_id_clone = collection_id.clone();
                let transfer_clone = transfer.clone();

                let on_failure = async move {
                    if let Err(error) = proposal_sender.send(ConsensusOperations::abort_transfer(
                        collection_id_clone,
                        transfer_clone,
                        "transmission failed",
                    )) {
                        log::error!("Can't report transfer progress to consensus: {}", error)
                    };
                };

                collection
                    .start_shard_transfer(transfer, on_finish, on_failure)
                    .await?;
            }
            ShardTransferOperations::Finish(transfer) => {
                // Validate transfer exists to prevent double handling
                shard_transfer::validate_transfer_exists(
                    &transfer.key(),
                    &collection.state().await.transfers,
                )?;
                collection.finish_shard_transfer(transfer).await?;
            }
            ShardTransferOperations::Abort { transfer, reason } => {
                // Validate transfer exists to prevent double handling
                shard_transfer::validate_transfer_exists(
                    &transfer,
                    &collection.state().await.transfers,
                )?;
                log::warn!("Aborting shard transfer: {reason}");
                collection.abort_shard_transfer(transfer).await?;
            }
        };
        Ok(())
    }

    async fn set_shard_replica_state(
        &self,
        operation: SetShardReplicaState,
    ) -> Result<(), StorageError> {
        self.get_collection(&operation.collection_name)
            .await?
            .set_shard_replica_state(
                operation.shard_id,
                operation.peer_id,
                operation.state,
                operation.from_state,
            )
            .await?;
        Ok(())
    }
}
