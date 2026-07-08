use crate::mq::dto::Topic02Dto;
use crate::mq::message::MQMessage;
use anyhow::Result;
use infrastructure::state::AppState;
use std::sync::Arc;

pub async fn handle(_app_state: Arc<AppState>, msg: MQMessage) -> Result<()> {
    let dto: Topic02Dto = msg.deserialize_json()?;
    tracing::info!(
        topic = %msg.topic,
        example02 = %dto.example01,
        "topic02 mapped MQMessage to DTO"
    );
    Ok(())
}
