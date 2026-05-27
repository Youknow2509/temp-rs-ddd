//! Phase 2: start every inbound interface backed by the bootstrapped application.

pub mod grpc;
pub mod http;
pub mod kafka_consumer;
pub mod websocket;

use std::sync::Arc;

use anyhow::Result;
use interface::state::AppState;
use tokio::{sync::watch, task::JoinHandle};

#[derive(Debug)]
pub struct RunHandles {
    pub http: Option<JoinHandle<()>>,
    pub grpc: Option<JoinHandle<()>>,
    pub websocket: Option<JoinHandle<()>>,
    pub kafka_consumer: Option<JoinHandle<()>>,
    pub kafka_shutdown: watch::Sender<bool>,
}

impl RunHandles {
    pub fn request_stop(&self) {
        if let Some(http) = &self.http {
            http.abort();
        }
        if let Some(websocket) = &self.websocket {
            websocket.abort();
        }
        let _ = self.kafka_shutdown.send(true);
        if let Some(grpc) = &self.grpc {
            grpc.abort();
        }
    }

    pub async fn wait_all(&mut self) {
        if let Some(http) = self.http.take() {
            let _ = http.await;
            tracing::info!("HTTP server stopped");
        }
        if let Some(websocket) = self.websocket.take() {
            let _ = websocket.await;
            tracing::info!("WebSocket server stopped");
        }
        if let Some(kafka_consumer) = self.kafka_consumer.take() {
            let _ = kafka_consumer.await;
            tracing::info!("Kafka consumer stopped");
        }
        if let Some(grpc) = self.grpc.take() {
            let _ = grpc.await;
            tracing::info!("gRPC server stopped");
        }
    }
}

pub fn start(app_state: &Arc<AppState>) -> Result<RunHandles> {
    let (kafka_shutdown, kafka_shutdown_rx) = watch::channel(false);

    Ok(RunHandles {
        http: Some(http::start(app_state)?),
        grpc: Some(grpc::start(app_state)?),
        websocket: Some(websocket::start(app_state)?),
        kafka_consumer: Some(kafka_consumer::start(app_state, kafka_shutdown_rx)?),
        kafka_shutdown,
    })
}
