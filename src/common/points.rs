use std::time::Duration;

use collection::operations::consistency_params::ReadConsistency;
use collection::operations::payload_ops::{
    DeletePayload, DeletePayloadOp, PayloadOps, SetPayload, SetPayloadOp,
};
use collection::operations::point_ops::{
    FilterSelector, PointIdsList, PointInsertOperations, PointOperations, PointsSelector,
    WriteOrdering,
};
use collection::operations::types::{
    CoreSearchRequestBatch, CountRequest, CountResult, DiscoverRequest, DiscoverRequestBatch,
    GroupsResult, PointRequest, RecommendGroupsRequest, Record, ScrollRequest, ScrollResult,
    SearchGroupsRequest, SearchRequest, SearchRequestBatch, UpdateResult,
};
use collection::operations::vector_ops::{
    DeleteVectors, UpdateVectors, UpdateVectorsOp, VectorOperations,
};
use collection::operations::{CollectionUpdateOperations, CreateIndex, FieldIndexOperations};
use collection::shards::shard::ShardId;
use schemars::JsonSchema;
use segment::types::{PayloadFieldSchema, PayloadKeyType, ScoredPoint};
use serde::{Deserialize, Serialize};
use storage::content_manager::collection_meta_ops::{
    CollectionMetaOperations, CreatePayloadIndex, DropPayloadIndex,
};
use storage::content_manager::errors::StorageError;
use storage::content_manager::shard_key_selection::ShardKeySelectorInternal;
use storage::content_manager::toc::TableOfContent;
use storage::dispatcher::Dispatcher;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate)]
pub struct CreateFieldIndex {
    pub field_name: PayloadKeyType,
    #[serde(alias = "field_type")]
    pub field_schema: Option<PayloadFieldSchema>,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpsertOperation {
    #[validate]
    upsert: PointInsertOperations,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct DeleteOperation {
    #[validate]
    delete: PointsSelector,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct SetPayloadOperation {
    #[validate]
    set_payload: SetPayload,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct OverwritePayloadOperation {
    #[validate]
    overwrite_payload: SetPayload,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct DeletePayloadOperation {
    #[validate]
    delete_payload: DeletePayload,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct ClearPayloadOperation {
    #[validate]
    clear_payload: PointsSelector,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpdateVectorsOperation {
    #[validate]
    update_vectors: UpdateVectors,
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct DeleteVectorsOperation {
    #[validate]
    delete_vectors: DeleteVectors,
}

#[derive(Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum UpdateOperation {
    Upsert(UpsertOperation),
    Delete(DeleteOperation),
    SetPayload(SetPayloadOperation),
    OverwritePayload(OverwritePayloadOperation),
    DeletePayload(DeletePayloadOperation),
    ClearPayload(ClearPayloadOperation),
    UpdateVectors(UpdateVectorsOperation),
    DeleteVectors(DeleteVectorsOperation),
}

#[derive(Deserialize, Serialize, JsonSchema, Validate)]
pub struct UpdateOperations {
    pub operations: Vec<UpdateOperation>,
}

impl Validate for UpdateOperation {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        match self {
            UpdateOperation::Upsert(op) => op.validate(),
            UpdateOperation::Delete(op) => op.validate(),
            UpdateOperation::SetPayload(op) => op.validate(),
            UpdateOperation::OverwritePayload(op) => op.validate(),
            UpdateOperation::DeletePayload(op) => op.validate(),
            UpdateOperation::ClearPayload(op) => op.validate(),
            UpdateOperation::UpdateVectors(op) => op.validate(),
            UpdateOperation::DeleteVectors(op) => op.validate(),
        }
    }
}

pub async fn do_upsert_points(
    toc: &TableOfContent,
    collection_name: &str,
    operation: PointInsertOperations,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let (shard_key, operation) = operation.decompose();
    let collection_operation =
        CollectionUpdateOperations::PointOperation(PointOperations::UpsertPoints(operation));
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key.into(),
    )
    .await
}

pub async fn do_delete_points(
    toc: &TableOfContent,
    collection_name: &str,
    points: PointsSelector,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let (point_operation, shard_key_selector) = match points {
        PointsSelector::PointIdsSelector(PointIdsList { points, shard_key }) => (
            PointOperations::DeletePoints { ids: points },
            ShardKeySelectorInternal::from(shard_key),
        ),
        PointsSelector::FilterSelector(FilterSelector { filter, shard_key }) => (
            PointOperations::DeletePointsByFilter(filter),
            ShardKeySelectorInternal::from(shard_key),
        ),
    };
    let collection_operation = CollectionUpdateOperations::PointOperation(point_operation);
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key_selector,
    )
    .await
}

pub async fn do_update_vectors(
    toc: &TableOfContent,
    collection_name: &str,
    operation: UpdateVectors,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let UpdateVectors { points, shard_key } = operation;

    let shard_key_selector = ShardKeySelectorInternal::from(shard_key);

    let collection_operation = CollectionUpdateOperations::VectorOperation(
        VectorOperations::UpdateVectors(UpdateVectorsOp { points }),
    );
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key_selector,
    )
    .await
}

pub async fn do_delete_vectors(
    toc: &TableOfContent,
    collection_name: &str,
    operation: DeleteVectors,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let DeleteVectors {
        vector,
        filter,
        points,
        shard_key,
    } = operation;

    let shard_key_selector = ShardKeySelectorInternal::from(shard_key);

    let vector_names: Vec<_> = vector.into_iter().collect();

    let mut result = None;

    if let Some(filter) = filter {
        let vectors_operation =
            VectorOperations::DeleteVectorsByFilter(filter, vector_names.clone());
        let collection_operation = CollectionUpdateOperations::VectorOperation(vectors_operation);
        result = Some(
            toc.update(
                collection_name,
                collection_operation,
                shard_selection,
                wait,
                ordering,
                shard_key_selector.clone(),
            )
            .await?,
        );
    }

    if let Some(points) = points {
        let vectors_operation = VectorOperations::DeleteVectors(points.into(), vector_names);
        let collection_operation = CollectionUpdateOperations::VectorOperation(vectors_operation);
        result = Some(
            toc.update(
                collection_name,
                collection_operation,
                shard_selection,
                wait,
                ordering,
                shard_key_selector,
            )
            .await?,
        );
    }

    result.ok_or_else(|| StorageError::bad_request("No filter or points provided"))
}

pub async fn do_set_payload(
    toc: &TableOfContent,
    collection_name: &str,
    operation: SetPayload,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let SetPayload {
        points,
        payload,
        filter,
        shard_key,
    } = operation;

    let collection_operation =
        CollectionUpdateOperations::PayloadOperation(PayloadOps::SetPayload(SetPayloadOp {
            payload,
            points,
            filter,
        }));
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key.into(),
    )
    .await
}

pub async fn do_overwrite_payload(
    toc: &TableOfContent,
    collection_name: &str,
    operation: SetPayload,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let SetPayload {
        points,
        payload,
        filter,
        shard_key,
    } = operation;

    let collection_operation =
        CollectionUpdateOperations::PayloadOperation(PayloadOps::OverwritePayload(SetPayloadOp {
            payload,
            points,
            filter,
        }));
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key.into(),
    )
    .await
}

pub async fn do_delete_payload(
    toc: &TableOfContent,
    collection_name: &str,
    operation: DeletePayload,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let DeletePayload {
        keys,
        points,
        filter,
        shard_key,
    } = operation;

    let collection_operation =
        CollectionUpdateOperations::PayloadOperation(PayloadOps::DeletePayload(DeletePayloadOp {
            keys,
            points,
            filter,
        }));
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key.into(),
    )
    .await
}

