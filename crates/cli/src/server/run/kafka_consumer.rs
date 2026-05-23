//! Kafka consumer runtime — drives event handlers off subscribed topics.

use anyhow::Result;
use tokio::task::JoinHandle;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<JoinHandle<()>> {
    // TODO: build rdkafka consumer from `wired.config.interfaces.kafka_consumer`,
    // spawn the poll loop that dispatches messages to `wired.use_cases`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
