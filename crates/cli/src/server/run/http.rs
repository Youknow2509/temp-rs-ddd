//! HTTP server runtime.

use anyhow::Result;
use tokio::task::JoinHandle;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<JoinHandle<()>> {
    // TODO: bind axum/hyper to `wired.config.interfaces.http_server`,
    // mount routes that call into `wired.use_cases`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
