use super::TLSSetting;
use serde::Deserialize;

/// Repository System Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RepositorySystemSetting {
    pub postgresql: PostgresqlSettingRepository,
    pub redis: RedisSettingRepository,
    pub object_storage: ObjectStorageSetting,
    pub scylladb: ScyllaDbSettingRepository,
    pub local_cache: LocalCacheSetting,
    pub qdrant: QdrantSettingRepository,
}

// ===
// Qdrant - Vector Database
// ===
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct QdrantSettingRepository {
    pub api_key: String,
    pub host: String,
    pub port: u16,
    pub tls: TLSSetting,

    pub collection: String,
    pub dimension: u64,

    // Timeouts
    pub timeout_ms: u64,
    pub connect_timeout_ms: u64,

    // Custom headers
    pub custom_headers: Vec<(String, String)>,

    // Pool settings
    pub pool_size: usize,

    // Other
    pub keep_alive_interval: bool,
    pub check_compatibility: bool,
    pub compression: Option<String>, // none | gzip
}

// ===
// Local Cache
// ===

/// Local in-process cache setting (e.g. Moka, LRU)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LocalCacheSetting {
    /// Maximum number of entries the cache may hold
    pub max_capacity: u64,

    /// Pre-allocated number of slots at startup (hint, not a hard limit)
    pub initial_capacity: u64,

    /// Global default TTL in seconds (0 = entries never expire)
    pub time_to_live_secs: u64,

    /// Evict an entry if it has not been accessed for this many seconds
    /// (0 = disabled)
    pub time_to_idle_secs: u64,

    pub eviction: LocalCacheEvictionSetting,
    pub concurrency: LocalCacheConcurrencySetting,
}

/// Eviction policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LocalCacheEvictionSetting {
    /// `lru` | `lfu` | `tinylfu`
    pub policy: String,
}

/// Concurrency / sharding
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LocalCacheConcurrencySetting {
    /// Number of internal shards for concurrent access (power of 2 recommended)
    pub num_segments: u32,
}

// ===
// ScyllaDB
// ===

/// ScyllaDB / Cassandra repository setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaDbSettingRepository {
    pub cluster: ScyllaClusterSetting,
    pub authentication: ScyllaAuthSetting,
    pub tls: TLSSetting,
    pub consistency: ScyllaConsistencySetting,
    pub timeouts: ScyllaTimeoutSetting,
    pub pool: ScyllaPoolSetting,
    pub retry: ScyllaRetrySetting,
    pub speculative: ScyllaSpeculativeSetting,
    pub load_balancing: ScyllaLoadBalancingSetting,
    pub reconnection: ScyllaReconnectionSetting,
    pub prepared: ScyllaPreparedSetting,
    pub query: ScyllaQuerySetting,
}

/// Cluster contact points and keyspace
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaClusterSetting {
    pub contact_points: Vec<String>,
    pub keyspace: String,
    pub local_dc: String,
    pub protocol_version: u8, // 3 | 4 | 5
    pub compression: String,  // none | snappy | lz4
}

/// Authentication credentials
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaAuthSetting {
    pub username: String,
    pub password: String,
}

/// Consistency levels
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaConsistencySetting {
    /// ANY | ONE | TWO | THREE | QUORUM | ALL | LOCAL_QUORUM | EACH_QUORUM | LOCAL_ONE
    pub default: String,
    /// SERIAL | LOCAL_SERIAL (for lightweight transactions)
    pub serial: String,
}

/// Timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaTimeoutSetting {
    /// Milliseconds
    pub connect_timeout_ms: u64,
    /// Milliseconds - query / request timeout
    pub request_timeout_ms: u64,
    /// Milliseconds - schema agreement timeout
    pub schema_agreement_timeout_ms: u64,
}

/// Connection pool (CQL native protocol model)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaPoolSetting {
    /// Connections per host in local DC (typically 1-4)
    pub connections_per_host_local: u32,
    /// Connections per host in remote DCs
    pub connections_per_host_remote: u32,
    /// Max concurrent requests per connection (CQL stream IDs, max 32768)
    pub max_requests_per_connection: u32,
    /// Milliseconds
    pub keepalive_interval_ms: u64,
}

/// Retry policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaRetrySetting {
    /// default | downgrading | fallthrough
    pub policy: String,
    pub max_retries: u32,
}

/// Speculative execution (tail latency optimization)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaSpeculativeSetting {
    pub enabled: bool,
    pub max_speculative_executions: u32,
    /// Milliseconds - delay before sending speculative query
    pub delay_ms: u64,
}

/// Load balancing policy
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaLoadBalancingSetting {
    /// round_robin | dc_aware | token_aware
    pub policy: String,
    pub shuffle_replicas: bool,
    pub allow_remote_dcs_for_local_cl: bool,
}

/// Reconnection policy when node goes down
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaReconnectionSetting {
    /// constant | exponential
    pub policy: String,
    /// Milliseconds
    pub base_delay_ms: u64,
    /// Milliseconds
    pub max_delay_ms: u64,
}

/// Prepared statement cache
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaPreparedSetting {
    pub cache_size: u32,
    pub prepare_on_all_hosts: bool,
}

/// Query default options
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ScyllaQuerySetting {
    pub page_size: u32,
    pub fetch_size: u32,
    pub tracing_enabled: bool,
}

