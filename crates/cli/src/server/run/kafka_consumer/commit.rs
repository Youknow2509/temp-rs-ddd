use rdkafka::consumer::{CommitMode, Consumer};
use rdkafka::{Offset, TopicPartitionList};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CommitAck {
    pub topic: String,
    pub partition: i32,
    pub next_offset: i64,
}

#[derive(Debug, Default)]
pub struct CommitState {
    pending: HashMap<(String, i32), i64>,
}

impl CommitState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, ack: CommitAck) {
        self.pending
            .insert((ack.topic, ack.partition), ack.next_offset);
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    pub fn flush(&mut self, consumer: &rdkafka::consumer::StreamConsumer) {
        if self.pending.is_empty() {
            return;
        }

        let mut tpl = TopicPartitionList::new();
        for ((topic, partition), next_offset) in self.pending.drain() {
            if let Err(err) =
                tpl.add_partition_offset(&topic, partition, Offset::Offset(next_offset))
            {
                tracing::error!(
                    topic = %topic,
                    partition,
                    next_offset,
                    error = ?err,
                    "failed to add partition offset to commit list"
                );
            }
        }

        if let Err(err) = consumer.commit(&tpl, CommitMode::Async) {
            tracing::error!(error = ?err, "failed to commit kafka offsets");
        }
    }
}
