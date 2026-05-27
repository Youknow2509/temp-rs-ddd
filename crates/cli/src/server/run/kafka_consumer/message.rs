use bytes::Bytes;
use interface::mq::message::MQMessage;
use rdkafka::message::{BorrowedMessage, Headers, Message};

pub fn message_to_mq_message(msg: &BorrowedMessage<'_>) -> MQMessage {
    let mut headers = Vec::new();
    if let Some(hdrs) = msg.headers() {
        for i in 0..hdrs.count() {
            let header = hdrs.get(i);
            let bytes = header
                .value
                .map(Bytes::copy_from_slice)
                .unwrap_or_else(Bytes::new);
            headers.push((header.key.to_string(), bytes));
        }
    }

    MQMessage {
        topic: msg.topic().to_string(),
        partition: msg.partition(),
        offset: msg.offset(),
        key: msg.key().map(Bytes::copy_from_slice),
        value: msg.payload().map(Bytes::copy_from_slice),
        headers,
    }
}
