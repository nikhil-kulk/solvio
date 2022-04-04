mod api;

use crate::tonic::api::collections_api::CollectionsService;
use crate::tonic::api::points_api::PointsService;
use crate::tonic::api::points_internal_api::PointsInternalService;
use ::api::grpc::models::VersionInfo;
use ::api::grpc::solvio::collections_server::CollectionsServer;
use ::api::grpc::solvio::points_internal_server::PointsInternalServer;
use ::api::grpc::solvio::points_server::PointsServer;
use ::api::grpc::solvio::solvio_server::{Solvio, SolvioServer};
use ::api::grpc::solvio::{HealthCheckReply, HealthCheckRequest};
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use storage::content_manager::toc::TableOfContent;
use tokio::{runtime, signal};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct SolvioService {}

#[tonic::async_trait]
impl Solvio for SolvioService {
    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckReply>, Status> {
        Ok(Response::new(VersionInfo::default().into()))
    }
}

pub fn init(toc: Arc<TableOfContent>, host: String, grpc_port: u16) -> std::io::Result<()> {
    let tonic_runtime = runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .thread_name_fn(|| {
            static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
            let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
            format!("tonic-{}", id)
        })
        .build()?;
    tonic_runtime
        .block_on(async {
            let socket = SocketAddr::from((host.parse::<IpAddr>().unwrap(), grpc_port));

            let service = SolvioService::default();
            let collections_service = CollectionsService::new(toc.clone());
            let points_service = PointsService::new(toc.clone());
            let points_internal_service = PointsInternalService::new(toc.clone());

            log::info!("Solvio gRPC listening on {}", grpc_port);

            Server::builder()
                .add_service(SolvioServer::new(service))
                .add_service(CollectionsServer::new(collections_service))
                .add_service(PointsServer::new(points_service))
                .add_service(PointsInternalServer::new(points_internal_service)) // TODO serve from different port
                .serve_with_shutdown(socket, async {
                    signal::ctrl_c().await.unwrap();
                    log::info!("Stopping gRPC");
                })
                .await
        })
        .unwrap();
    Ok(())
}
