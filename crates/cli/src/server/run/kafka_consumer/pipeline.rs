use std::collections::HashMap;
use std::sync::Arc;

use domain::config::KafkaTopicSetting;
use infrastructure::state::AppState;
use interface::mq::handler;
use interface::mq::message::MQMessage;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use super::commit::CommitAck;

pub struct TopicPipelines {
    pub senders: HashMap<String, mpsc::Sender<MQMessage>>,
    pub topic_names: Vec<String>,
    pub handles: Vec<JoinHandle<()>>,
}

pub fn build_topic_pipelines(
    state: Arc<AppState>,
    topics_cfg: &[KafkaTopicSetting],
    ack_tx: mpsc::Sender<CommitAck>,
) -> TopicPipelines {
    let mut topic_senders: HashMap<String, mpsc::Sender<MQMessage>> = HashMap::new();
    let mut topic_names: Vec<String> = Vec::with_capacity(topics_cfg.len());
    let mut handles: Vec<JoinHandle<()>> = Vec::new();

    for topic_cfg in topics_cfg.iter() {
        tracing::info!(
            topic = %topic_cfg.name,
            handler = %topic_cfg.handler,
            workers = topic_cfg.workers,
            buffer_size = topic_cfg.buffer_size,
            "setting up pipeline for Kafka topic"
        );
        let topic_name = topic_cfg.name.clone();
        let handler_name = if topic_cfg.handler.is_empty() {
            topic_name.clone()
        } else {
            topic_cfg.handler.clone()
        };
        let worker_count = topic_cfg.workers.max(1);
        let topic_buffer = topic_cfg.buffer_size.max(1);

        topic_names.push(topic_name.clone());

        let (topic_tx, mut topic_rx) = mpsc::channel::<MQMessage>(topic_buffer);
        topic_senders.insert(topic_name.clone(), topic_tx);

        let mut worker_senders = Vec::with_capacity(worker_count);
        let worker_buffer = std::cmp::max(1, topic_buffer / worker_count);

        for _worker_idx in 0..worker_count {
            let (worker_tx, mut worker_rx) = mpsc::channel::<MQMessage>(worker_buffer);
            worker_senders.push(worker_tx);

            let ack_tx = ack_tx.clone();
            let handler_name = handler_name.clone();
            let app_state = state.clone();
            handles.push(tokio::spawn(async move {
                while let Some(msg) = worker_rx.recv().await {
                    let topic = msg.topic.clone();
                    let partition = msg.partition;
                    let next_offset = msg.offset + 1;

                    match handler::dispatch(&handler_name, app_state.clone(), msg).await {
                        Ok(()) => {
                            if let Err(err) = ack_tx
                                .send(CommitAck {
                                    topic,
                                    partition,
                                    next_offset,
                                })
                                .await
                            {
                                tracing::error!(error = ?err, "failed to send commit ack");
                            }
                        }
                        Err(err) => {
                            tracing::error!(
                                topic = %topic,
                                partition,
                                offset = next_offset - 1,
                                error = ?err,
                                "handler failed; offset will not be committed"
                            );
                        }
                    }
                }
            }));
        }

        handles.push(tokio::spawn(async move {
            while let Some(msg) = topic_rx.recv().await {
                let worker_idx = (msg.partition as usize) % worker_senders.len();
                if let Err(err) = worker_senders[worker_idx].send(msg).await {
                    tracing::error!(error = ?err, "failed to route MQMessage to worker queue");
                }
            }
        }));
    }

    TopicPipelines {
        senders: topic_senders,
        topic_names,
        handles,
    }
}
