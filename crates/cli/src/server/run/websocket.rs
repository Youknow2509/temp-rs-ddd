//! WebSocket server runtime.

use anyhow::Result;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<()> {
    // TODO: bind WS handler to `wired.bootstrap.config.interfaces.ws_server`.
    println!("[run::websocket] (stub) WebSocket server would start here");
    Ok(())
}
