//! gRPC server runtime.

use anyhow::Result;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<()> {
    // TODO: bind tonic to `wired.bootstrap.config.interfaces.grpc_server`,
    // register services backed by `wired.use_cases`.
    println!("[run::grpc] (stub) gRPC server would start here");
    Ok(())
}
