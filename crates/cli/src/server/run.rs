//! Phase 2: start every inbound interface backed by the bootstrapped application.

pub mod grpc;
pub mod http;
pub mod kafka_consumer;
pub mod websocket;

use std::sync::Arc;

use anyhow::Result;
use interface::state::AppState;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct RunHandles {
    pub http: JoinHandle<()>,
    pub grpc: JoinHandle<()>,
    pub websocket: JoinHandle<()>,
    pub kafka_consumer: JoinHandle<()>,
}

impl RunHandles {
    pub fn stop_all(self) {
        self.http.abort();
        self.grpc.abort();
        self.websocket.abort();
        self.kafka_consumer.abort();
    }
}

pub fn start(app_state: &Arc<AppState>) -> Result<RunHandles> {
    Ok(RunHandles {
        http: http::start(app_state)?,
        grpc: grpc::start(app_state)?,
        websocket: websocket::start(app_state)?,
        kafka_consumer: kafka_consumer::start(app_state)?,
    })
}
