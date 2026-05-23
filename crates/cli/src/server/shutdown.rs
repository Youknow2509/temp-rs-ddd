//! Phase 4: wait for a shutdown signal, then drain and close all resources.

pub mod signal;

use anyhow::Result;
use tracing::info;

use super::wiring::Wired;

/// Block until SIGINT / SIGTERM, then close all connection pools.
pub fn drain(wired: Wired) -> Result<()> {
    signal::wait()?;

    info!("shutdown signal received — draining connections");

    wired.bootstrap.connections.pg_pool.close();
    info!("PostgreSQL pool closed");

    Ok(())
}
