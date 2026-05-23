//! Long-lived connection pools owned for the lifetime of the process.

use anyhow::Result;

use domain::config::SystemConfig;

/// Aggregates every connection pool / client the application depends on.
/// Each field is built once at boot and shared (typically via `Arc`) into
/// repositories, the Kafka publisher, etc.
#[derive(Debug, Default)]
pub struct Connections {
    // TODO: pg_pool, redis_pool, scylla_session, s3_client,
    //       kafka_publisher, grpc clients map, ...
}

pub fn init(_config: &SystemConfig) -> Result<Connections> {
    // TODO: build pools from `config.repository` and `config.clients`.
    Ok(Connections::default())
}
