use std::collections::HashMap;

use super::{KafkaConnectionSetting, KafkaProducerSetting, TLSSetting};
use serde::Deserialize;

/// Outbound clients: things this service calls / publishes to.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ClientSystemSetting {
    /// Kafka publisher (this service producing events)
    pub kafka_publisher: KafkaPublisherSetting,

    /// gRPC clients targeting internal services
    pub grpc_clients: GrpcClientsSetting,
}

// ===
// Kafka Publisher
// ===

/// Kafka publisher: connection + producer tuning.
///
/// Connection is duplicated from the consumer side because publish-side
/// clusters / credentials are commonly different from the consume-side
/// cluster (e.g. dedicated egress cluster, different SASL principal).
///
/// Topic names are defined as constants in `infrastructure::connection::kafka_topics`
/// and are not configured here — only infra knobs (brokers, auth, timeouts) belong.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaPublisherSetting {
    pub connection: KafkaConnectionSetting,
    pub producer: KafkaProducerSetting,
}

// ===
// gRPC Clients
// ===

/// gRPC clients setting: default knobs plus per-service overrides.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientsSetting {
    /// Is enabled
    pub is_enabled: bool,

    /// Defaults applied to every internal gRPC client unless overridden.
    pub defaults: GrpcClientSetting,

    /// Per-service overrides keyed by logical service name
    /// (e.g. "user-service", "billing-service", "search-service").
    pub services: HashMap<String, GrpcClientSetting>,
}

/// gRPC client configuration for a single upstream service.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientSetting {
    /// Run grpc.health.v1.Health/Check at startup before accepting traffic.
    /// Set to false in dev for services that are not locally available.
    #[serde(default = "default_true")]
    pub health_check_enabled: bool,

    /// Service discovery / endpoint resolution
    pub endpoint: GrpcClientEndpointSetting,

    /// Connection establishment
    pub connection: GrpcClientConnectionSetting,

    /// Request/response timeouts
    pub timeouts: GrpcClientTimeoutSetting,

    /// Connection pool / multiplexing
    pub pool: GrpcClientPoolSetting,

    /// HTTP/2 keep-alive
    pub keepalive: GrpcClientKeepaliveSetting,

    /// Retry policy
    pub retry: GrpcClientRetrySetting,

    /// Circuit breaker
    pub circuit_breaker: GrpcClientCircuitBreakerSetting,

    /// Load balancing across endpoints
    pub load_balancing: GrpcClientLoadBalancingSetting,

    /// Message size limits
    pub limits: GrpcClientLimitSetting,

    /// Compression
    pub compression: GrpcClientCompressionSetting,

    /// TLS / mTLS
    pub tls: TLSSetting,

    /// Auth (token attached to every outbound RPC, optional)
    pub auth: Option<GrpcClientAuthSetting>,
}

/// Endpoint resolution for a gRPC client.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientEndpointSetting {
    /// "static" | "dns" | "consul" | "etcd" | "k8s"
    pub discovery: String,
    /// Static endpoints (host:port). Used when discovery = "static".
    pub addresses: Vec<String>,
    /// DNS / service name (e.g. "user-service.internal"). Used for non-static discovery.
    pub service_name: Option<String>,
    /// Authority override (HTTP/2 :authority pseudo-header)
    pub authority: Option<String>,
    /// gRPC service path prefix (e.g. "com.example.UserService")
    pub service: Option<String>,
}

/// Connection establishment settings.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientConnectionSetting {
    /// Milliseconds
    pub connect_timeout_ms: u64,
    /// Milliseconds — total time spent on initial connect attempts
    pub initial_backoff_ms: u64,
    /// Milliseconds
    pub max_backoff_ms: u64,
    /// Lazily connect on first RPC instead of eagerly at startup
    pub lazy_connect: bool,
    pub tcp_nodelay: bool,
}

/// Per-RPC timeouts.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientTimeoutSetting {
    /// Milliseconds — default deadline per unary RPC
    pub request_timeout_ms: u64,
    /// Milliseconds — default deadline per streaming RPC (0 = no deadline)
    pub stream_timeout_ms: u64,
    /// Milliseconds — drain time on shutdown
    pub shutdown_timeout_ms: u64,
}

/// Connection pool / channel multiplexing.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientPoolSetting {
    /// Number of physical HTTP/2 sub-channels per endpoint
    pub connections_per_endpoint: u32,
    /// Max concurrent streams per single HTTP/2 connection
    pub max_concurrent_streams_per_connection: u32,
    /// Milliseconds — idle channel reaper
    pub idle_timeout_ms: u64,
}

/// HTTP/2 keep-alive (client side).
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientKeepaliveSetting {
    /// Milliseconds
    pub http2_keepalive_interval_ms: u64,
    /// Milliseconds
    pub http2_keepalive_timeout_ms: u64,
    /// Send keep-alive even when no active streams
    pub keepalive_while_idle: bool,
    /// Milliseconds — TCP-level keep-alive
    pub tcp_keepalive_ms: u64,
}

/// Retry policy (gRPC service config style).
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientRetrySetting {
    pub enabled: bool,
    pub max_attempts: u32,
    /// Milliseconds
    pub initial_backoff_ms: u64,
    /// Milliseconds
    pub max_backoff_ms: u64,
    pub backoff_multiplier: f64,
    /// gRPC status codes that are retryable
    /// (e.g. "UNAVAILABLE", "DEADLINE_EXCEEDED", "RESOURCE_EXHAUSTED")
    pub retryable_status_codes: Vec<String>,
    /// Add jitter to backoff to avoid thundering herd
    pub use_jitter: bool,
}

/// Circuit breaker for fast-fail on a failing upstream.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientCircuitBreakerSetting {
    pub enabled: bool,
    /// Failures within the rolling window before tripping open
    pub failure_threshold: u32,
    /// Successes in half-open state before closing again
    pub success_threshold: u32,
    /// Milliseconds — how long the breaker stays open before half-open
    pub open_state_duration_ms: u64,
    /// Milliseconds — rolling window for counting failures
    pub rolling_window_ms: u64,
}

/// Client-side load balancing across endpoints.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientLoadBalancingSetting {
    /// "round_robin" | "pick_first" | "least_request" | "ring_hash"
    pub policy: String,
    /// For "ring_hash" — header used to compute the hash
    pub hash_header: Option<String>,
    /// Enable client-side subsetting (only talk to N of all endpoints)
    pub subset_size: Option<u32>,
}

/// gRPC message size limits.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientLimitSetting {
    pub max_decoding_message_size: usize,
    pub max_encoding_message_size: usize,
    pub max_frame_size: u32,
}

/// Compression for outbound gRPC calls.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientCompressionSetting {
    /// "none" | "gzip" | "zstd"
    pub send_codec: String,
    /// Accepted response codecs (server may choose any)
    pub accept_codecs: Vec<String>,
}

/// Outbound auth attached to every RPC.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientAuthSetting {
    /// "bearer" | "basic" | "mtls" | "api_key"
    pub scheme: String,
    /// Header name to inject (defaults to "authorization" for bearer/basic)
    pub header: Option<String>,
    /// Static token (prefer secret store / env in real deployments)
    pub token: Option<String>,
    /// API key value when scheme = "api_key"
    pub api_key: Option<String>,
}

fn default_true() -> bool {
    true
}