pub async fn do_clear_payload(
    toc: &TableOfContent,
    collection_name: &str,
    points: PointsSelector,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let (point_operation, shard_key_selector) = match points {
        PointsSelector::PointIdsSelector(PointIdsList { points, shard_key }) => (
            PayloadOps::ClearPayload { points },
            ShardKeySelectorInternal::from(shard_key),
        ),
        PointsSelector::FilterSelector(FilterSelector { filter, shard_key }) => (
            PayloadOps::ClearPayloadByFilter(filter),
            ShardKeySelectorInternal::from(shard_key),
        ),
    };

    let collection_operation = CollectionUpdateOperations::PayloadOperation(point_operation);
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        shard_key_selector,
    )
    .await
}

pub async fn do_batch_update_points(
    toc: &TableOfContent,
    collection_name: &str,
    operations: Vec<UpdateOperation>,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<Vec<UpdateResult>, StorageError> {
    let mut results = Vec::with_capacity(operations.len());
    for operation in operations {
        let result = match operation {
            UpdateOperation::Upsert(operation) => {
                do_upsert_points(
                    toc,
                    collection_name,
                    operation.upsert,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::Delete(operation) => {
                do_delete_points(
                    toc,
                    collection_name,
                    operation.delete,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::SetPayload(operation) => {
                do_set_payload(
                    toc,
                    collection_name,
                    operation.set_payload,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::OverwritePayload(operation) => {
                do_overwrite_payload(
                    toc,
                    collection_name,
                    operation.overwrite_payload,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::DeletePayload(operation) => {
                do_delete_payload(
                    toc,
                    collection_name,
                    operation.delete_payload,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::ClearPayload(operation) => {
                do_clear_payload(
                    toc,
                    collection_name,
                    operation.clear_payload,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::UpdateVectors(operation) => {
                do_update_vectors(
                    toc,
                    collection_name,
                    operation.update_vectors,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
            UpdateOperation::DeleteVectors(operation) => {
                do_delete_vectors(
                    toc,
                    collection_name,
                    operation.delete_vectors,
                    shard_selection,
                    wait,
                    ordering,
                )
                .await
            }
        }?;
        results.push(result);
    }
    Ok(results)
}

pub async fn do_create_index_internal(
    toc: &TableOfContent,
    collection_name: &str,
    field_name: PayloadKeyType,
    field_schema: Option<PayloadFieldSchema>,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let collection_operation = CollectionUpdateOperations::FieldIndexOperation(
        FieldIndexOperations::CreateIndex(CreateIndex {
            field_name,
            field_schema,
        }),
    );

    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        ShardKeySelectorInternal::All,
    )
    .await
}

pub async fn do_create_index(
    dispatcher: &Dispatcher,
    collection_name: &str,
    operation: CreateFieldIndex,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let Some(field_schema) = operation.field_schema else {
        return Err(StorageError::bad_request(
            "Can't auto-detect field type, please specify `field_schema` in the request",
        ));
    };

    let consensus_op = CollectionMetaOperations::CreatePayloadIndex(CreatePayloadIndex {
        collection_name: collection_name.to_string(),
        field_name: operation.field_name.clone(),
        field_schema: field_schema.clone(),
    });

    // Default consensus timeout will be used
    let wait_timeout = None; // ToDo: make it configurable

    dispatcher
        .submit_collection_meta_op(consensus_op, wait_timeout)
        .await?;

    // This function is required as long as we want to maintain interface compatibility
    // for `wait` parameter and return type.
    // The idea is to migrate from the point-like interface to consensus-like interface in the next few versions

    do_create_index_internal(
        dispatcher.toc(),
        collection_name,
        operation.field_name,
        Some(field_schema),
        shard_selection,
        wait,
        ordering,
    )
    .await
}

pub async fn do_delete_index_internal(
    toc: &TableOfContent,
    collection_name: &str,
    index_name: String,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let collection_operation = CollectionUpdateOperations::FieldIndexOperation(
        FieldIndexOperations::DeleteIndex(index_name),
    );
    toc.update(
        collection_name,
        collection_operation,
        shard_selection,
        wait,
        ordering,
        ShardKeySelectorInternal::All,
    )
    .await
}

pub async fn do_delete_index(
    dispatcher: &Dispatcher,
    collection_name: &str,
    index_name: String,
    shard_selection: Option<ShardId>,
    wait: bool,
    ordering: WriteOrdering,
) -> Result<UpdateResult, StorageError> {
    let consensus_op = CollectionMetaOperations::DropPayloadIndex(DropPayloadIndex {
        collection_name: collection_name.to_string(),
        field_name: index_name.clone(),
    });

    // Default consensus timeout will be used
    let wait_timeout = None; // ToDo: make it configurable

    dispatcher
        .submit_collection_meta_op(consensus_op, wait_timeout)
        .await?;

    do_delete_index_internal(
        dispatcher.toc(),
        collection_name,
        index_name,
        shard_selection,
        wait,
        ordering,
    )
    .await
}

pub async fn do_search_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: SearchRequest,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
    timeout: Option<Duration>,
) -> Result<Vec<ScoredPoint>, StorageError> {
    toc.search(
        collection_name,
        request,
        read_consistency,
        shard_selection,
        timeout,
    )
    .await
}

pub async fn do_search_batch_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: SearchRequestBatch,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
    timeout: Option<Duration>,
) -> Result<Vec<Vec<ScoredPoint>>, StorageError> {
    toc.search_batch(
        collection_name,
        request,
        read_consistency,
        shard_selection,
        timeout,
    )
    .await
}

pub async fn do_core_search_batch_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: CoreSearchRequestBatch,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
    timeout: Option<Duration>,
) -> Result<Vec<Vec<ScoredPoint>>, StorageError> {
    toc.core_search_batch(
        collection_name,
        request,
        read_consistency,
        shard_selection,
        timeout,
    )
    .await
}

pub async fn do_search_point_groups(
    toc: &TableOfContent,
    collection_name: &str,
    request: SearchGroupsRequest,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
    timeout: Option<Duration>,
) -> Result<GroupsResult, StorageError> {
    toc.group(
        collection_name,
        request.into(),
        read_consistency,
        shard_selection,
        timeout,
    )
    .await
}

pub async fn do_recommend_point_groups(
    toc: &TableOfContent,
    collection_name: &str,
    request: RecommendGroupsRequest,
    read_consistency: Option<ReadConsistency>,
    timeout: Option<Duration>,
) -> Result<GroupsResult, StorageError> {
    toc.group(
        collection_name,
        request.into(),
        read_consistency,
        None,
        timeout,
    )
    .await
}

pub async fn do_discover_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: DiscoverRequest,
    read_consistency: Option<ReadConsistency>,
    timeout: Option<Duration>,
) -> Result<Vec<ScoredPoint>, StorageError> {
    toc.discover(collection_name, request, read_consistency, timeout)
        .await
}

pub async fn do_discover_batch_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: DiscoverRequestBatch,
    read_consistency: Option<ReadConsistency>,
    timeout: Option<Duration>,
) -> Result<Vec<Vec<ScoredPoint>>, StorageError> {
    toc.discover_batch(collection_name, request, read_consistency, timeout)
        .await
}

pub async fn do_count_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: CountRequest,
    shard_selection: Option<ShardId>,
) -> Result<CountResult, StorageError> {
    toc.count(collection_name, request, shard_selection).await
}

pub async fn do_get_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: PointRequest,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
) -> Result<Vec<Record>, StorageError> {
    toc.retrieve(collection_name, request, read_consistency, shard_selection)
        .await
}

pub async fn do_scroll_points(
    toc: &TableOfContent,
    collection_name: &str,
    request: ScrollRequest,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
) -> Result<ScrollResult, StorageError> {
    toc.scroll(collection_name, request, read_consistency, shard_selection)
        .await
}
