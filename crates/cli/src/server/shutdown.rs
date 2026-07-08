//! Phase 3: wait for a shutdown signal, then drain all resources in safe order.

pub mod signal;

use super::bootstrap::Bootstrap;
use super::run::RunHandles;
use anyhow::Result;
use tracing::info;

/// Await SIGINT / SIGTERM, then drain all resources in dependency order:
///
///   1. Interfaces  — stop accepting new requests.
///   2. Connections — last Arc<Connections> dropped here, pools drain.
///   3. Telemetry   — flush last so all shutdown logs are captured.
pub async fn drain(bootstrap: Bootstrap, handles: RunHandles) -> Result<()> {
    signal::wait().await?;
    info!("shutdown signal received — draining connections");

    let mut handles = handles;

    // 1. Stop all inbound interfaces so no new requests enter the system.
    handles.request_stop();
    info!("interfaces stopped");

    // 2. Wait for all tasks to complete so their AppState clones
    //    are definitely released before we drop shared state.
    handles.wait_all().await;
    info!("all servers drained");

    // 3. Drop Arc<AppState>. When the last clone is released (after all
    //    interface tasks finish), Connections closes via Drop.
    let Bootstrap {
        app_state,
        telemetry_guard,
    } = bootstrap;
    drop(app_state);
    info!("connections dropped");

    // 4. Stop telemetry explicitly so the metrics exporter logs its shutdown.
    let mut telemetry_guard = telemetry_guard;
    telemetry_guard.stop();
    info!("telemetry stopped");

    Ok(())
}
