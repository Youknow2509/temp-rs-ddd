use anyhow::{Context, Result};
use domain::config::{KafkaConnectionSetting, KafkaConsumerSetting, KafkaProducerSetting};
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::FutureProducer;
use std::fmt;
use std::time::Duration;

pub type KafkaProducer = FutureProducer;
pub type KafkaConsumerClient = StreamConsumer;

pub struct KafkaClient {
    pub producer: KafkaProducer,
    pub consumer: KafkaConsumerClient,
}

impl fmt::Debug for KafkaClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KafkaClient")
            .field("producer", &"FutureProducer")
            .field("consumer", &"StreamConsumer")
            .finish()
    }
}

/// Build a Kafka client (producer + consumer) from split config sections.
///
/// Must be called from within a Tokio runtime — rdkafka's `tokio` feature
/// resolves `Handle::current()` at client creation time.
///
/// `conn`         — shared cluster / auth settings (`clients.kafka_publisher.connection`)
/// `producer_cfg` — producer-specific tuning (`clients.kafka_publisher.producer`)
/// `consumer_cfg` — consumer-specific tuning (`interfaces.kafka_consumer`)
pub async fn create_client(
    conn: &KafkaConnectionSetting,
    producer_cfg: &KafkaProducerSetting,
    consumer_cfg: &KafkaConsumerSetting,
) -> Result<KafkaClient> {
    let base = build_base_config(conn)?;

    let producer: FutureProducer = build_producer_config(&base, conn, producer_cfg)?
        .create()
        .context("creating Kafka FutureProducer")?;

    let consumer: StreamConsumer = build_consumer_config(&base, consumer_cfg)?
        .create()
        .context("creating Kafka StreamConsumer")?;

    // Ping reuses the already-created consumer — avoids an extra connection.
    // fetch_metadata is blocking, so block_in_place keeps the async thread yielding.
    ping(&consumer)
        .await
        .context("Kafka cluster health-check failed at startup")?;

    // Topic subscription is NOT done here — topics are code constants defined in
    // kafka_topics.rs and subscribed in the consumer run loop (kafka_consumer::start).

    Ok(KafkaClient { producer, consumer })
}

fn build_base_config(conn: &KafkaConnectionSetting) -> Result<ClientConfig> {
    let brokers = conn.cluster.brokers.join(",");

    let mut cfg = ClientConfig::new();
    cfg.set("bootstrap.servers", brokers.as_str())
        .set("client.id", conn.cluster.client_id.as_str())
        .set(
            "socket.timeout.ms",
            conn.timeouts.socket_timeout_ms.to_string(),
        )
        .set(
            "socket.connection.setup.timeout.ms",
            conn.timeouts.connection_timeout_ms.to_string(),
        )
        .set("retry.backoff.ms", conn.retry.retry_backoff_ms.to_string())
        .set(
            "reconnect.backoff.ms",
            conn.retry.reconnect_backoff_ms.to_string(),
        )
        .set(
            "reconnect.backoff.max.ms",
            conn.retry.reconnect_backoff_max_ms.to_string(),
        );

    apply_security(&mut cfg, conn)?;

    Ok(cfg)
}

fn apply_security(cfg: &mut ClientConfig, conn: &KafkaConnectionSetting) -> Result<()> {
    let has_sasl = conn.sasl.is_some();
    let has_tls = conn.tls.as_ref().is_some_and(|t| t.is_enabled);

    let protocol = match (has_sasl, has_tls) {
        (false, false) => "PLAINTEXT",
        (true, false) => "SASL_PLAINTEXT",
        (false, true) => "SSL",
        (true, true) => "SASL_SSL",
    };
    cfg.set("security.protocol", protocol);

    if let Some(sasl) = &conn.sasl {
        cfg.set("sasl.mechanism", sasl.mechanism.as_str())
            .set("sasl.username", sasl.username.as_str())
            .set("sasl.password", sasl.password.as_str());
    }

    if let Some(tls) = &conn.tls
        && tls.is_enabled
    {
        if let Some(ca) = &tls.client_ca_file {
            cfg.set("ssl.ca.location", ca.to_string_lossy().as_ref());
        }
        if !tls.cert_file.as_os_str().is_empty() {
            cfg.set(
                "ssl.certificate.location",
                tls.cert_file.to_string_lossy().as_ref(),
            );
        }
        if !tls.key_file.as_os_str().is_empty() {
            cfg.set("ssl.key.location", tls.key_file.to_string_lossy().as_ref());
        }
    }

    Ok(())
}

