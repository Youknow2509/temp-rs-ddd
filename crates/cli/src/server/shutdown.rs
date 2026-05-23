//! Phase 4: wait for a shutdown signal, then drain all resources in safe order.

pub mod signal;

use anyhow::Result;
use tracing::info;

use super::run::RunHandles;
use super::wiring::Wired;

/// Await SIGINT / SIGTERM, then drain all resources in dependency order:
///
///   1. Interfaces  — stop accepting new requests.
///   2. DDD layers  — no new I/O after this point; Redis closed here.
///   3. Connections — drain shared pools after layers release them.
///   4. Telemetry   — flush last so all shutdown logs are captured.
pub async fn drain(wired: Wired, handles: RunHandles) -> Result<()> {
    signal::wait().await?;
    info!("shutdown signal received — draining connections");

    // 1. Stop all inbound interfaces so no new requests enter the system.
    handles.stop_all();
    info!("interfaces stopped");

    // 2. Drop DDD layers — no new database / cache calls from here on.
    //    Dropping repositories also drops RedisCache → RedisPool (close via Drop).
    drop(wired.use_cases);
    drop(wired.services);
    drop(wired.repositories);
    info!("DDD layers stopped; Redis pool closed");

    // 3. Close shared connection pools (only refs remaining are in Wired).
    wired.pg_pool.close();
    info!("PostgreSQL pool closed");

    drop(wired.scylla_session);
    info!("ScyllaDB session closed");

    drop(wired.s3_client);
    info!("S3 client closed");

    drop(wired.kafka_client);
    info!("Kafka client closed");

    drop(wired.grpc_clients);
    info!("gRPC clients closed");

    // 4. Flush telemetry last so every log line above is captured.
    drop(wired.telemetry_guard);

    Ok(())
}
