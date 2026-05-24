//! HTTP server runtime.

use std::sync::Arc;

use anyhow::Result;
use interface::state::AppState;
use tokio::task::JoinHandle;

pub fn start(app_state: &Arc<AppState>) -> Result<JoinHandle<()>> {
    let _state = Arc::clone(app_state);
    // TODO: bind axum/hyper to `_state.config.interfaces.http_server`,
    // mount routes with `_state` as shared state.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
