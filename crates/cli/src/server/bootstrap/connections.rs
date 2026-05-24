//! Initialise every connection pool / client once at boot.
//!
//! Returns `infrastructure::Connections` — the single source of truth for
//! all long-lived handles.  The caller wraps it in `Arc` so every interface
//! can clone the pointer without copying any connection data.

use anyhow::{Context, Result};
use tracing::info;

use domain::config::SystemConfig;
use infrastructure::cache::{MokaCache, RedisCache};
use infrastructure::connection::{
    Connections, GrpcClients, KafkaClient, PgPool, S3Client, ScyllaSession, grpc_conn, kafka_conn,
    postgres_conn, redis_conn, s3_conn, scylla_conn,
};

pub async fn init(config: &SystemConfig) -> Result<Connections> {
    let (pg, redis, scylla, s3) = tokio::task::block_in_place(|| -> Result<_> {
        let pg: PgPool = postgres_conn::create_pool(&config.repository.postgresql)
            .context("initialising PostgreSQL pool")?;
        info!("PostgreSQL connection pool initialised");

        let redis_pool =
            redis_conn::create_pool(&config.repository.redis).context("initialising Redis pool")?;
        let redis = RedisCache::new(redis_pool);
        info!("Redis connection pool initialised");

        let scylla: ScyllaSession = scylla_conn::create_session(&config.repository.scylladb)
            .context("initialising ScyllaDB session")?;
        info!("ScyllaDB session initialised");

        let s3: S3Client = s3_conn::create_client(&config.repository.object_storage)
            .context("initialising S3 client")?;
        info!("S3 client initialised");

        Ok((pg, redis, scylla, s3))
    })?;

    let moka = MokaCache::new(&config.repository.local_cache);
    info!("Moka local cache initialised");

    let kafka: KafkaClient = kafka_conn::create_client(
        &config.clients.kafka_publisher.connection,
        &config.clients.kafka_publisher.producer,
        &config.interfaces.kafka_consumer,
    )
    .await
    .context("initialising Kafka client")?;
    info!("Kafka client initialised");

    let grpc: GrpcClients = grpc_conn::create_clients(&config.clients.grpc_clients)
        .await
        .context("initialising gRPC clients")?;
    info!("gRPC clients initialised");

    Ok(Connections {
        pg,
        redis,
        moka,
        scylla,
        s3,
        kafka,
        grpc,
    })
}
