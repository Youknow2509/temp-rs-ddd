use super::TLSSetting;
use serde::Deserialize;

/// Kafka Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaSetting {
    // Shared connection config
    pub connection: KafkaConnectionSetting,

    // Producer config
    pub producer: Option<KafkaProducerSetting>,

    // Consumer config
    pub consumer: Option<KafkaConsumerSetting>,
}

// ===
// Connection (shared)
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConnectionSetting {
    // Cluster
    pub cluster: KafkaClusterSetting,

    // Timeouts
    pub timeouts: KafkaConnectionTimeoutSetting,

    // Retry / Reconnect
    pub retry: KafkaReconnectSetting,

    // Security
    pub sasl: Option<SaslSetting>,
    pub tls: Option<TLSSetting>,
}

/// Kafka cluster identification
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaClusterSetting {
    pub brokers: Vec<String>,
    pub client_id: String,
}

/// Kafka connection timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConnectionTimeoutSetting {
    /// Milliseconds
    pub connection_timeout_ms: u64,
    /// Milliseconds
    pub request_timeout_ms: u64,
    /// Milliseconds
    pub socket_timeout_ms: u64,
}

/// Kafka retry / reconnect backoff (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaReconnectSetting {
    /// Milliseconds
    pub retry_backoff_ms: u64,
    /// Milliseconds
    pub reconnect_backoff_ms: u64,
    /// Milliseconds
    pub reconnect_backoff_max_ms: u64,
}

// ===
// Producer
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerSetting {
    // Reliability
    pub reliability: KafkaProducerReliabilitySetting,

    // Batching (performance)
    pub batching: KafkaProducerBatchingSetting,

    // Compression
    pub compression_type: String, // "none" | "gzip" | "snappy" | "lz4" | "zstd"

    // Partitioning
    pub partitioner: String, // "default" | "round_robin" | "uniform_sticky"

    // Transaction (optional - exactly-once across topics)
    pub transaction: Option<KafkaProducerTransactionSetting>,
}

/// Producer reliability / delivery guarantees
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerReliabilitySetting {
    pub acks: String, // "0" | "1" | "all"
    pub enable_idempotence: bool,
    pub max_in_flight_requests_per_connection: u32,
    pub retries: u32,
    /// Milliseconds
    pub delivery_timeout_ms: u64,
}

/// Producer batching configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerBatchingSetting {
    pub batch_size: usize,
    /// Milliseconds
    pub linger_ms: u64,
    pub buffer_memory: usize,
    pub max_request_size: usize,
}

/// Producer transactional configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerTransactionSetting {
    pub transactional_id: String,
    /// Milliseconds
    pub transaction_timeout_ms: u64,
}

// ===
// Consumer
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerSetting {
    // Group
    pub group: KafkaConsumerGroupSetting,

    // Offset
    pub offset: KafkaConsumerOffsetSetting,

    // Fetch (performance)
    pub fetch: KafkaConsumerFetchSetting,

    // Session
    pub session: KafkaConsumerSessionSetting,

    // Concurrency
    pub concurrency: KafkaConsumerConcurrencySetting,

    // Per-topic overrides (optional). Use to tune workers, channel buffer and
    // handler mapping per topic. If absent, consumer-level concurrency applies.
    pub topics: Option<Vec<KafkaTopicSetting>>,
}

/// Consumer group identification
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerGroupSetting {
    pub group_id: String,
    pub group_instance_id: Option<String>,
}

/// Consumer offset management
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerOffsetSetting {
    pub auto_offset_reset: String, // "earliest" | "latest" | "none"
    pub enable_auto_commit: bool,
    /// Milliseconds
    pub auto_commit_interval_ms: u64,
    pub isolation_level: String, // "read_committed" | "read_uncommitted"
}

/// Consumer fetch tuning
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerFetchSetting {
    pub fetch_min_bytes: usize,
    pub fetch_max_bytes: usize,
    /// Milliseconds
    pub fetch_max_wait_ms: u64,
    pub max_partition_fetch_bytes: usize,
    pub max_poll_records: u32,
    /// Milliseconds
    pub max_poll_interval_ms: u64,
}

/// Consumer session / heartbeat
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerSessionSetting {
    /// Milliseconds
    pub session_timeout_ms: u64,
    /// Milliseconds
    pub heartbeat_interval_ms: u64,
}

/// Consumer concurrency settings
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerConcurrencySetting {
    /// Number of parallel message processing tasks
    pub worker_threads: usize,
    /// Buffer size between fetch and processing
    pub channel_buffer_size: usize,
}

/// Per-topic consumer configuration allowing overrides for worker count,
/// channel buffer size and handler mapping. All fields are optional so the
/// global `KafkaConsumerSetting` values can be used as defaults.
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaTopicSetting {
    /// Topic name
    pub name: String,

    /// Number of workers to spawn for this topic. If `None`, use global
    /// `concurrency.worker_threads`.
    pub workers: usize,

    /// Buffer size for the mpsc channel queued between consumer and workers.
    /// If `None`, use global `concurrency.channel_buffer_size`.
    pub buffer_size: usize,

    /// Handler identifier to map the topic to a specific handler
    /// function/module in your application (e.g. "handle_topic_a").
    pub handler: String,

    /// Override for auto-commit behavior for this topic. If `Some`,
    /// it overrides the global `offset.enable_auto_commit` for messages from
    /// this topic (implementation must respect this flag when committing).
    pub enable_auto_commit: bool,
}

// ===
// SASL
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SaslSetting {
    pub mechanism: String, // "PLAIN" | "SCRAM-SHA-256" | "SCRAM-SHA-512"
    pub username: String,
    pub password: String,
}
