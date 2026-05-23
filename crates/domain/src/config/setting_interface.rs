use super::{KafkaConsumerSetting, TLSSetting};
use serde::Deserialize;
use std::net::IpAddr;

/// Interface for system setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct InterfacesSystemSetting {
    http_server: HttpServerSetting,
    grpc_server: GrpcServerSetting,
    ws_server: WebsocketServerSetting,
    kafka_consumer: KafkaConsumerSetting,
}

// ===
// HTTP Server
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpServerSetting {
    network: HttpNetworkSetting,
    worker: HttpWorkerSetting,
    timeouts: HttpTimeoutSetting,
    limits: HttpLimitSetting,
    tcp: HttpTcpSetting,
    security: HttpSecuritySetting,
}

/// HTTP network binding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpNetworkSetting {
    host: IpAddr,
    port: u16,
}

/// HTTP worker / concurrency settings
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpWorkerSetting {
    worker_threads: usize,
    max_connections: usize,
    backlog: i32,
}

/// HTTP timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpTimeoutSetting {
    /// Milliseconds
    read_timeout_ms: u64,
    /// Milliseconds
    write_timeout_ms: u64,
    /// Milliseconds
    idle_timeout_ms: u64,
    /// Milliseconds
    shutdown_timeout_ms: u64,
}

/// HTTP request size limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpLimitSetting {
    max_body_size: usize,
    max_header_size: usize,
}

/// HTTP TCP / protocol options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpTcpSetting {
    http2_enabled: bool,
    tcp_nodelay: bool,
}

/// HTTP security: CORS + TLS
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpSecuritySetting {
    cors: CorsSetting,
    tls: TLSSetting,
}

// ===
// gRPC Server
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcServerSetting {
    network: GrpcNetworkSetting,
    worker: GrpcWorkerSetting,
    timeouts: GrpcTimeoutSetting,
    limits: GrpcLimitSetting,
    http2: GrpcHttp2Setting,
    keepalive: GrpcKeepaliveSetting,
    tcp: GrpcTcpSetting,
    protocol: GrpcProtocolSetting,
    security: GrpcSecuritySetting,
}

/// gRPC network binding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcNetworkSetting {
    host: IpAddr,
    port: u16,
}

/// gRPC worker / concurrency settings
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcWorkerSetting {
    worker_threads: usize,
    max_connections: usize,
    concurrency_limit_per_connection: usize,
    backlog: i32,
}

/// gRPC timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcTimeoutSetting {
    /// Milliseconds
    request_timeout_ms: u64,
    /// Milliseconds
    connect_timeout_ms: u64,
    /// Milliseconds
    idle_timeout_ms: u64,
    /// Milliseconds
    shutdown_timeout_ms: u64,
}

/// gRPC message size limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcLimitSetting {
    max_decoding_message_size: usize,
    max_encoding_message_size: usize,
    max_frame_size: u32,
}

/// gRPC HTTP/2 tuning
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcHttp2Setting {
    initial_stream_window_size: u32,
    initial_connection_window_size: u32,
    max_concurrent_streams: u32,
}

/// gRPC keep-alive (HTTP/2 + TCP)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcKeepaliveSetting {
    /// Milliseconds
    http2_keepalive_interval_ms: u64,
    /// Milliseconds
    http2_keepalive_timeout_ms: u64,
    http2_adaptive_window: bool,
    /// Milliseconds
    tcp_keepalive_ms: u64,
}

/// gRPC TCP socket options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcTcpSetting {
    tcp_nodelay: bool,
}

/// gRPC protocol feature flags
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcProtocolSetting {
    accept_http1: bool,
    reflection_enabled: bool,
    health_check_enabled: bool,
}

/// gRPC security: TLS only
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcSecuritySetting {
    tls: TLSSetting,
}

// ===
// WebSocket Server
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketServerSetting {
    network: WebsocketNetworkSetting,
    worker: WebsocketWorkerSetting,
    timeouts: WebsocketTimeoutSetting,
    heartbeat: WebsocketHeartbeatSetting,
    limits: WebsocketLimitSetting,
    protocol: WebsocketProtocolSetting,
    tcp: WebsocketTcpSetting,
    security: WebsocketSecuritySetting,
}

/// WebSocket network binding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketNetworkSetting {
    host: IpAddr,
    port: u16,
    path: String,
}

/// WebSocket worker / connection limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketWorkerSetting {
    worker_threads: usize,
    max_connections: usize,
    max_connections_per_ip: usize,
    backlog: i32,
}

/// WebSocket timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketTimeoutSetting {
    /// Milliseconds
    handshake_timeout_ms: u64,
    /// Milliseconds
    read_timeout_ms: u64,
    /// Milliseconds
    write_timeout_ms: u64,
    /// Milliseconds
    idle_timeout_ms: u64,
    /// Milliseconds
    shutdown_timeout_ms: u64,
}

/// WebSocket ping/pong heartbeat
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketHeartbeatSetting {
    /// Milliseconds
    ping_interval_ms: u64,
    /// Milliseconds
    pong_timeout_ms: u64,
}

/// WebSocket message limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketLimitSetting {
    max_message_size: usize,
    max_frame_size: usize,
    max_messages_per_second: u32,
}

/// WebSocket protocol options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketProtocolSetting {
    subprotocols: Vec<String>,
    permessage_deflate: bool,
    auto_fragment: bool,
    accept_unmasked_frames: bool,
}

/// WebSocket TCP socket options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketTcpSetting {
    tcp_nodelay: bool,
    tcp_keepalive: bool,
}

/// WebSocket security: CORS + TLS + origin whitelist
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketSecuritySetting {
    cors: CorsSetting,
    tls: TLSSetting,
    allowed_origins: Vec<String>,
}

// ===
// CORS
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsSetting {
    enabled: bool,

    // Origin
    origin: CorsOriginSetting,

    // Methods
    methods: CorsMethodSetting,

    // Headers
    headers: CorsHeaderSetting,

    // Credentials
    allow_credentials: bool,

    // Preflight
    /// Seconds
    max_age_secs: u64,

    // Private network access
    allow_private_network: bool,
}

/// CORS origin policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsOriginSetting {
    allowed_origins: Vec<String>,
    allow_any_origin: bool,
}

/// CORS method policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsMethodSetting {
    allowed_methods: Vec<String>,
    allow_any_method: bool,
}

/// CORS header policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsHeaderSetting {
    allowed_headers: Vec<String>,
    allow_any_header: bool,
    exposed_headers: Vec<String>,
}
