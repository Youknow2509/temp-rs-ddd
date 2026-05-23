//! Long-lived connection pools owned for the lifetime of the process.

use anyhow::{Context, Result};

use domain::config::SystemConfig;
use infrastructure::connection::KafkaClient;
use infrastructure::connection::PgPool;
use infrastructure::connection::RedisPool;
use infrastructure::connection::S3Client;
use infrastructure::connection::ScyllaSession;
use infrastructure::connection::kafka_conn;
use infrastructure::connection::postgres_conn;
use infrastructure::connection::redis_conn;
use infrastructure::connection::s3_conn;
use infrastructure::connection::scylla_conn;
use tracing::info;

/// Aggregates every connection pool / client the application depends on.
/// Each field is built once at boot and shared (typically via `Arc`) into
/// repositories, the Kafka publisher, etc.
#[derive(Debug)]
pub struct Connections {
    pub pg_pool: PgPool,
    pub redis_pool: RedisPool,
    pub scylla_session: ScyllaSession,
    pub s3_client: S3Client,
    pub kafka_client: KafkaClient,
}

pub async fn init(config: &SystemConfig) -> Result<Connections> {
    // postgres / redis / scylla / s3 init is synchronous and each creates its
    // own temp Tokio runtime for the health-check ping.  block_in_place lets
    // those nested block_on calls run without conflicting with the outer runtime.
    let (pg_pool, redis_pool, scylla_session, s3_client) =
        tokio::task::block_in_place(|| -> Result<_> {
            let pg = postgres_conn::create_pool(&config.repository.postgresql)
                .context("initialising PostgreSQL pool")?;
            info!("PostgreSQL connection pool initialised");

            let redis = redis_conn::create_pool(&config.repository.redis)
                .context("initialising Redis pool")?;
            info!("Redis connection pool initialised");

            let scylla = scylla_conn::create_session(&config.repository.scylladb)
                .context("initialising ScyllaDB session")?;
            info!("ScyllaDB session initialised");

            let s3 = s3_conn::create_client(&config.repository.object_storage)
                .context("initialising S3 client")?;
            info!("S3 client initialised");

            Ok((pg, redis, scylla, s3))
        })?;

    // Kafka uses rdkafka's tokio feature — init is natively async and runs on
    // the caller's runtime, so no embedded runtime is needed.
    let kafka_client = kafka_conn::create_client(
        &config.clients.kafka_publisher.connection,
        &config.clients.kafka_publisher.producer,
        &config.interfaces.kafka_consumer,
    )
    .await
    .context("initialising Kafka client")?;
    info!("Kafka client initialised");

    Ok(Connections {
        pg_pool,
        redis_pool,
        scylla_session,
        s3_client,
        kafka_client,
    })
}
