use super::{KafkaConnectionSetting, KafkaProducerSetting, TLSSetting};
use serde::Deserialize;
use std::collections::HashMap;

/// Outbound clients: things this service calls / publishes to.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ClientSystemSetting {
    /// Kafka publisher (this service producing events)
    kafka_publisher: KafkaPublisherSetting,

    /// gRPC clients targeting internal services
    grpc_clients: GrpcClientsSetting,
}

// ===
// Kafka Publisher
// ===

/// Kafka publisher: connection + producer tuning.
///
/// Connection is duplicated from the consumer side because publish-side
/// clusters / credentials are commonly different from the consume-side
/// cluster (e.g. dedicated egress cluster, different SASL principal).
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaPublisherSetting {
    connection: KafkaConnectionSetting,
    producer: KafkaProducerSetting,

    /// Default topic when caller does not specify one (optional).
    default_topic: Option<String>,

    /// Per-topic overrides (e.g. different acks / linger per topic).
    topics: HashMap<String, KafkaPublisherTopicSetting>,
}

/// Per-topic publisher overrides.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaPublisherTopicSetting {
    /// Override acks for this topic ("0" | "1" | "all")
    acks: Option<String>,
    /// Override compression for this topic
    compression_type: Option<String>,
    /// Milliseconds — override linger
    linger_ms: Option<u64>,
    /// Override batch size
    batch_size: Option<usize>,
    /// Override partitioner
    partitioner: Option<String>,
}

// ===
// gRPC Clients
// ===

/// gRPC clients setting: default knobs plus per-service overrides.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientsSetting {
    /// Defaults applied to every internal gRPC client unless overridden.
    defaults: GrpcClientSetting,

    /// Per-service overrides keyed by logical service name
    /// (e.g. "user-service", "billing-service", "search-service").
    services: HashMap<String, GrpcClientSetting>,
}

/// gRPC client configuration for a single upstream service.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientSetting {
    /// Service discovery / endpoint resolution
    endpoint: GrpcClientEndpointSetting,

    /// Connection establishment
    connection: GrpcClientConnectionSetting,

    /// Request/response timeouts
    timeouts: GrpcClientTimeoutSetting,

    /// Connection pool / multiplexing
    pool: GrpcClientPoolSetting,

    /// HTTP/2 keep-alive
    keepalive: GrpcClientKeepaliveSetting,

    /// Retry policy
    retry: GrpcClientRetrySetting,

    /// Circuit breaker
    circuit_breaker: GrpcClientCircuitBreakerSetting,

    /// Load balancing across endpoints
    load_balancing: GrpcClientLoadBalancingSetting,

    /// Message size limits
    limits: GrpcClientLimitSetting,

    /// Compression
    compression: GrpcClientCompressionSetting,

    /// TLS / mTLS
    tls: TLSSetting,

    /// Auth (token attached to every outbound RPC, optional)
    auth: Option<GrpcClientAuthSetting>,
}

/// Endpoint resolution for a gRPC client.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientEndpointSetting {
    /// "static" | "dns" | "consul" | "etcd" | "k8s"
    discovery: String,
    /// Static endpoints (host:port). Used when discovery = "static".
    addresses: Vec<String>,
    /// DNS / service name (e.g. "user-service.internal"). Used for non-static discovery.
    service_name: Option<String>,
    /// Authority override (HTTP/2 :authority pseudo-header)
    authority: Option<String>,
    /// gRPC service path prefix (e.g. "com.example.UserService")
    service: Option<String>,
}

/// Connection establishment settings.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientConnectionSetting {
    /// Milliseconds
    connect_timeout_ms: u64,
    /// Milliseconds — total time spent on initial connect attempts
    initial_backoff_ms: u64,
    /// Milliseconds
    max_backoff_ms: u64,
    /// Lazily connect on first RPC instead of eagerly at startup
    lazy_connect: bool,
    tcp_nodelay: bool,
}

/// Per-RPC timeouts.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientTimeoutSetting {
    /// Milliseconds — default deadline per unary RPC
    request_timeout_ms: u64,
    /// Milliseconds — default deadline per streaming RPC (0 = no deadline)
    stream_timeout_ms: u64,
    /// Milliseconds — drain time on shutdown
    shutdown_timeout_ms: u64,
}

/// Connection pool / channel multiplexing.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientPoolSetting {
    /// Number of physical HTTP/2 sub-channels per endpoint
    connections_per_endpoint: u32,
    /// Max concurrent streams per single HTTP/2 connection
    max_concurrent_streams_per_connection: u32,
    /// Milliseconds — idle channel reaper
    idle_timeout_ms: u64,
}

/// HTTP/2 keep-alive (client side).
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientKeepaliveSetting {
    /// Milliseconds
    http2_keepalive_interval_ms: u64,
    /// Milliseconds
    http2_keepalive_timeout_ms: u64,
    /// Send keep-alive even when no active streams
    keepalive_while_idle: bool,
    /// Milliseconds — TCP-level keep-alive
    tcp_keepalive_ms: u64,
}

/// Retry policy (gRPC service config style).
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientRetrySetting {
    enabled: bool,
    max_attempts: u32,
    /// Milliseconds
    initial_backoff_ms: u64,
    /// Milliseconds
    max_backoff_ms: u64,
    backoff_multiplier: f64,
    /// gRPC status codes that are retryable
    /// (e.g. "UNAVAILABLE", "DEADLINE_EXCEEDED", "RESOURCE_EXHAUSTED")
    retryable_status_codes: Vec<String>,
    /// Add jitter to backoff to avoid thundering herd
    use_jitter: bool,
}

/// Circuit breaker for fast-fail on a failing upstream.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientCircuitBreakerSetting {
    enabled: bool,
    /// Failures within the rolling window before tripping open
    failure_threshold: u32,
    /// Successes in half-open state before closing again
    success_threshold: u32,
    /// Milliseconds — how long the breaker stays open before half-open
    open_state_duration_ms: u64,
    /// Milliseconds — rolling window for counting failures
    rolling_window_ms: u64,
}

/// Client-side load balancing across endpoints.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientLoadBalancingSetting {
    /// "round_robin" | "pick_first" | "least_request" | "ring_hash"
    policy: String,
    /// For "ring_hash" — header used to compute the hash
    hash_header: Option<String>,
    /// Enable client-side subsetting (only talk to N of all endpoints)
    subset_size: Option<u32>,
}

/// gRPC message size limits.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientLimitSetting {
    max_decoding_message_size: usize,
    max_encoding_message_size: usize,
    max_frame_size: u32,
}

/// Compression for outbound gRPC calls.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientCompressionSetting {
    /// "none" | "gzip" | "zstd"
    send_codec: String,
    /// Accepted response codecs (server may choose any)
    accept_codecs: Vec<String>,
}

/// Outbound auth attached to every RPC.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcClientAuthSetting {
    /// "bearer" | "basic" | "mtls" | "api_key"
    scheme: String,
    /// Header name to inject (defaults to "authorization" for bearer/basic)
    header: Option<String>,
    /// Static token (prefer secret store / env in real deployments)
    token: Option<String>,
    /// API key value when scheme = "api_key"
    api_key: Option<String>,
}
