use std::collections::BTreeMap;

use collection::operations::types::VectorsConfig;
use tonic::Status;

use crate::content_manager::collection_meta_ops::{
    AliasOperations, ChangeAliasesOperation, CollectionMetaOperations, CreateAlias,
    CreateAliasOperation, CreateCollection, CreateCollectionOperation, DeleteAlias,
    DeleteAliasOperation, DeleteCollectionOperation, InitFrom, RenameAlias, RenameAliasOperation,
    UpdateCollection, UpdateCollectionOperation,
};
use crate::content_manager::errors::StorageError;

pub fn error_to_status(error: StorageError) -> tonic::Status {
    let error_code = match &error {
        StorageError::BadInput { .. } => tonic::Code::InvalidArgument,
        StorageError::NotFound { .. } => tonic::Code::NotFound,
        StorageError::ServiceError { .. } => tonic::Code::Internal,
        StorageError::BadRequest { .. } => tonic::Code::InvalidArgument,
        StorageError::Locked { .. } => tonic::Code::FailedPrecondition,
    };
    tonic::Status::new(error_code, format!("{}", error))
}

impl TryFrom<api::grpc::solvio::CreateCollection> for CollectionMetaOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::CreateCollection) -> Result<Self, Self::Error> {
        Ok(Self::CreateCollection(CreateCollectionOperation::new(
            value.collection_name,
            CreateCollection {
                vectors: match value.vectors_config {
                    Some(vectors) => match vectors.config {
                        None => return Err(Status::invalid_argument("vectors config is required")),
                        Some(params) => match params {
                            api::grpc::solvio::vectors_config::Config::Params(vector_params) => {
                                VectorsConfig::Single(vector_params.try_into()?)
                            }
                            api::grpc::solvio::vectors_config::Config::ParamsMap(
                                vectors_params,
                            ) => {
                                let mut params_map = BTreeMap::new();
                                for (name, params) in vectors_params.map {
                                    params_map.insert(name, params.try_into()?);
                                }
                                VectorsConfig::Multi(params_map)
                            }
                        },
                    },
                    None => return Err(Status::invalid_argument("vectors config is required")),
                },
                hnsw_config: value.hnsw_config.map(|v| v.into()),
                wal_config: value.wal_config.map(|v| v.into()),
                optimizers_config: value.optimizers_config.map(|v| v.into()),
                shard_number: value.shard_number,
                on_disk_payload: value.on_disk_payload,
                replication_factor: value.replication_factor,
                write_consistency_factor: value.write_consistency_factor,
                init_from: value
                    .init_from_collection
                    .map(|v| InitFrom { collection: v }),
            },
        )))
    }
}

impl TryFrom<api::grpc::solvio::UpdateCollection> for CollectionMetaOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::UpdateCollection) -> Result<Self, Self::Error> {
        Ok(Self::UpdateCollection(UpdateCollectionOperation::new(
            value.collection_name,
            UpdateCollection {
                optimizers_config: value.optimizers_config.map(Into::into),
                params: value.params.map(TryInto::try_into).transpose()?,
            },
        )))
    }
}

impl TryFrom<api::grpc::solvio::DeleteCollection> for CollectionMetaOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::DeleteCollection) -> Result<Self, Self::Error> {
        Ok(Self::DeleteCollection(DeleteCollectionOperation(
            value.collection_name,
        )))
    }
}

impl From<api::grpc::solvio::CreateAlias> for AliasOperations {
    fn from(value: api::grpc::solvio::CreateAlias) -> Self {
        Self::CreateAlias(CreateAliasOperation {
            create_alias: CreateAlias {
                collection_name: value.collection_name,
                alias_name: value.alias_name,
            },
        })
    }
}

impl From<api::grpc::solvio::DeleteAlias> for AliasOperations {
    fn from(value: api::grpc::solvio::DeleteAlias) -> Self {
        Self::DeleteAlias(DeleteAliasOperation {
            delete_alias: DeleteAlias {
                alias_name: value.alias_name,
            },
        })
    }
}

impl From<api::grpc::solvio::RenameAlias> for AliasOperations {
    fn from(value: api::grpc::solvio::RenameAlias) -> Self {
        Self::RenameAlias(RenameAliasOperation {
            rename_alias: RenameAlias {
                old_alias_name: value.old_alias_name,
                new_alias_name: value.new_alias_name,
            },
        })
    }
}

impl TryFrom<api::grpc::solvio::AliasOperations> for AliasOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::AliasOperations) -> Result<Self, Self::Error> {
        match value.action {
            Some(api::grpc::solvio::alias_operations::Action::CreateAlias(create)) => {
                Ok(create.into())
            }
            Some(api::grpc::solvio::alias_operations::Action::DeleteAlias(delete)) => {
                Ok(delete.into())
            }
            Some(api::grpc::solvio::alias_operations::Action::RenameAlias(rename)) => {
                Ok(rename.into())
            }
            _ => Err(Status::invalid_argument("Malformed AliasOperation type")),
        }
    }
}

impl TryFrom<api::grpc::solvio::ChangeAliases> for CollectionMetaOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::ChangeAliases) -> Result<Self, Self::Error> {
        let actions: Vec<AliasOperations> = value
            .actions
            .into_iter()
            .map(|a| a.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Self::ChangeAliases(ChangeAliasesOperation { actions }))
    }
}
