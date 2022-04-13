use crate::tonic::api::collections_common::get;
use api::grpc::solvio::collections_internal_server::CollectionsInternal;
use api::grpc::solvio::{GetCollectionInfoRequestInternal, GetCollectionInfoResponse};
use std::sync::Arc;
use storage::content_manager::toc::TableOfContent;
use tonic::{Request, Response, Status};

pub struct CollectionsInternalService {
    toc: Arc<TableOfContent>,
}

impl CollectionsInternalService {
    pub fn new(toc: Arc<TableOfContent>) -> Self {
        Self { toc }
    }
}

#[tonic::async_trait]
impl CollectionsInternal for CollectionsInternalService {
    async fn get(
        &self,
        request: Request<GetCollectionInfoRequestInternal>,
    ) -> Result<Response<GetCollectionInfoResponse>, Status> {
        let GetCollectionInfoRequestInternal {
            get_collection_info_request,
            shard_id,
        } = request.into_inner();

        let get_collection_info_request = get_collection_info_request
            .ok_or_else(|| Status::invalid_argument("GetCollectionInfoRequest is missing"))?;

        get(
            self.toc.as_ref(),
            get_collection_info_request,
            Some(shard_id),
        )
        .await
    }
}
