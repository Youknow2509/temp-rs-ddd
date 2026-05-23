//! Kafka consumer runtime — drives event handlers off subscribed topics.

use anyhow::Result;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<()> {
    // TODO: build rdkafka consumer from
    // `wired.bootstrap.config.interfaces.kafka_consumer`, spawn the poll loop
    // that dispatches messages to `wired.use_cases`.
    println!("[run::kafka_consumer] (stub) Kafka consumer would start here");
    Ok(())
}