fn build_producer_config(
    base: &ClientConfig,
    conn: &KafkaConnectionSetting,
    prod: &KafkaProducerSetting,
) -> Result<ClientConfig> {
    let mut cfg = base.clone();

    // buffer_memory is in bytes; librdkafka wants KB
    #[allow(clippy::integer_division)]
    let buffer_kb = (prod.batching.buffer_memory / 1024).max(1);

    cfg.set(
        "request.timeout.ms",
        conn.timeouts.request_timeout_ms.to_string(),
    )
    .set("acks", prod.reliability.acks.as_str())
    .set(
        "enable.idempotence",
        prod.reliability.enable_idempotence.to_string(),
    )
    .set(
        "max.in.flight.requests.per.connection",
        prod.reliability
            .max_in_flight_requests_per_connection
            .to_string(),
    )
    .set("retries", prod.reliability.retries.to_string())
    .set(
        "delivery.timeout.ms",
        prod.reliability.delivery_timeout_ms.to_string(),
    )
    .set("batch.size", prod.batching.batch_size.to_string())
    .set("linger.ms", prod.batching.linger_ms.to_string())
    .set("queue.buffering.max.kbytes", buffer_kb.to_string())
    .set(
        "message.max.bytes",
        prod.batching.max_request_size.to_string(),
    )
    .set("compression.type", prod.compression_type.as_str())
    .set("partitioner", map_partitioner(prod.partitioner.as_str()));

    if let Some(tx) = &prod.transaction {
        cfg.set("transactional.id", tx.transactional_id.as_str())
            .set(
                "transaction.timeout.ms",
                tx.transaction_timeout_ms.to_string(),
            );
    }

    Ok(cfg)
}

fn build_consumer_config(base: &ClientConfig, cons: &KafkaConsumerSetting) -> Result<ClientConfig> {
    let mut cfg = base.clone();

    cfg.set("allow.auto.create.topics", "true")
        .set("group.id", cons.group.group_id.as_str())
        .set("auto.offset.reset", cons.offset.auto_offset_reset.as_str())
        .set(
            "enable.auto.commit",
            cons.offset.enable_auto_commit.to_string(),
        )
        .set(
            "auto.commit.interval.ms",
            cons.offset.auto_commit_interval_ms.to_string(),
        )
        .set("isolation.level", cons.offset.isolation_level.as_str())
        .set("fetch.min.bytes", cons.fetch.fetch_min_bytes.to_string())
        .set(
            "max.partition.fetch.bytes",
            cons.fetch.max_partition_fetch_bytes.to_string(),
        )
        .set(
            "fetch.wait.max.ms",
            cons.fetch.fetch_max_wait_ms.to_string(),
        )
        .set(
            "max.poll.interval.ms",
            cons.fetch.max_poll_interval_ms.to_string(),
        )
        .set(
            "session.timeout.ms",
            cons.session.session_timeout_ms.to_string(),
        )
        .set(
            "heartbeat.interval.ms",
            cons.session.heartbeat_interval_ms.to_string(),
        );

    if let Some(iid) = &cons.group.group_instance_id {
        cfg.set("group.instance.id", iid.as_str());
    }

    Ok(cfg)
}

/// Broker reachability check using the already-created consumer.
/// Uses block_in_place so the blocking fetch_metadata call does not starve
/// other tasks on the Tokio worker thread.
async fn ping(consumer: &StreamConsumer) -> Result<()> {
    tokio::task::block_in_place(|| {
        consumer
            .fetch_metadata(None, Duration::from_secs(5))
            .context("Kafka PING failed")
            .map(|_| ())
    })
}

fn map_partitioner(s: &str) -> &'static str {
    match s {
        "round_robin" => "random",
        "uniform_sticky" => "murmur2_random",
        _ => "consistent_random",
    }
}
