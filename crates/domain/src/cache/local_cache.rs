use std::time::Duration;

use serde::{Serialize, de::DeserializeOwned};

/// Local in-process cache abstraction (e.g. Moka, DashMap, LRU).
///
/// Keys are plain UTF-8 strings. Values are (de)serialized by the implementor.
/// Unlike [`super::DistributedCache`], operations here are synchronous and
/// never cross a network boundary.
#[allow(async_fn_in_trait)]
pub trait LocalCache: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    // --- Basic get / set / delete ---

    /// Return the value stored at `key`, or `None` if absent or expired.
    async fn get<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Store `value` at `key`. `ttl = None` means the entry never expires.
    async fn set<V>(&self, key: &str, value: &V, ttl: Option<Duration>) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Remove `key`. Returns `true` if the key existed.
    async fn delete(&self, key: &str) -> Result<bool, Self::Error>;

    // --- Key metadata ---

    /// Return `true` if `key` exists and has not expired.
    async fn exists(&self, key: &str) -> Result<bool, Self::Error>;

    /// Remaining TTL of `key`. `None` when the key has no expiry or is absent.
    async fn ttl(&self, key: &str) -> Result<Option<Duration>, Self::Error>;

    // --- Atomic counter ---

    /// Atomically increment the integer at `key` by `delta`.
    /// The key is created at 0 before the increment when absent.
    /// Returns the value *after* the increment.
    async fn increment(&self, key: &str, delta: i64) -> Result<i64, Self::Error>;

    /// Atomically decrement the integer at `key` by `delta`.
    /// Returns the value *after* the decrement.
    async fn decrement(&self, key: &str, delta: i64) -> Result<i64, Self::Error>;

    // --- Conditional set ---

    /// Store `value` only if `key` does not already exist.
    /// Returns `true` when the key was written.
    async fn set_if_not_exists<V>(
        &self,
        key: &str,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Fetch `key`; if absent, call `f`, store the result with `ttl`, return it.
    async fn get_or_set<V, F, Fut>(
        &self,
        key: &str,
        ttl: Option<Duration>,
        f: F,
    ) -> Result<V, Self::Error>
    where
        V: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<V, Self::Error>> + Send;

    // --- Batch ---

    /// Fetch multiple keys. The returned `Vec` has the same length as `keys`;
    /// entries are `None` for missing or expired keys.
    async fn get_many<V>(&self, keys: &[&str]) -> Result<Vec<Option<V>>, Self::Error>
    where
        V: DeserializeOwned;

    /// Store multiple key-value pairs. All pairs share the same `ttl`.
    async fn set_many<V>(
        &self,
        pairs: &[(&str, &V)],
        ttl: Option<Duration>,
    ) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Remove multiple keys. Returns the count of keys that existed.
    async fn delete_many(&self, keys: &[&str]) -> Result<u64, Self::Error>;
}
