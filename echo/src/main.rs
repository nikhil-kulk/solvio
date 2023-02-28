use std::{
    collections::HashMap,
    fmt,
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use clap::Parser as _;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let opts = Opts::parse();

    let router = axum::Router::new()
        .route("/", axum::routing::post(query))
        .with_state(GrpcClientsCache::default());

    let http = axum::Server::bind(&opts.http)
        .serve(router.into_make_service());

    let grpc = tonic::transport::Server::builder()
        .add_service(rpc_service_server::RpcServiceServer::new(RpcServer))
        .serve(opts.grpc);

    let (http_res, grpc_res) = futures::future::join(http, grpc).await;

    if let Err(err) = &http_res {
        log::error!("HTTP server failed: {err:#}");
    }

    if let Err(err) = &grpc_res {
        log::error!("gRPC server failed: {err:#}");
    }

    http_res.map_err(Into::into).and(grpc_res.map_err(Into::into))
}


#[derive(Copy, Clone, Debug, clap::Parser)]
struct Opts {
    #[arg(long, default_value = "127.0.0.1:8080")]
    http: SocketAddr,

    #[arg(long, default_value = "127.0.0.1:8081")]
    grpc: SocketAddr,
}


async fn query(
    axum::extract::State(clients): axum::extract::State<GrpcClientsCache>,
    axum::extract::Json(params): axum::extract::Json<Params>,
) -> Result<axum::response::Json<Vec<SocketAddr>>, Error> {
    log::info!("POST /");

    let mut resp = Vec::new();

    for node in params.nodes {
        let echo = clients.get_or_connect(node).await?.lock().await.query(()).await?;
        resp.push(echo.into_inner());
    }

    Ok(axum::response::Json(resp))
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct Params {
    nodes: Vec<SocketAddr>,
}

#[derive(Clone, Debug, Default)]
struct GrpcClientsCache {
    cache: Arc<tokio::sync::RwLock<GrpcClientsCacheInner>>,
}

impl GrpcClientsCache {
    pub async fn get_or_connect(
        &self,
        addr: impl Into<SocketAddr>,
    ) -> anyhow::Result<SharedRpcClient> {
        let addr = addr.into();

        if let Some(client) = self.cache.read().await.clients.get(&addr) {
            return Ok(client.clone());
        }

        let endpoint = format!("http://{addr}");
        let endpoint = tonic::transport::channel::Endpoint::from_shared(endpoint)?
            .timeout(Duration::from_millis(200))
            .connect_timeout(Duration::from_millis(200));

        let client = rpc_service_client::RpcServiceClient::connect(endpoint).await?;
        let client = Arc::new(tokio::sync::Mutex::new(client));

        self.cache.write().await.clients.insert(addr, client.clone());

        Ok(client)
    }
}

#[derive(Debug, Default)]
struct GrpcClientsCacheInner {
    clients: HashMap<SocketAddr, SharedRpcClient>,
}

#[derive(Debug)]
struct Error(anyhow::Error);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}

impl<T: Into<anyhow::Error>> From<T> for Error {
    fn from(err: T) -> Self {
        Self(err.into())
    }
}


#[derive(Copy, Clone, Debug)]
struct RpcServer;

#[tonic::async_trait]
impl rpc_service_server::RpcService for RpcServer {
    async fn query(
        &self,
        request: tonic::Request<()>,
    ) -> Result<tonic::Response<SocketAddr>, tonic::Status> {
        Ok(tonic::Response::new(request.remote_addr().unwrap_or(([0, 0, 0, 0], 0).into())))
    }
}


type SharedRpcClient = Arc<tokio::sync::Mutex<RpcClient>>;
type RpcClient = rpc_service_client::RpcServiceClient<tonic::transport::Channel>;


#[tonic_rpc::tonic_rpc(json)]
trait RpcService {
    fn query() -> SocketAddr;
}
