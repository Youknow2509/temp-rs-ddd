//! WebSocket server runtime.

use anyhow::Result;
use infrastructure::state::AppState;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub fn start(app_state: &Arc<AppState>) -> Result<JoinHandle<()>> {
    let _state = Arc::clone(app_state);
    // TODO: bind WS handler to `_state.config.interfaces.ws_server`,
    // mount handlers with `_state` as shared state.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
