//! gRPC server runtime.

use anyhow::Result;
use infrastructure::state::AppState;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub fn start(app_state: &Arc<AppState>) -> Result<JoinHandle<()>> {
    let _state = Arc::clone(app_state);
    // TODO: bind tonic to `_state.config.interfaces.grpc_server`,
    // register services backed by `_state`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
