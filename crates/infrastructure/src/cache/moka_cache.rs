use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use bytes::Bytes;
use domain::cache::LocalCache;
use domain::config::LocalCacheSetting;
use moka::Expiry;
use moka::future::Cache;
use serde::Serialize;
use serde::de::DeserializeOwned;

// --- Error ---

#[derive(Debug)]
pub enum MokaCacheError {
    Serde(serde_json::Error),
    InvalidType,
}

impl fmt::Display for MokaCacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serde(e) => write!(f, "serialization error: {e}"),
            Self::InvalidType => write!(f, "value has wrong type or is not a counter"),
        }
    }
}

impl std::error::Error for MokaCacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Serde(e) => Some(e),
            Self::InvalidType => None,
        }
    }
}

// --- Cache entry ---

struct CacheEntry {
    data: Bytes,
    expires_at: Option<Instant>,
}

// --- Per-entry expiry ---

struct PerEntryExpiry;

impl Expiry<String, Arc<CacheEntry>> for PerEntryExpiry {
    fn expire_after_create(
        &self,
        _key: &String,
        value: &Arc<CacheEntry>,
        _created_at: Instant,
    ) -> Option<Duration> {
        value
            .expires_at
            .map(|t| t.saturating_duration_since(Instant::now()))
    }

    fn expire_after_update(
        &self,
        key: &String,
        value: &Arc<CacheEntry>,
        updated_at: Instant,
        _duration_until_expiry: Option<Duration>,
    ) -> Option<Duration> {
        self.expire_after_create(key, value, updated_at)
    }
}

// --- MokaCache ---

#[derive(Debug, Clone)]
pub struct MokaCache {
    inner: Cache<String, Arc<CacheEntry>>,
    default_ttl: Option<Duration>,
}

impl fmt::Debug for CacheEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CacheEntry")
            .field("data_len", &self.data.len())
            .field("expires_at", &self.expires_at)
            .finish()
    }
}

impl MokaCache {
    pub fn new(cfg: &LocalCacheSetting) -> Self {
        let default_ttl =
            (cfg.time_to_live_secs > 0).then(|| Duration::from_secs(cfg.time_to_live_secs));

        #[allow(clippy::cast_possible_truncation)]
        let mut builder = Cache::builder()
            .max_capacity(cfg.max_capacity)
            .initial_capacity(cfg.initial_capacity as usize)
            .expire_after(PerEntryExpiry);

        if let Some(ttl) = default_ttl {
            builder = builder.time_to_live(ttl);
        }
        if cfg.time_to_idle_secs > 0 {
            builder = builder.time_to_idle(Duration::from_secs(cfg.time_to_idle_secs));
        }

        Self {
            inner: builder.build(),
            default_ttl,
        }
    }
}

// --- Serialization helpers ---

fn ser<V: Serialize>(v: &V) -> Result<Bytes, MokaCacheError> {
    serde_json::to_vec(v)
        .map(Bytes::from)
        .map_err(MokaCacheError::Serde)
}

fn de<V: DeserializeOwned>(bytes: &[u8]) -> Result<V, MokaCacheError> {
    serde_json::from_slice(bytes).map_err(MokaCacheError::Serde)
}

fn make_entry(data: Bytes, ttl: Option<Duration>) -> Arc<CacheEntry> {
    Arc::new(CacheEntry {
        data,
        expires_at: ttl.map(|d| Instant::now() + d),
    })
}

// --- impl LocalCache ---

impl LocalCache for MokaCache {
    type Error = MokaCacheError;

    async fn get<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        match self.inner.get(key).await {
            None => Ok(None),
            Some(entry) => de(entry.data.as_ref()).map(Some),
        }
    }

    async fn set<V>(&self, key: &str, value: &V, ttl: Option<Duration>) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(value)?;
        let effective_ttl = ttl.or(self.default_ttl);
        self.inner
            .insert(key.to_owned(), make_entry(data, effective_ttl))
            .await;
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<bool, Self::Error> {
        let existed = self.inner.contains_key(key);
        self.inner.remove(key).await;
        Ok(existed)
    }

    async fn exists(&self, key: &str) -> Result<bool, Self::Error> {
        Ok(self.inner.contains_key(key))
    }

    async fn ttl(&self, key: &str) -> Result<Option<Duration>, Self::Error> {
        Ok(self.inner.get(key).await.and_then(|e| {
            e.expires_at
                .map(|t| t.saturating_duration_since(Instant::now()))
        }))
    }

    async fn increment(&self, key: &str, delta: i64) -> Result<i64, Self::Error> {
        let current: i64 = self
            .inner
            .get(key)
            .await
            .and_then(|e| de::<i64>(e.data.as_ref()).ok())
            .unwrap_or(0);
        let new_val = current + delta;
        let data = ser(&new_val)?;
        self.inner
            .insert(key.to_owned(), make_entry(data, self.default_ttl))
            .await;
        Ok(new_val)
    }

    async fn decrement(&self, key: &str, delta: i64) -> Result<i64, Self::Error> {
        self.increment(key, -delta).await
    }

    async fn set_if_not_exists<V>(
        &self,
        key: &str,
        value: &V,
        ttl: Option<Duration>,
    ) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(value)?;
        let effective_ttl = ttl.or(self.default_ttl);

        if self.inner.contains_key(key) {
            return Ok(false);
        }
        self.inner
            .insert(key.to_owned(), make_entry(data, effective_ttl))
            .await;
        Ok(true)
    }

    async fn get_or_set<V, F, Fut>(
        &self,
        key: &str,
        ttl: Option<Duration>,
        f: F,
    ) -> Result<V, Self::Error>
    where
        V: Serialize + DeserializeOwned + Send + Sync,
        F: FnOnce() -> Fut + Send,
        Fut: std::future::Future<Output = Result<V, Self::Error>> + Send,
    {
        if let Some(entry) = self.inner.get(key).await {
            return de(entry.data.as_ref());
        }
        let value = f().await?;
        self.set(key, &value, ttl).await?;
        Ok(value)
    }

    async fn get_many<V>(&self, keys: &[&str]) -> Result<Vec<Option<V>>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let mut out = Vec::with_capacity(keys.len());
        for key in keys {
            out.push(self.get(key).await?);
        }
        Ok(out)
    }

    async fn set_many<V>(
        &self,
        pairs: &[(&str, &V)],
        ttl: Option<Duration>,
    ) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        for (key, value) in pairs {
            self.set(key, value, ttl).await?;
        }
        Ok(())
    }

    async fn delete_many(&self, keys: &[&str]) -> Result<u64, Self::Error> {
        let mut count = 0u64;
        for key in keys {
            if self.delete(key).await? {
                count += 1;
            }
        }
        Ok(count)
    }
}
