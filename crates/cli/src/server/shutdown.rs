//! Phase 3: wait for a shutdown signal, then drain all resources in safe order.

pub mod signal;

use anyhow::Result;
use tracing::info;

use super::bootstrap::Bootstrap;
use super::run::RunHandles;

/// Await SIGINT / SIGTERM, then drain all resources in dependency order:
///
///   1. Interfaces  — stop accepting new requests.
///   2. Connections — last Arc<Connections> dropped here, pools drain.
///   3. Telemetry   — flush last so all shutdown logs are captured.
pub async fn drain(bootstrap: Bootstrap, handles: RunHandles) -> Result<()> {
    signal::wait().await?;
    info!("shutdown signal received — draining connections");

    // 1. Stop all inbound interfaces so no new requests enter the system.
    handles.stop_all();
    info!("interfaces stopped");

    // 2. Drop Arc<AppState>. When the last clone is released (after all
    //    interface tasks finish), Connections closes via Drop.
    let Bootstrap { app_state, telemetry_guard } = bootstrap;
    drop(app_state);
    info!("connections dropped");

    // 3. Flush telemetry last so every log line above is captured.
    drop(telemetry_guard);

    Ok(())
}
