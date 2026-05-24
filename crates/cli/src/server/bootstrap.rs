//! Phase 1: load config, init telemetry, open connection pools.

pub mod config;
pub mod connections;
pub mod telemetry;

use std::sync::Arc;

use self::telemetry::TelemetryGuard;
use anyhow::Result;
use interface::state::AppState;
use tracing::info;

pub struct Bootstrap {
    pub app_state: Arc<AppState>,
    pub telemetry_guard: TelemetryGuard,
}

pub async fn init() -> Result<Bootstrap> {
    let config = config::load()?;
    let telemetry_guard = self::telemetry::init(&config)?;
    info!("Config loaded, telemetry initialized");

    let connections = connections::init(&config).await?;
    info!("Connection pools initialized");

    Ok(Bootstrap {
        app_state: Arc::new(AppState::new(connections, config)),
        telemetry_guard,
    })
}
