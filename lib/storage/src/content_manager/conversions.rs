use crate::content_manager::collection_meta_ops::{
    default_shard_number, AliasOperations, ChangeAliasesOperation, CollectionMetaOperations,
    CreateAlias, CreateAliasOperation, CreateCollection, CreateCollectionOperation, DeleteAlias,
    DeleteAliasOperation, DeleteCollectionOperation, RenameAlias, RenameAliasOperation,
    UpdateCollection, UpdateCollectionOperation,
};
use crate::content_manager::errors::StorageError;
use tonic::Status;

pub fn error_to_status(error: StorageError) -> tonic::Status {
    let error_code = match &error {
        StorageError::BadInput { .. } => tonic::Code::InvalidArgument,
        StorageError::NotFound { .. } => tonic::Code::NotFound,
        StorageError::ServiceError { .. } => tonic::Code::Internal,
        StorageError::BadRequest { .. } => tonic::Code::InvalidArgument,
    };
    return tonic::Status::new(error_code, format!("{}", error));
}

impl TryFrom<api::grpc::solvio::CreateCollection> for CollectionMetaOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::CreateCollection) -> Result<Self, Self::Error> {
        let internal_distance = match api::grpc::solvio::Distance::from_i32(value.distance) {
            Some(api::grpc::solvio::Distance::Cosine) => segment::types::Distance::Cosine,
            Some(api::grpc::solvio::Distance::Euclid) => segment::types::Distance::Euclid,
            Some(api::grpc::solvio::Distance::Dot) => segment::types::Distance::Dot,
            Some(_) => return Err(Status::failed_precondition("Unknown distance")),
            _ => return Err(Status::failed_precondition("Bad value of distance field!")),
        };

        Ok(Self::CreateCollection(CreateCollectionOperation {
            collection_name: value.collection_name,
            create_collection: CreateCollection {
                vector_size: value.vector_size as usize,
                distance: internal_distance,
                hnsw_config: value.hnsw_config.map(|v| v.into()),
                wal_config: value.wal_config.map(|v| v.into()),
                optimizers_config: value.optimizers_config.map(|v| v.into()),
                shard_number: value.shard_number.unwrap_or_else(default_shard_number),
                on_disk_payload: value.on_disk_payload,
            },
        }))
    }
}

impl TryFrom<api::grpc::solvio::UpdateCollection> for CollectionMetaOperations {
    type Error = Status;

    fn try_from(value: api::grpc::solvio::UpdateCollection) -> Result<Self, Self::Error> {
        Ok(Self::UpdateCollection(UpdateCollectionOperation {
            collection_name: value.collection_name,
            update_collection: UpdateCollection {
                optimizers_config: value.optimizers_config.map(|v| v.into()),
            },
        }))
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
