//! Phase 1: load config, init logger, open connection pools.

pub mod config;
pub mod connections;
pub mod logger;

use anyhow::Result;

use domain::config::SystemConfig;

use self::connections::Connections;

/// Output of the bootstrap phase. Consumed by `wiring` to build the
/// repositories / services / use cases on top of these primitives.
#[derive(Debug)]
#[allow(dead_code)] // fields consumed once real adapters land
pub struct Bootstrap {
    pub config: SystemConfig,
    pub connections: Connections,
}

pub fn init() -> Result<Bootstrap> {
    let config = config::load()?;
    logger::init(&config)?;
    let connections = connections::init(&config)?;
    Ok(Bootstrap { config, connections })
}
