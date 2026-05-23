//! gRPC server runtime.

use anyhow::Result;
use tokio::task::JoinHandle;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<JoinHandle<()>> {
    // TODO: bind tonic to `wired.config.interfaces.grpc_server`,
    // register services backed by `wired.use_cases`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
