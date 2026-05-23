use super::TLSSetting;
use serde::Deserialize;

/// Repository System Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RepositorySystemSetting {
    postgresql: PostgresqlSettingRepository,
    redis: RedisSettingRepository,
    object_storage: ObjectStorageSetting,
    scylladb: ScyllaDbSettingRepository,
}

// ===
// ScyllaDB
// ===

/// ScyllaDB / Cassandra repository setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaDbSettingRepository {
    cluster: ScyllaClusterSetting,
    authentication: ScyllaAuthSetting,
    ssl: ScyllaSslSetting,
    consistency: ScyllaConsistencySetting,
    timeouts: ScyllaTimeoutSetting,
    pool: ScyllaPoolSetting,
    retry: ScyllaRetrySetting,
    speculative: ScyllaSpeculativeSetting,
    load_balancing: ScyllaLoadBalancingSetting,
    reconnection: ScyllaReconnectionSetting,
    prepared: ScyllaPreparedSetting,
    query: ScyllaQuerySetting,
}

/// Cluster contact points and keyspace
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaClusterSetting {
    contact_points: Vec<String>,
    keyspace: String,
    local_dc: String,
    protocol_version: u8, // 3 | 4 | 5
    compression: String,  // none | snappy | lz4
}

/// Authentication credentials
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaAuthSetting {
    username: String,
    password: String,
}

/// SSL/TLS configuration (ScyllaDB-specific naming)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaSslSetting {
    enabled: bool,
    ca_cert_path: String,
    user_cert_path: String,
    user_key_path: String,
    validate_hostname: bool,
    cipher_suites: Vec<String>,
}

/// Consistency levels
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaConsistencySetting {
    /// ANY | ONE | TWO | THREE | QUORUM | ALL | LOCAL_QUORUM | EACH_QUORUM | LOCAL_ONE
    default: String,
    /// SERIAL | LOCAL_SERIAL (for lightweight transactions)
    serial: String,
}

/// Timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaTimeoutSetting {
    /// Milliseconds
    connect_timeout_ms: u64,
    /// Milliseconds - query / request timeout
    request_timeout_ms: u64,
    /// Milliseconds - schema agreement timeout
    schema_agreement_timeout_ms: u64,
}

/// Connection pool (CQL native protocol model)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaPoolSetting {
    /// Connections per host in local DC (typically 1-4)
    connections_per_host_local: u32,
    /// Connections per host in remote DCs
    connections_per_host_remote: u32,
    /// Max concurrent requests per connection (CQL stream IDs, max 32768)
    max_requests_per_connection: u32,
    /// Milliseconds
    keepalive_interval_ms: u64,
}

/// Retry policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaRetrySetting {
    /// default | downgrading | fallthrough
    policy: String,
    max_retries: u32,
}

/// Speculative execution (tail latency optimization)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaSpeculativeSetting {
    enabled: bool,
    max_speculative_executions: u32,
    /// Milliseconds - delay before sending speculative query
    delay_ms: u64,
}

/// Load balancing policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaLoadBalancingSetting {
    /// round_robin | dc_aware | token_aware
    policy: String,
    shuffle_replicas: bool,
    allow_remote_dcs_for_local_cl: bool,
}

/// Reconnection policy when node goes down
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaReconnectionSetting {
    /// constant | exponential
    policy: String,
    /// Milliseconds
    base_delay_ms: u64,
    /// Milliseconds
    max_delay_ms: u64,
}

/// Prepared statement cache
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaPreparedSetting {
    cache_size: u32,
    prepare_on_all_hosts: bool,
}

/// Query default options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaQuerySetting {
    page_size: u32,
    fetch_size: u32,
    tracing_enabled: bool,
}

// ===
// Object Storage
// ===

