use super::commit::{CommitAck, CommitState};
use super::message::message_to_mq_message;
use super::pipeline::{TopicPipelines, build_topic_pipelines};
use anyhow::Result;
use infrastructure::state::AppState;
use rdkafka::consumer::Consumer;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;

pub fn start(
    app_state: &Arc<AppState>,
    mut shutdown_rx: watch::Receiver<bool>,
) -> Result<JoinHandle<()>> {
    let state = Arc::clone(app_state);
    Ok(tokio::spawn(async move {
        let consumer = &state.connections.kafka.consumer;
        let kafka_consumer_cfg = &state.config.interfaces.kafka_consumer;

        let topics_cfg = match kafka_consumer_cfg.topics.as_ref() {
            Some(topics) if !topics.is_empty() => topics,
            _ => {
                tracing::warn!(
                    "No topics configured for kafka_consumer.topics — consumer not started"
                );
                return;
            }
        };

        let (ack_tx, mut ack_rx) = mpsc::channel::<CommitAck>(2048);
        let mut commit_state = CommitState::new();
        let mut flush_interval = tokio::time::interval(Duration::from_millis(100));
        let mut shutting_down = false;

        let TopicPipelines {
            senders: topic_senders,
            topic_names,
            handles: pipeline_handles,
        } = build_topic_pipelines(Arc::clone(&state), topics_cfg, ack_tx);
        let mut topic_senders = Some(topic_senders);

        let topic_name_refs: Vec<&str> = topic_names.iter().map(|s| s.as_str()).collect();
        if let Err(err) = consumer.subscribe(&topic_name_refs) {
            tracing::error!(error = ?err, "Kafka subscribe failed");
            return;
        }

        loop {
            tokio::select! {
                _ = shutdown_rx.changed(), if !shutting_down => {
                    if *shutdown_rx.borrow() {
                        tracing::info!("kafka consumer shutdown requested — unsubscribing and draining");
                        shutting_down = true;
                        consumer.unsubscribe();
                        drop(topic_senders.take());
                    }
                }
                maybe_ack = ack_rx.recv() => {
                    match maybe_ack {
                        Some(ack) => {
                            commit_state.record(ack);
                            if commit_state.pending_len() >= 64 {
                                commit_state.flush(consumer);
                            }
                        }
                        None => {
                            commit_state.flush(consumer);
                            break;
                        }
                    }
                }
                _ = flush_interval.tick() => {
                    commit_state.flush(consumer);
                }
                msg_res = consumer.recv(), if !shutting_down => {
                    match msg_res {
                        Ok(m) => {
                            let mq = message_to_mq_message(&m);
                            let topic = mq.topic.clone();

                            match topic_senders.as_ref().and_then(|senders| senders.get(&topic)) {
                                Some(topic_tx) => {
                                    if let Err(err) = topic_tx.send(mq).await {
                                        tracing::error!(
                                            topic = %topic,
                                            error = ?err,
                                            "failed to enqueue MQMessage into topic queue"
                                        );
                                    }
                                }
                                None => {
                                    tracing::warn!(
                                        "No handler configured for topic '{}', dropping message",
                                        topic
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!(error = ?e, "Kafka message error");
                        }
                    }
                }
            }
        }

        // Allow worker/router tasks to finish draining any queued messages.
        drop(topic_senders);
        for handle in pipeline_handles {
            let _ = handle.await;
        }

        commit_state.flush(consumer);
    }))
}
