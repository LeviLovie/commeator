#[cfg(all(feature = "server", feature = "client"))]
compile_error!("Features 'server' and 'client' cannot be enabled at the same time.");

pub mod auth;
pub mod config;
pub mod data;
pub mod requests;
pub mod updates;

pub trait LogError {
    fn log_error(self) -> Self;
}

impl<T> LogError for anyhow::Result<T> {
    fn log_error(self) -> Self {
        if let Err(ref e) = self {
            tracing::error!("Error: {:?}", e);
        }
        self
    }
}

#[cfg(target_arch = "wasm32")]
#[cfg(feature = "client")]
pub async fn sleep_ms(ms: u64) {
    gloo_timers::future::TimeoutFuture::new(ms as u32).await;
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(feature = "client")]
pub async fn sleep_ms(ms: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
}
