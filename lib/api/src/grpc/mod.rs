#![allow(deprecated)]

pub mod conversions;
pub mod models;
#[allow(clippy::all)]
#[rustfmt::skip] // tonic uses `prettyplease` to format its output
pub mod solvio;
pub mod transport_channel_pool;

pub const fn api_crate_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
