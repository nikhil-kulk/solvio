use actix_web_httpauth::headers::authorization::{Bearer, Scheme};
use tonic::Status;
use tower::filter::{FilterLayer, Predicate};

use crate::common::auth::AuthKeys;
use crate::common::strings::ct_eq;

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
pub struct ApiKeyMiddleware {
    auth_keys: AuthKeys,
}

impl ApiKeyMiddleware {
    pub fn new_layer(auth_keys: AuthKeys) -> FilterLayer<Self> {
        FilterLayer::new(Self { auth_keys })
    }
}

impl Predicate<tonic::codegen::http::Request<tonic::transport::Body>> for ApiKeyMiddleware {
    type Request = tonic::codegen::http::Request<tonic::transport::Body>;

    fn check(&mut self, request: Self::Request) -> Result<Self::Request, tower::BoxError> {
        // Grab API key from request
        let key =
            // Request header
            request.headers().get("api-key").and_then(|key| key.to_str().ok()).map(|key| key.to_string())
                // Fall back to authentication header with bearer token
                .or_else(|| {
                    request.headers().get("authorization")
                        .and_then(|auth| {
                            Bearer::parse(auth).ok().map(|bearer| bearer.token().into())
                        })
                });

        if let Some(key) = key {
            let is_allowed = self.auth_keys.can_write(&key)
                || (is_read_only(&request) && self.auth_keys.can_read(&key));
            if is_allowed {
                return Ok(request);
            }
        }

        Err(Box::new(Status::permission_denied("Invalid api-key")))
    }
}

fn is_read_only<R>(req: &tonic::codegen::http::Request<R>) -> bool {
    let uri_path = req.uri().path();
    READ_ONLY_RPC_PATHS
        .iter()
        .any(|ro_uri_path| ct_eq(uri_path, ro_uri_path))
}
