//! Kafka consumer runtime — drives event handlers off subscribed topics.

use std::sync::Arc;

use anyhow::Result;
use interface::state::AppState;
use tokio::task::JoinHandle;

pub fn start(app_state: &Arc<AppState>) -> Result<JoinHandle<()>> {
    let _state = Arc::clone(app_state);
    // TODO: build rdkafka consumer from `_state.config.interfaces.kafka_consumer`,
    // spawn the poll loop that dispatches messages via use case factory methods
    // borrowing from `_state.connections`.
    Ok(tokio::spawn(async {
        // placeholder — real impl blocks here until shutdown signal
    }))
}
