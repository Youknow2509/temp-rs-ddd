use crate::mq::dto::Topic01Dto;
use crate::mq::message::MQMessage;
use anyhow::Result;
use infrastructure::state::AppState;
use std::sync::Arc;

pub async fn handle(_app_state: Arc<AppState>, msg: MQMessage) -> Result<()> {
    let dto: Topic01Dto = msg.deserialize_json()?;
    tracing::info!(
        topic = %msg.topic,
        example01 = %dto.example01,
        "topic01 mapped MQMessage to DTO"
    );
    Ok(())
}
