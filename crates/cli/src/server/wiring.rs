//! Phase 2: wire the DDD layers on top of the bootstrap primitives.
//!
//! Direction of dependency (outer -> inner):
//!   Connections ──► Repositories ──► Services ──► UseCases
//!
//! Bootstrap is fully destructured here so `redis_pool` can be moved
//! directly into `Repositories` without Option or Arc.

pub mod repositories;
pub mod services;
pub mod use_cases;

use anyhow::Result;
use telemetry::TelemetryGuard;

use domain::config::SystemConfig;
use infrastructure::{
    cache::{MokaCache, RedisCache},
    connection::{GrpcClients, KafkaClient, PgPool, S3Client, ScyllaSession},
};

use std::sync::Arc;

use super::bootstrap::Bootstrap;

use self::{repositories::Repositories, services::Services, use_cases::UseCases};

/// Everything the run and shutdown phases need — flattened for clarity.
/// Bootstrap is consumed during wiring; only the fields actually needed
/// afterwards are kept.
#[derive(Debug)]
#[allow(dead_code)]
pub struct Wired {
    // ── Config ────────────────────────────────────────────────────────────
    pub config: SystemConfig,

    // ── Shared connections (outlive wiring) ───────────────────────────────
    pub pg_pool: Arc<PgPool>,
    pub scylla_session: Arc<ScyllaSession>,
    pub s3_client: S3Client,
    pub kafka_client: KafkaClient,
    pub grpc_clients: GrpcClients,

    // ── DDD layers ────────────────────────────────────────────────────────
    pub repositories: Repositories<MokaCache, RedisCache>,
    pub services: Services,
    pub use_cases: UseCases,

    // ── Telemetry (dropped last in shutdown) ──────────────────────────────
    pub telemetry_guard: TelemetryGuard,
}

pub fn wire(bootstrap: Bootstrap) -> Result<Wired> {
    let Bootstrap {
        config,
        connections,
        telemetry_guard,
    } = bootstrap;

    // Destructure Connections so redis_pool is moved — no Arc, no Option.
    let super::bootstrap::connections::Connections {
        pg_pool,
        redis_pool,
        scylla_session,
        s3_client,
        kafka_client,
        grpc_clients,
    } = connections;

    let repositories = repositories::build(&config, redis_pool)?;
    let services = services::build(&repositories)?;
    let use_cases = use_cases::build(&services)?;

    Ok(Wired {
        config,
        pg_pool,
        scylla_session,
        s3_client,
        kafka_client,
        grpc_clients,
        repositories,
        services,
        use_cases,
        telemetry_guard,
    })
}
