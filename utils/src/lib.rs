pub mod auth;
pub mod config;
pub mod data;
pub mod requests;
pub mod updates;

#[cfg(all(feature = "server", feature = "client"))]
compile_error!("Features 'server' and 'client' cannot be enabled at the same time.");