// ===
// Object Storage
// ===

/// Object Storage for Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageSetting {
    pub r#type: String, // s3 | gcs | azure

    pub bucket_name: String,
    pub region: String,

    pub endpoint: String,
    pub force_path_style: bool,

    pub access_key_id: String,
    pub secret_access_key: String,
    pub session_token: String,

    pub tls: TLSSetting,

    pub timeouts: ObjectStorageTimeoutSetting,
    pub pool: ObjectStoragePoolSetting,
    pub retry: ObjectStorageRetrySetting,
    pub upload: ObjectStorageUploadSetting,
    pub download: ObjectStorageDownloadSetting,
    pub server_side: ObjectStorageServerSideSetting,
    pub options: ObjectStorageOptionSetting,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageTimeoutSetting {
    /// Milliseconds
    pub connect_timeout_ms: u64,
    /// Milliseconds
    pub request_timeout_ms: u64,
    /// Milliseconds
    pub operation_timeout_ms: u64,
    /// Milliseconds
    pub idle_timeout_ms: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStoragePoolSetting {
    pub max_idle_connections: u32,
    pub max_connections_per_host: u32,
    /// Milliseconds
    pub keep_alive_ms: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageRetrySetting {
    pub max_retries: u32,
    /// Milliseconds
    pub min_backoff_ms: u64,
    /// Milliseconds
    pub max_backoff_ms: u64,
    pub use_jitter: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageUploadSetting {
    pub multipart_threshold: u64,
    pub multipart_chunk_size: u64,
    pub multipart_concurrency: u32,
    pub max_object_size: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageDownloadSetting {
    pub chunk_size: u64,
    pub concurrency: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageServerSideSetting {
    pub encryption: String, // none | aes256 | aws:kms
    pub kms_key_id: String,
    pub storage_class: String,
    pub checksum_algorithm: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct ObjectStorageOptionSetting {
    pub enable_compression: bool,
    pub enable_acceleration: bool,
    pub addressing_style: String, // path | virtual
    pub user_agent: String,
}

// ===
// Redis
// ===

/// Redis for Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisSettingRepository {
    pub r#type: String, // standalone, sentinel, cluster

    // TLS
    pub tls: TLSSetting,

    // Auth
    pub username: String,
    pub password: String,
    pub db: u8,
    pub client_name: String,

    // Mode-specific
    pub standalone: Option<RedisStandaloneSetting>,
    pub sentinel: Option<RedisSentinelSetting>,
    pub cluster: Option<RedisClusterSetting>,

    // Shared settings
    pub timeouts: RedisTimeoutSetting,
    pub pool: RedisPoolSetting,
    pub retry: RedisRetrySetting,
}

/// Redis sub setting for standalone type
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisStandaloneSetting {
    pub host: String,
    pub port: u16,
}

/// Redis sub setting for sentinel type
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisSentinelSetting {
    pub master_name: String,
    pub sentinel_addrs: Vec<String>,
    pub sentinel_username: String,
    pub sentinel_password: String,
}

/// Redis sub setting for cluster type
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisClusterSetting {
    pub cluster_addrs: Vec<String>,
    pub route_by_latency: bool,
    pub route_randomly: bool,
    pub read_only: bool,
    pub max_redirects: u8,
}

/// Redis timeout configuration (seconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisTimeoutSetting {
    pub dial_timeout: u64,
    pub read_timeout: u64,
    pub write_timeout: u64,
    pub pool_timeout: u64,
}

/// Redis connection pool configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisPoolSetting {
    pub pool_size: u32,
    pub min_idle_conns: u32,
    pub max_idle_conns: u32,
    pub pool_fifo: bool,
    /// Seconds
    pub conn_max_idle_time: u64,
    /// Seconds
    pub conn_max_lifetime: u64,
}

/// Redis retry configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct RedisRetrySetting {
    pub max_retries: u32,
    /// Milliseconds
    pub min_retry_backoff: u64,
    /// Milliseconds
    pub max_retry_backoff: u64,
}

// ===
// PostgreSQL
// ===

/// Postgresql for Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlSettingRepository {
    // Connection
    pub connection: PostgresqlConnectionSetting,

    // SSL
    pub tls: TLSSetting,

    // Other
    pub appname: String,
    pub tz: String,

    // Timeouts
    pub timeouts: PostgresqlTimeoutSetting,

    // Connection Pool
    pub pool: PostgresqlPoolSetting,
}

/// Postgresql connection setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlConnectionSetting {
    pub address: Vec<String>,
    pub database: String,
    pub username: String,
    pub password: String,
}

/// Postgresql timeout configuration (seconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlTimeoutSetting {
    /// Seconds
    pub connection_timeout: u64,
    /// Seconds
    pub statement_timeout: u64,
    /// Seconds
    pub idle_in_transaction_timeout: u64,
}

/// Postgresql connection pool configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct PostgresqlPoolSetting {
    pub max_conns: u32,
    pub min_conns: u32,
    pub min_idle_conns: u32,
    /// Seconds
    pub max_conn_idle_time: u64,
    /// Seconds
    pub max_conn_lifetime: u64,
    /// Seconds
    pub max_conn_lifetime_jitter: u64,
    /// Seconds
    pub health_check_period: u64,
}
