//! gRPC server runtime.

use std::sync::Arc;

use anyhow::Result;
use interface::state::AppState;
use tokio::task::JoinHandle;

pub fn start(app_state: &Arc<AppState>) -> Result<JoinHandle<()>> {
    let _state = Arc::clone(app_state);
    // TODO: bind tonic to `_state.config.interfaces.grpc_server`,
    // register services backed by `_state`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
