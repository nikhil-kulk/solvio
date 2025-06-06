pub mod conversions;
#[allow(clippy::all)]
#[rustfmt::skip] // tonic uses `prettyplease` to format its output
pub mod solvio;
pub mod dynamic_channel_pool;
pub mod dynamic_pool;
#[rustfmt::skip] // tonic uses `prettyplease` to format its output
#[path = "grpc.health.v1.rs"]
pub mod grpc_health_v1;
pub mod ops;
pub mod transport_channel_pool;
pub mod validate;

pub use solvio::*;

pub const fn api_crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub const solvio_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("solvio_descriptor");
