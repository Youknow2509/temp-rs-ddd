//! Phase 3: start every inbound interface backed by the wired application.
//!
//! In the real implementation each `start` will spawn a long-running async
//! task; here they are sync stubs that return immediately so the orchestration
//! shape is visible end-to-end.

pub mod grpc;
pub mod http;
pub mod kafka_consumer;
pub mod websocket;

use anyhow::Result;

use super::wiring::Wired;

pub fn start(wired: &Wired) -> Result<()> {
    http::start(wired)?;
    grpc::start(wired)?;
    websocket::start(wired)?;
    kafka_consumer::start(wired)?;
    Ok(())
}
