//! Phase 3: start every inbound interface backed by the wired application.

pub mod grpc;
pub mod http;
pub mod kafka_consumer;
pub mod websocket;

use anyhow::Result;
use tokio::task::JoinHandle;

use super::wiring::Wired;

/// Handles returned by each interface so shutdown can stop them gracefully.
#[derive(Debug)]
pub struct RunHandles {
    pub http: JoinHandle<()>,
    pub grpc: JoinHandle<()>,
    pub websocket: JoinHandle<()>,
    pub kafka_consumer: JoinHandle<()>,
}

impl RunHandles {
    /// Abort all interface tasks immediately (best-effort graceful stop).
    pub fn stop_all(self) {
        self.http.abort();
        self.grpc.abort();
        self.websocket.abort();
        self.kafka_consumer.abort();
    }
}

pub fn start(wired: &Wired) -> Result<RunHandles> {
    Ok(RunHandles {
        http: http::start(wired)?,
        grpc: grpc::start(wired)?,
        websocket: websocket::start(wired)?,
        kafka_consumer: kafka_consumer::start(wired)?,
    })
}
