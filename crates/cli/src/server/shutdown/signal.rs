//! OS-signal handling — block until the user/orchestrator asks us to stop.

use anyhow::Result;

pub fn wait() -> Result<()> {
    // TODO: install handlers for SIGINT + SIGTERM (e.g. `tokio::signal`)
    // and block until either fires.
    println!("[shutdown::signal] (stub) would block on Ctrl-C / SIGTERM");
    Ok(())
}
