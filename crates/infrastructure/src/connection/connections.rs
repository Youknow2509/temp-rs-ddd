use crate::cache::{MokaCache, RedisCache};
use crate::connection::{GrpcClients, KafkaClient, PgPool, S3Client, ScyllaSession};

/// All long-lived connection handles owned for the process lifetime.
///
/// Wrapped in `Arc<Connections>` and shared read-only across every inbound
/// interface (HTTP, gRPC, WebSocket, Kafka consumer).  Each interface clones
/// the `Arc` — O(1) atomic bump, no data is copied.
///
/// Per-request DDD structs (repos, services, use cases) borrow `&'a`
/// references from here — zero heap allocation in the hot path.
pub struct Connections {
    /// PostgreSQL connection pool (deadpool, internally Arc-based).
    pub pg: PgPool,

    /// Redis connection pool, single or cluster mode.
    pub redis: RedisCache,

    /// In-process Moka cache.
    pub moka: MokaCache,

    /// ScyllaDB session (`Arc<Session>` internally).
    pub scylla: ScyllaSession,

    /// AWS S3 (or compatible) client.
    pub s3: S3Client,

    /// Kafka producer + consumer.
    pub kafka: KafkaClient,

    /// gRPC client channels keyed by logical service name.
    pub grpc: GrpcClients,
    // Qdrant client.
    // pub qdrant: QdrantClient,
}
