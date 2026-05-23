//! Phase 1: load config, init telemetry, open connection pools.

pub mod config;
pub mod connections;
pub mod telemetry;

use self::connections::Connections;
use self::telemetry::TelemetryGuard;
use anyhow::Result;
use domain::config::SystemConfig;
use tracing::info;

/// Output of the bootstrap phase. Consumed by `wiring` to build the
/// repositories / services / use cases on top of these primitives.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Bootstrap {
    pub config: SystemConfig,
    pub connections: Connections,
    pub telemetry_guard: TelemetryGuard,
}

pub async fn init() -> Result<Bootstrap> {
    let config = config::load()?;
    let telemetry_guard = self::telemetry::init(&config)?;
    info!("Config loaded, telemetry initialized");

    let connections = connections::init(&config).await?;
    info!("Connection pools initialized");

    Ok(Bootstrap {
        config,
        connections,
        telemetry_guard,
    })
}
