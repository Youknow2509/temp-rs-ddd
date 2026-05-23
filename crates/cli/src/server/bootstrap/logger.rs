//! Initialise the global logging / tracing subscriber.

use anyhow::Result;

use domain::config::SystemConfig;

pub fn init(_config: &SystemConfig) -> Result<()> {
    // TODO: wire `tracing-subscriber` (or `log` + env_logger) using a
    // logging section added to `SystemConfig` (level, json/pretty, file sink).
    Ok(())
}
