mod acme;
mod cache;
mod check;
mod cli;
mod core;
mod dns;
mod firewall;
mod http;
mod management;
mod metrics;
mod persist;
mod rate_limiting;
mod retry;
mod routes;
mod snapshot;
#[cfg(any(test, feature = "bench"))]
pub mod test_utils;
mod tls_verify;

#[cfg(feature = "tls")]
mod configuration;
#[cfg(feature = "tls")]
mod tls;

pub use crate::core::main;
pub use crate::routes::{status, Health, RootKey};
