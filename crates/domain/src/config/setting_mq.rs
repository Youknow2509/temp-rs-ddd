use super::TLSSetting;
use serde::Deserialize;

/// Kafka Setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaSetting {
    // Shared connection config
    connection: KafkaConnectionSetting,

    // Producer config
    producer: Option<KafkaProducerSetting>,

    // Consumer config
    consumer: Option<KafkaConsumerSetting>,
}

// ===
// Connection (shared)
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConnectionSetting {
    // Cluster
    cluster: KafkaClusterSetting,

    // Timeouts
    timeouts: KafkaConnectionTimeoutSetting,

    // Retry / Reconnect
    retry: KafkaReconnectSetting,

    // Security
    sasl: Option<SaslSetting>,
    tls: Option<TLSSetting>,
}

/// Kafka cluster identification
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaClusterSetting {
    brokers: Vec<String>,
    client_id: String,
}

/// Kafka connection timeouts (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConnectionTimeoutSetting {
    /// Milliseconds
    connection_timeout_ms: u64,
    /// Milliseconds
    request_timeout_ms: u64,
    /// Milliseconds
    metadata_timeout_ms: u64,
    /// Milliseconds
    socket_timeout_ms: u64,
}

/// Kafka retry / reconnect backoff (milliseconds)
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaReconnectSetting {
    /// Milliseconds
    retry_backoff_ms: u64,
    /// Milliseconds
    reconnect_backoff_ms: u64,
    /// Milliseconds
    reconnect_backoff_max_ms: u64,
}

// ===
// Producer
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerSetting {
    // Reliability
    reliability: KafkaProducerReliabilitySetting,

    // Batching (performance)
    batching: KafkaProducerBatchingSetting,

    // Compression
    compression_type: String, // "none" | "gzip" | "snappy" | "lz4" | "zstd"

    // Partitioning
    partitioner: String, // "default" | "round_robin" | "uniform_sticky"

    // Transaction (optional - exactly-once across topics)
    transaction: Option<KafkaProducerTransactionSetting>,
}

/// Producer reliability / delivery guarantees
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerReliabilitySetting {
    acks: String, // "0" | "1" | "all"
    enable_idempotence: bool,
    max_in_flight_requests_per_connection: u32,
    retries: u32,
    /// Milliseconds
    delivery_timeout_ms: u64,
}

/// Producer batching configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerBatchingSetting {
    batch_size: usize,
    /// Milliseconds
    linger_ms: u64,
    buffer_memory: usize,
    max_request_size: usize,
}

/// Producer transactional configuration
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaProducerTransactionSetting {
    transactional_id: String,
    /// Milliseconds
    transaction_timeout_ms: u64,
}

// ===
// Consumer
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerSetting {
    // Group
    group: KafkaConsumerGroupSetting,

    // Topics
    subscription: KafkaConsumerSubscriptionSetting,

    // Offset
    offset: KafkaConsumerOffsetSetting,

    // Fetch (performance)
    fetch: KafkaConsumerFetchSetting,

    // Session
    session: KafkaConsumerSessionSetting,

    // Concurrency
    concurrency: KafkaConsumerConcurrencySetting,
}

/// Consumer group identification
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerGroupSetting {
    group_id: String,
    group_instance_id: Option<String>,
}

/// Consumer topic subscription
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerSubscriptionSetting {
    topics: Vec<String>,
    topic_pattern: Option<String>,
}

/// Consumer offset management
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerOffsetSetting {
    auto_offset_reset: String, // "earliest" | "latest" | "none"
    enable_auto_commit: bool,
    /// Milliseconds
    auto_commit_interval_ms: u64,
    isolation_level: String, // "read_committed" | "read_uncommitted"
}

/// Consumer fetch tuning
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerFetchSetting {
    fetch_min_bytes: usize,
    fetch_max_bytes: usize,
    /// Milliseconds
    fetch_max_wait_ms: u64,
    max_partition_fetch_bytes: usize,
    max_poll_records: u32,
    /// Milliseconds
    max_poll_interval_ms: u64,
}

/// Consumer session / heartbeat
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerSessionSetting {
    /// Milliseconds
    session_timeout_ms: u64,
    /// Milliseconds
    heartbeat_interval_ms: u64,
}

/// Consumer concurrency settings
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct KafkaConsumerConcurrencySetting {
    /// Number of parallel message processing tasks
    worker_threads: usize,
    /// Buffer size between fetch and processing
    channel_buffer_size: usize,
}

// ===
// SASL
// ===

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SaslSetting {
    mechanism: String, // "PLAIN" | "SCRAM-SHA-256" | "SCRAM-SHA-512"
    username: String,
    password: String,
}