/// Object Storage for Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageSetting {
    r#type: String, // s3 | gcs | azure

    bucket_name: String,
    region: String,

    endpoint: String,
    force_path_style: bool,

    access_key_id: String,
    secret_access_key: String,
    session_token: String,

    tls: TLSSetting,

    timeouts: ObjectStorageTimeoutSetting,
    pool: ObjectStoragePoolSetting,
    retry: ObjectStorageRetrySetting,
    upload: ObjectStorageUploadSetting,
    download: ObjectStorageDownloadSetting,
    server_side: ObjectStorageServerSideSetting,
    options: ObjectStorageOptionSetting,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageTimeoutSetting {
    /// Milliseconds
    connect_timeout_ms: u64,
    /// Milliseconds
    request_timeout_ms: u64,
    /// Milliseconds
    operation_timeout_ms: u64,
    /// Milliseconds
    idle_timeout_ms: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStoragePoolSetting {
    max_idle_connections: u32,
    max_connections_per_host: u32,
    /// Milliseconds
    keep_alive_ms: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageRetrySetting {
    max_retries: u32,
    /// Milliseconds
    min_backoff_ms: u64,
    /// Milliseconds
    max_backoff_ms: u64,
    use_jitter: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageUploadSetting {
    multipart_threshold: u64,
    multipart_chunk_size: u64,
    multipart_concurrency: u32,
    max_object_size: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageDownloadSetting {
    chunk_size: u64,
    concurrency: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageServerSideSetting {
    encryption: String, // none | aes256 | aws:kms
    kms_key_id: String,
    storage_class: String,
    checksum_algorithm: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageOptionSetting {
    enable_compression: bool,
    enable_acceleration: bool,
    addressing_style: String, // path | virtual
    user_agent: String,
}

// ===
// Redis
// ===

/// Redis for Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisSettingRepository {
    r#type: String, // standalone, sentinel, cluster

    // TLS
    tls: TLSSetting,

    // Auth
    username: String,
    password: String,
    db: u8,
    client_name: String,

    // Mode-specific
    standalone: Option<RedisStandaloneSetting>,
    sentinel: Option<RedisSentinelSetting>,
    cluster: Option<RedisClusterSetting>,

    // Shared settings
    timeouts: RedisTimeoutSetting,
    pool: RedisPoolSetting,
    retry: RedisRetrySetting,
}

/// Redis sub setting for standalone type
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisStandaloneSetting {
    host: String,
    port: u16,
}

/// Redis sub setting for sentinel type
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisSentinelSetting {
    master_name: String,
    sentinel_addrs: Vec<String>,
    sentinel_username: String,
    sentinel_password: String,
}

/// Redis sub setting for cluster type
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisClusterSetting {
    cluster_addrs: Vec<String>,
    route_by_latency: bool,
    route_randomly: bool,
    read_only: bool,
    max_redirects: u8,
}

/// Redis timeout configuration (seconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisTimeoutSetting {
    dial_timeout: u64,
    read_timeout: u64,
    write_timeout: u64,
    pool_timeout: u64,
}

/// Redis connection pool configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisPoolSetting {
    pool_size: u32,
    min_idle_conns: u32,
    max_idle_conns: u32,
    pool_fifo: bool,
    /// Seconds
    conn_max_idle_time: u64,
    /// Seconds
    conn_max_lifetime: u64,
}

/// Redis retry configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisRetrySetting {
    max_retries: u32,
    /// Milliseconds
    min_retry_backoff: u64,
    /// Milliseconds
    max_retry_backoff: u64,
}

// ===
// PostgreSQL
// ===

/// Postgresql for Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlSettingRepository {
    // Connection
    connection: PostgresqlConnectionSetting,

    // SSL
    tls: TLSSetting,

    // Other
    appname: String,
    tz: String,

    // Timeouts
    timeouts: PostgresqlTimeoutSetting,

    // Connection Pool
    pool: PostgresqlPoolSetting,
}

/// Postgresql connection setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlConnectionSetting {
    address: Vec<String>,
    database: String,
    username: String,
    password: String,
}

/// Postgresql timeout configuration (seconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlTimeoutSetting {
    /// Seconds
    connection_timeout: u64,
    /// Seconds
    statement_timeout: u64,
    /// Seconds
    idle_in_transaction_timeout: u64,
}

/// Postgresql connection pool configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlPoolSetting {
    max_conns: u32,
    min_conns: u32,
    min_idle_conns: u32,
    /// Seconds
    max_conn_idle_time: u64,
    /// Seconds
    max_conn_lifetime: u64,
    /// Seconds
    max_conn_lifetime_jitter: u64,
    /// Seconds
    health_check_period: u64,
}
