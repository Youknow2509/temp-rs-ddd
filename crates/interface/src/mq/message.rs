use bytes::Bytes;
use serde::de::DeserializeOwned;

/// Lightweight transport message used inside the `interface` layer.
/// Keeps raw bytes for key/value/headers and provides helpers to
/// decode on-demand.
#[derive(Debug, Clone)]
pub struct MQMessage {
    pub topic: String,
    pub partition: i32,
    pub offset: i64,
    pub key: Option<Bytes>,
    pub value: Option<Bytes>,
    pub headers: Vec<(String, Bytes)>,
}

impl MQMessage {
    pub fn value_as_str(&self) -> Option<&str> {
        self.value
            .as_ref()
            .and_then(|b| std::str::from_utf8(b).ok())
    }

    pub fn key_as_str(&self) -> Option<&str> {
        self.key.as_ref().and_then(|b| std::str::from_utf8(b).ok())
    }

    /// Deserialize value as JSON into a DTO type. Returns serde_json error
    /// if parsing fails or value is missing.
    pub fn deserialize_json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        let slice: &[u8] = self.value.as_deref().unwrap_or(&[]);
        serde_json::from_slice(slice)
    }
}
