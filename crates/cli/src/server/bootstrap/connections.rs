//! Long-lived connection pools owned for the lifetime of the process.

use anyhow::{Context, Result};

use domain::config::SystemConfig;
use infrastructure::connection::PgPool;
use infrastructure::connection::postgres_conn;
use tracing::{info};

/// Aggregates every connection pool / client the application depends on.
/// Each field is built once at boot and shared (typically via `Arc`) into
/// repositories, the Kafka publisher, etc.
#[derive(Debug)]
pub struct Connections {
    pub pg_pool: PgPool,
}

pub fn init(config: &SystemConfig) -> Result<Connections> {
    let pg_pool = postgres_conn::create_pool(&config.repository.postgresql)
        .context("initialising PostgreSQL pool")?;
    info!("PostgreSQL connection pool initialised");

    Ok(Connections { pg_pool })
}
