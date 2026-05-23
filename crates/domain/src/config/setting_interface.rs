use super::{KafkaConsumerSetting, TLSSetting};
use serde::Deserialize;
use std::net::IpAddr;

/// Interface for system setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct InterfacesSystemSetting {
    pub http_server: HttpServerSetting,
    pub grpc_server: GrpcServerSetting,
    pub ws_server: WebsocketServerSetting,
    pub kafka_consumer: KafkaConsumerSetting,
}

// ===
// HTTP Server
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpServerSetting {
    pub network: HttpNetworkSetting,
    pub worker: HttpWorkerSetting,
    pub timeouts: HttpTimeoutSetting,
    pub limits: HttpLimitSetting,
    pub tcp: HttpTcpSetting,
    pub security: HttpSecuritySetting,
}

/// HTTP network binding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpNetworkSetting {
    pub host: IpAddr,
    pub port: u16,
}

/// HTTP worker / concurrency settings
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpWorkerSetting {
    pub worker_threads: usize,
    pub max_connections: usize,
    pub backlog: i32,
}

/// HTTP timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpTimeoutSetting {
    /// Milliseconds
    pub read_timeout_ms: u64,
    /// Milliseconds
    pub write_timeout_ms: u64,
    /// Milliseconds
    pub idle_timeout_ms: u64,
    /// Milliseconds
    pub shutdown_timeout_ms: u64,
}

/// HTTP request size limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpLimitSetting {
    pub max_body_size: usize,
    pub max_header_size: usize,
}

/// HTTP TCP / protocol options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpTcpSetting {
    pub http2_enabled: bool,
    pub tcp_nodelay: bool,
}

/// HTTP security: CORS + TLS
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct HttpSecuritySetting {
    pub cors: CorsSetting,
    pub tls: TLSSetting,
}

// ===
// gRPC Server
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcServerSetting {
    pub network: GrpcNetworkSetting,
    pub worker: GrpcWorkerSetting,
    pub timeouts: GrpcTimeoutSetting,
    pub limits: GrpcLimitSetting,
    pub http2: GrpcHttp2Setting,
    pub keepalive: GrpcKeepaliveSetting,
    pub tcp: GrpcTcpSetting,
    pub protocol: GrpcProtocolSetting,
    pub security: GrpcSecuritySetting,
}

/// gRPC network binding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcNetworkSetting {
    pub host: IpAddr,
    pub port: u16,
}

/// gRPC worker / concurrency settings
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcWorkerSetting {
    pub worker_threads: usize,
    pub max_connections: usize,
    pub concurrency_limit_per_connection: usize,
    pub backlog: i32,
}

/// gRPC timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcTimeoutSetting {
    /// Milliseconds
    pub request_timeout_ms: u64,
    /// Milliseconds
    pub connect_timeout_ms: u64,
    /// Milliseconds
    pub idle_timeout_ms: u64,
    /// Milliseconds
    pub shutdown_timeout_ms: u64,
}

/// gRPC message size limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcLimitSetting {
    pub max_decoding_message_size: usize,
    pub max_encoding_message_size: usize,
    pub max_frame_size: u32,
}

/// gRPC HTTP/2 tuning
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcHttp2Setting {
    pub initial_stream_window_size: u32,
    pub initial_connection_window_size: u32,
    pub max_concurrent_streams: u32,
}

/// gRPC keep-alive (HTTP/2 + TCP)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcKeepaliveSetting {
    /// Milliseconds
    pub http2_keepalive_interval_ms: u64,
    /// Milliseconds
    pub http2_keepalive_timeout_ms: u64,
    pub http2_adaptive_window: bool,
    /// Milliseconds
    pub tcp_keepalive_ms: u64,
}

/// gRPC TCP socket options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcTcpSetting {
    pub tcp_nodelay: bool,
}

/// gRPC protocol feature flags
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcProtocolSetting {
    pub accept_http1: bool,
    pub reflection_enabled: bool,
    pub health_check_enabled: bool,
}

/// gRPC security: TLS only
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct GrpcSecuritySetting {
    pub tls: TLSSetting,
}

// ===
// WebSocket Server
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketServerSetting {
    pub network: WebsocketNetworkSetting,
    pub worker: WebsocketWorkerSetting,
    pub timeouts: WebsocketTimeoutSetting,
    pub heartbeat: WebsocketHeartbeatSetting,
    pub limits: WebsocketLimitSetting,
    pub protocol: WebsocketProtocolSetting,
    pub tcp: WebsocketTcpSetting,
    pub security: WebsocketSecuritySetting,
}

/// WebSocket network binding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketNetworkSetting {
    pub host: IpAddr,
    pub port: u16,
    pub path: String,
}

/// WebSocket worker / connection limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketWorkerSetting {
    pub worker_threads: usize,
    pub max_connections: usize,
    pub max_connections_per_ip: usize,
    pub backlog: i32,
}

/// WebSocket timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketTimeoutSetting {
    /// Milliseconds
    pub handshake_timeout_ms: u64,
    /// Milliseconds
    pub read_timeout_ms: u64,
    /// Milliseconds
    pub write_timeout_ms: u64,
    /// Milliseconds
    pub idle_timeout_ms: u64,
    /// Milliseconds
    pub shutdown_timeout_ms: u64,
}

/// WebSocket ping/pong heartbeat
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketHeartbeatSetting {
    /// Milliseconds
    pub ping_interval_ms: u64,
    /// Milliseconds
    pub pong_timeout_ms: u64,
}

/// WebSocket message limits
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketLimitSetting {
    pub max_message_size: usize,
    pub max_frame_size: usize,
    pub max_messages_per_second: u32,
}

/// WebSocket protocol options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketProtocolSetting {
    pub subprotocols: Vec<String>,
    pub permessage_deflate: bool,
    pub auto_fragment: bool,
    pub accept_unmasked_frames: bool,
}

/// WebSocket TCP socket options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketTcpSetting {
    pub tcp_nodelay: bool,
    pub tcp_keepalive: bool,
}

/// WebSocket security: CORS + TLS + origin whitelist
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct WebsocketSecuritySetting {
    pub cors: CorsSetting,
    pub tls: TLSSetting,
    pub allowed_origins: Vec<String>,
}

// ===
// CORS
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsSetting {
    pub enabled: bool,

    // Origin
    pub origin: CorsOriginSetting,

    // Methods
    pub methods: CorsMethodSetting,

    // Headers
    pub headers: CorsHeaderSetting,

    // Credentials
    pub allow_credentials: bool,

    // Preflight
    /// Seconds
    pub max_age_secs: u64,

    // Private network access
    pub allow_private_network: bool,
}

/// CORS origin policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsOriginSetting {
    pub allowed_origins: Vec<String>,
    pub allow_any_origin: bool,
}

/// CORS method policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsMethodSetting {
    pub allowed_methods: Vec<String>,
    pub allow_any_method: bool,
}

/// CORS header policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct CorsHeaderSetting {
    pub allowed_headers: Vec<String>,
    pub allow_any_header: bool,
    pub exposed_headers: Vec<String>,
}
