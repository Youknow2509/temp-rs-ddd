//! Long-lived connection pools owned for the lifetime of the process.

use anyhow::{Context, Result};

use domain::config::SystemConfig;
use infrastructure::connection::PgPool;
use infrastructure::connection::RedisPool;
use infrastructure::connection::postgres_conn;
use infrastructure::connection::redis_conn;
use tracing::info;

/// Aggregates every connection pool / client the application depends on.
/// Each field is built once at boot and shared (typically via `Arc`) into
/// repositories, the Kafka publisher, etc.
#[derive(Debug)]
pub struct Connections {
    pub pg_pool: PgPool,
    pub redis_pool: RedisPool,
}

pub fn init(config: &SystemConfig) -> Result<Connections> {
    let pg_pool = postgres_conn::create_pool(&config.repository.postgresql)
        .context("initialising PostgreSQL pool")?;
    info!("PostgreSQL connection pool initialised");

    let redis_pool = redis_conn::create_pool(&config.repository.redis)
        .context("initialising Redis pool")?;
    info!("Redis connection pool initialised");

    Ok(Connections {
        pg_pool,
        redis_pool,
    })
}
