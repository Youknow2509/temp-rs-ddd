//! Phase 4: wait for a shutdown signal, then drain.

pub mod signal;

use anyhow::Result;

pub fn wait() -> Result<()> {
    signal::wait()?;
    // TODO: drain in-flight requests, close pools, flush logs / metrics.
    Ok(())
}
