mod api;
pub mod proto;

use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;

use tokio::runtime;
use tonic::{transport::Server, Request, Response, Status};

use proto::collections_server::CollectionsServer;
use proto::solvio_server::{Solvio, SolvioServer};
use proto::{HealthCheckReply, HealthCheckRequest};
use storage::content_manager::toc::TableOfContent;

use crate::common::models::VersionInfo;
use crate::settings::Settings;
use crate::tonic::api::collections_api::CollectionsService;

#[derive(Default)]
pub struct SolvioService {}

impl From<VersionInfo> for HealthCheckReply {
    fn from(info: VersionInfo) -> Self {
        HealthCheckReply {
            title: info.title,
            version: info.version,
        }
    }
}

#[tonic::async_trait]
impl Solvio for SolvioService {
    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckReply>, Status> {
        Ok(Response::new(VersionInfo::default().into()))
    }
}

pub fn init(toc: Arc<TableOfContent>, settings: Settings) -> std::io::Result<()> {
    let tonic_runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()?;
    tonic_runtime
        .block_on(async {
            let socket = SocketAddr::from((
                settings.service.host.parse::<IpAddr>().unwrap(),
                settings.service.grpc_port,
            ));

            let service = SolvioService::default();
            let collections_service = CollectionsService::new(toc.clone());

            info!("solvio grpc listening on {}", settings.service.grpc_port);

            Server::builder()
                .add_service(SolvioServer::new(service))
                .add_service(CollectionsServer::new(collections_service))
                .serve(socket)
                .await
        })
        .unwrap();
    Ok(())
}
