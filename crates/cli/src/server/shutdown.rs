//! Phase 4: wait for a shutdown signal, then drain and close all resources.

pub mod signal;

use anyhow::Result;
use tracing::info;

use infrastructure::connection::redis_conn::RedisPool;

use super::wiring::Wired;

/// Await SIGINT / SIGTERM, then close all connection pools.
pub async fn drain(wired: Wired) -> Result<()> {
    signal::wait().await?;

    info!("shutdown signal received — draining connections");

    wired.bootstrap.connections.pg_pool.close();
    info!("PostgreSQL pool closed");

    match &wired.bootstrap.connections.redis_pool {
        RedisPool::Single(p) => p.close(),
        RedisPool::Cluster(p) => p.close(),
    }
    info!("Redis pool closed");

    drop(wired.bootstrap.connections.scylla_session);
    info!("ScyllaDB session closed");

    drop(wired.bootstrap.connections.s3_client);
    info!("S3 client closed");

    drop(wired.bootstrap.connections.kafka_client);
    info!("Kafka client closed");

    drop(wired.bootstrap.connections.grpc_clients);
    info!("gRPC clients closed");

    Ok(())
}
