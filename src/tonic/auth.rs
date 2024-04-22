use std::sync::Arc;
use std::task::{Context, Poll};

use futures::future::BoxFuture;
use rbac::jwt::Claims;
use tonic::body::BoxBody;
use tonic::Status;
use tower::{Layer, Service};

use crate::common::auth::AuthKeys;
use crate::common::strings::ct_eq;

type Request = tonic::codegen::http::Request<tonic::transport::Body>;
type Response = tonic::codegen::http::Response<BoxBody>;

const READ_ONLY_RPC_PATHS: [&str; 14] = [
    "/solvio.Collections/CollectionExists",
    "/solvio.Collections/List",
    "/solvio.Collections/Get",
    "/solvio.Points/Scroll",
    "/solvio.Points/Get",
    "/solvio.Points/Count",
    "/solvio.Points/Search",
    "/solvio.Points/SearchGroups",
    "/solvio.Points/SearchBatch",
    "/solvio.Points/Recommend",
    "/solvio.Points/RecommendGroups",
    "/solvio.Points/RecommendBatch",
    "/solvio.Points/Discover",
    "/solvio.Points/DiscoverBatch",
];

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    auth_keys: Arc<AuthKeys>,
    service: S,
}

async fn check(auth_keys: Arc<AuthKeys>, mut req: Request) -> Result<Request, Status> {
    let claims = auth_keys
        .validate_request(
            |key| req.headers().get(key).and_then(|val| val.to_str().ok()),
            is_read_only(&req),
        )
        .await
        .map_err(Status::permission_denied)?;

    if let Some(claims) = claims {
        let _previous = req.extensions_mut().insert::<Claims>(claims);
        debug_assert!(
            _previous.is_none(),
            "Previous claims should not exist in the request"
        );
    }

    Ok(req)
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, S::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let auth_keys = self.auth_keys.clone();
        let mut service = self.service.clone();
        Box::pin(async move {
            match check(auth_keys, request).await {
                Ok(req) => service.call(req).await,
                Err(e) => Ok(e.to_http()),
            }
        })
    }
}

#[derive(Clone)]
pub struct AuthLayer {
    auth_keys: Arc<AuthKeys>,
}

impl AuthLayer {
    pub fn new(auth_keys: AuthKeys) -> Self {
        Self {
            auth_keys: Arc::new(auth_keys),
        }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, service: S) -> Self::Service {
        Self::Service {
            auth_keys: self.auth_keys.clone(),
            service,
        }
    }
}

pub fn extract_claims<R>(req: &mut tonic::Request<R>) -> Option<Claims> {
    req.extensions_mut().remove::<Claims>()
}

fn is_read_only<R>(req: &tonic::codegen::http::Request<R>) -> bool {
    let uri_path = req.uri().path();
    READ_ONLY_RPC_PATHS
        .iter()
        .any(|ro_uri_path| ct_eq(uri_path, ro_uri_path))
}
