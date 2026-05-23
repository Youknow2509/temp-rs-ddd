//! WebSocket server runtime.

use anyhow::Result;
use tokio::task::JoinHandle;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<JoinHandle<()>> {
    // TODO: bind WS handler to `wired.config.interfaces.ws_server`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
