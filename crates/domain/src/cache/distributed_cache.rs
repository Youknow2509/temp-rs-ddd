use std::time::Duration;

use serde::{Serialize, de::DeserializeOwned};

/// Distributed cache abstraction (e.g. Redis, Memcached).
///
/// All keys are plain UTF-8 strings. Values are (de)serialized by the
/// implementor — callers work with typed Rust values.
#[allow(async_fn_in_trait)]
pub trait DistributedCache: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    // --- String: basic get / set / delete ---

    /// Return the value stored at `key`, or `None` if the key does not exist.
    async fn get<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Store `value` at `key`.  `ttl = None` means the key never expires.
    async fn set<V>(&self, key: &str, value: &V, ttl: Option<Duration>) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Delete `key`. Returns `true` if the key existed.
    async fn delete(&self, key: &str) -> Result<bool, Self::Error>;

    // --- Key metadata ---

    /// Return `true` if `key` exists.
    async fn exists(&self, key: &str) -> Result<bool, Self::Error>;

    /// Set or update the expiry of an existing key.
    /// Returns `true` if the key existed and the TTL was applied.
    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool, Self::Error>;

    /// Remove the expiry from a key so it persists indefinitely.
    /// Returns `true` if the key existed and had a TTL that was removed.
    async fn persist(&self, key: &str) -> Result<bool, Self::Error>;

    /// Remaining TTL of a key. `None` when the key has no expiry or does not exist.
    async fn ttl(&self, key: &str) -> Result<Option<Duration>, Self::Error>;

    // --- String: atomic counters ---

    /// Atomically increment the integer at `key` by `delta`.
    /// The key is created at 0 before the increment when absent.
    /// Returns the value *after* the increment.
    async fn increment(&self, key: &str, delta: i64) -> Result<i64, Self::Error>;

    /// Atomically decrement the integer at `key` by `delta`.
    /// Returns the value *after* the decrement.
    async fn decrement(&self, key: &str, delta: i64) -> Result<i64, Self::Error>;

    // --- String: conditional set ---

    /// Store `value` only if `key` does not already exist (SET NX).
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

    // --- String: batch ---

    /// Fetch multiple keys in one round-trip (MGET).
    /// The returned `Vec` has the same length as `keys`; entries are `None`
    /// for missing keys.
    async fn get_many<V>(&self, keys: &[&str]) -> Result<Vec<Option<V>>, Self::Error>
    where
        V: DeserializeOwned;

    /// Store multiple key-value pairs in one round-trip (MSET).
    /// All pairs share the same `ttl`.
    async fn set_many<V>(
        &self,
        pairs: &[(&str, &V)],
        ttl: Option<Duration>,
    ) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Delete multiple keys in one round-trip.
    /// Returns the count of keys that existed and were removed.
    async fn delete_many(&self, keys: &[&str]) -> Result<u64, Self::Error>;

    // --- List (LPUSH / RPUSH / LPOP / RPOP / LRANGE / LLEN) ---

    /// Prepend `value` to the list at `key` (LPUSH).
    /// Returns the length of the list after the operation.
    async fn list_push_front<V>(&self, key: &str, value: &V) -> Result<u64, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Append `value` to the list at `key` (RPUSH).
    /// Returns the length of the list after the operation.
    async fn list_push_back<V>(&self, key: &str, value: &V) -> Result<u64, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Remove and return the first element of the list at `key` (LPOP).
    async fn list_pop_front<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Remove and return the last element of the list at `key` (RPOP).
    async fn list_pop_back<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return a slice of the list from `start` to `stop` inclusive (LRANGE).
    /// Negative indices count from the tail: -1 is the last element.
    async fn list_range<V>(&self, key: &str, start: i64, stop: i64) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return the length of the list at `key` (LLEN).
    async fn list_len(&self, key: &str) -> Result<u64, Self::Error>;

    // --- Set (SADD / SREM / SMEMBERS / SISMEMBER / SCARD) ---

    /// Add `member` to the set at `key` (SADD).
    /// Returns `true` if the member was new.
    async fn set_add<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Remove `member` from the set at `key` (SREM).
    /// Returns `true` if the member existed.
    async fn set_remove<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Return `true` if `member` belongs to the set at `key` (SISMEMBER).
    async fn set_is_member<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Return all members of the set at `key` (SMEMBERS).
    async fn set_members<V>(&self, key: &str) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return the cardinality (size) of the set at `key` (SCARD).
    async fn set_card(&self, key: &str) -> Result<u64, Self::Error>;

    // --- Hash (HSET / HGET / HDEL / HGETALL / HEXISTS / HLEN) ---

    /// Set `field` in the hash at `key` to `value` (HSET).
    /// Returns `true` if the field was new.
    async fn hash_set<V>(&self, key: &str, field: &str, value: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Set multiple fields in the hash at `key` in one round-trip (HMSET).
    async fn hash_set_many<V>(&self, key: &str, fields: &[(&str, &V)]) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Return the value of `field` in the hash at `key` (HGET).
    async fn hash_get<V>(&self, key: &str, field: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return values for multiple fields in one round-trip (HMGET).
    async fn hash_get_many<V>(
        &self,
        key: &str,
        fields: &[&str],
    ) -> Result<Vec<Option<V>>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return all field-value pairs in the hash at `key` (HGETALL).
    async fn hash_get_all<V>(&self, key: &str) -> Result<Vec<(String, V)>, Self::Error>
    where
        V: DeserializeOwned;

    /// Delete `field` from the hash at `key` (HDEL).
    /// Returns `true` if the field existed.
    async fn hash_delete(&self, key: &str, field: &str) -> Result<bool, Self::Error>;

    /// Return `true` if `field` exists in the hash at `key` (HEXISTS).
    async fn hash_exists(&self, key: &str, field: &str) -> Result<bool, Self::Error>;

    /// Return the number of fields in the hash at `key` (HLEN).
    async fn hash_len(&self, key: &str) -> Result<u64, Self::Error>;

    // --- Sorted set (ZADD / ZRANGE / ZRANK / ZREM / ZSCORE / ZCARD) ---

    /// Add `member` with `score` to the sorted set at `key` (ZADD).
    /// Returns `true` if the member was new.
    async fn zset_add<V>(&self, key: &str, member: &V, score: f64) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Remove `member` from the sorted set at `key` (ZREM).
    /// Returns `true` if the member existed.
    async fn zset_remove<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Return `score` of `member` in the sorted set at `key` (ZSCORE).
    async fn zset_score<V>(&self, key: &str, member: &V) -> Result<Option<f64>, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Return the rank (0-based, ascending) of `member` (ZRANK).
    async fn zset_rank<V>(&self, key: &str, member: &V) -> Result<Option<u64>, Self::Error>
    where
        V: Serialize + Send + Sync;

    /// Return members in sorted order from `start` to `stop` (ZRANGE).
    async fn zset_range<V>(&self, key: &str, start: i64, stop: i64) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return members with scores in `[min, max]` (ZRANGEBYSCORE).
    async fn zset_range_by_score<V>(
        &self,
        key: &str,
        min: f64,
        max: f64,
    ) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned;

    /// Return the cardinality of the sorted set at `key` (ZCARD).
    async fn zset_card(&self, key: &str) -> Result<u64, Self::Error>;

    // --- Lua scripting ---

    /// Execute a Lua `script` server-side (EVAL).
    ///
    /// - `keys`  — KEYS array passed to the script (1-indexed as KEYS[1]…)
    /// - `args`  — ARGV array passed to the script (1-indexed as ARGV[1]…)
    ///
    /// Returns the script's return value deserialized into `V`.
    async fn eval<V>(&self, script: &str, keys: &[&str], args: &[&str]) -> Result<V, Self::Error>
    where
        V: DeserializeOwned;

    /// Execute a pre-loaded Lua script by its SHA1 digest (EVALSHA).
    ///
    /// Use this when the same script is called frequently to avoid sending
    /// the full script body on every call.
    async fn eval_sha<V>(&self, sha: &str, keys: &[&str], args: &[&str]) -> Result<V, Self::Error>
    where
        V: DeserializeOwned;

    /// Load a Lua `script` into the server script cache (SCRIPT LOAD).
    /// Returns the SHA1 digest that can be passed to `eval_sha`.
    async fn script_load(&self, script: &str) -> Result<String, Self::Error>;

    // --- Health ---

    /// Check whether the cache backend is reachable (PING).
    async fn ping(&self) -> Result<(), Self::Error>;
}
