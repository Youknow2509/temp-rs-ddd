use crate::connection::RedisPool;
use bytes::Bytes;
use domain::cache::DistributedCache;
use redis::AsyncCommands;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt;
use std::time::Duration;

// --- Error ---

#[derive(Debug)]
pub struct RedisCacheError(String);

impl RedisCacheError {
    fn pool<E: fmt::Display>(e: E) -> Self {
        Self(format!("pool error: {e}"))
    }
    fn redis(e: redis::RedisError) -> Self {
        Self(format!("redis error: {e}"))
    }
    fn serde(e: serde_json::Error) -> Self {
        Self(format!("serialization error: {e}"))
    }
}

impl fmt::Display for RedisCacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RedisCacheError {}

// --- Serialization helpers ---

fn ser<V: Serialize>(v: &V) -> Result<Bytes, RedisCacheError> {
    serde_json::to_vec(v)
        .map(Bytes::from)
        .map_err(RedisCacheError::serde)
}

fn de<V: DeserializeOwned>(bytes: &[u8]) -> Result<V, RedisCacheError> {
    serde_json::from_slice(bytes).map_err(RedisCacheError::serde)
}

// --- RedisCache ---

#[derive(Debug, Clone)]
pub struct RedisCache {
    pool: RedisPool,
}

impl RedisCache {
    pub fn new(pool: RedisPool) -> Self {
        Self { pool }
    }

    pub fn close(&self) {
        match &self.pool {
            RedisPool::Single(p) => p.close(),
            RedisPool::Cluster(p) => p.close(),
        }
    }
}

// Helper macro: get a connection from Single or Cluster pool and run a block.
// Both connection types deref to a type implementing ConnectionLike.
macro_rules! with_conn {
    ($pool:expr, $conn:ident, $body:expr) => {
        match &$pool {
            RedisPool::Single(p) => {
                let mut c = p.get().await.map_err(RedisCacheError::pool)?;
                let $conn = &mut *c;
                $body
            }
            RedisPool::Cluster(p) => {
                let mut c = p.get().await.map_err(RedisCacheError::pool)?;
                let $conn = &mut *c;
                $body
            }
        }
    };
}

// --- impl DistributedCache ---

impl DistributedCache for RedisCache {
    type Error = RedisCacheError;

    // --- String: basic ---

    async fn get<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let bytes: Option<Bytes> = with_conn!(self.pool, conn, {
            conn.get(key).await.map_err(RedisCacheError::redis)?
        });
        match bytes {
            None => Ok(None),
            Some(b) => de(b.as_ref()).map(Some),
        }
    }

    async fn set<V>(&self, key: &str, value: &V, ttl: Option<Duration>) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(value)?;
        with_conn!(self.pool, conn, {
            match ttl {
                Some(d) => {
                    let secs = d.as_secs().max(1);
                    redis::cmd("SET")
                        .arg(key)
                        .arg(data.as_ref())
                        .arg("EX")
                        .arg(secs)
                        .query_async::<()>(conn)
                        .await
                        .map_err(RedisCacheError::redis)?;
                }
                None => {
                    conn.set::<_, _, ()>(key, data.as_ref())
                        .await
                        .map_err(RedisCacheError::redis)?;
                }
            }
        });
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<bool, Self::Error> {
        let count: u64 = with_conn!(self.pool, conn, {
            conn.del(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(count > 0)
    }

    // --- Key metadata ---

    async fn exists(&self, key: &str) -> Result<bool, Self::Error> {
        let count: u64 = with_conn!(self.pool, conn, {
            conn.exists(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(count > 0)
    }

    async fn expire(&self, key: &str, ttl: Duration) -> Result<bool, Self::Error> {
        let secs = ttl.as_secs().max(1);
        let ok: bool = with_conn!(self.pool, conn, {
            conn.expire(key, secs as i64)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(ok)
    }

    async fn persist(&self, key: &str) -> Result<bool, Self::Error> {
        let ok: bool = with_conn!(self.pool, conn, {
            conn.persist(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(ok)
    }

    async fn ttl(&self, key: &str) -> Result<Option<Duration>, Self::Error> {
        let secs: i64 = with_conn!(self.pool, conn, {
            conn.ttl(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(match secs {
            s if s < 0 => None,
            s => Some(Duration::from_secs(s as u64)),
        })
    }

    // --- Atomic counters ---

    async fn increment(&self, key: &str, delta: i64) -> Result<i64, Self::Error> {
        let v: i64 = with_conn!(self.pool, conn, {
            conn.incr(key, delta)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(v)
    }

    async fn decrement(&self, key: &str, delta: i64) -> Result<i64, Self::Error> {
        let v: i64 = with_conn!(self.pool, conn, {
            conn.decr(key, delta)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(v)
    }

    // --- Conditional set ---

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
        let ok: bool = with_conn!(self.pool, conn, {
            match ttl {
                Some(d) => {
                    let secs = d.as_secs().max(1);
                    redis::cmd("SET")
                        .arg(key)
                        .arg(data.as_ref())
                        .arg("NX")
                        .arg("EX")
                        .arg(secs)
                        .query_async::<Option<String>>(conn)
                        .await
                        .map_err(RedisCacheError::redis)?
                        .is_some()
                }
                None => {
                    let set: bool = conn
                        .set_nx(key, data.as_ref())
                        .await
                        .map_err(RedisCacheError::redis)?;
                    set
                }
            }
        });
        Ok(ok)
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
        if let Some(v) = self.get(key).await? {
            return Ok(v);
        }
        let value = f().await?;
        self.set(key, &value, ttl).await?;
        Ok(value)
    }

    // --- Batch ---

    async fn get_many<V>(&self, keys: &[&str]) -> Result<Vec<Option<V>>, Self::Error>
    where
        V: DeserializeOwned,
    {
        if keys.is_empty() {
            return Ok(vec![]);
        }
        let raw: Vec<Option<Bytes>> = with_conn!(self.pool, conn, {
            redis::cmd("MGET")
                .arg(keys)
                .query_async(conn)
                .await
                .map_err(RedisCacheError::redis)?
        });
        raw.into_iter()
            .map(|maybe| match maybe {
                None => Ok(None),
                Some(b) => de(b.as_ref()).map(Some),
            })
            .collect()
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
        if keys.is_empty() {
            return Ok(0);
        }
        let count: u64 = with_conn!(self.pool, conn, {
            conn.del(keys).await.map_err(RedisCacheError::redis)?
        });
        Ok(count)
    }

    // --- List ---

    async fn list_push_front<V>(&self, key: &str, value: &V) -> Result<u64, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(value)?;
        let len: u64 = with_conn!(self.pool, conn, {
            conn.lpush(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(len)
    }

    async fn list_push_back<V>(&self, key: &str, value: &V) -> Result<u64, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(value)?;
        let len: u64 = with_conn!(self.pool, conn, {
            conn.rpush(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(len)
    }

    async fn list_pop_front<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let bytes: Option<Bytes> = with_conn!(self.pool, conn, {
            conn.lpop(key, None).await.map_err(RedisCacheError::redis)?
        });
        match bytes {
            None => Ok(None),
            Some(b) => de(b.as_ref()).map(Some),
        }
    }

    async fn list_pop_back<V>(&self, key: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let bytes: Option<Bytes> = with_conn!(self.pool, conn, {
            conn.rpop(key, None).await.map_err(RedisCacheError::redis)?
        });
        match bytes {
            None => Ok(None),
            Some(b) => de(b.as_ref()).map(Some),
        }
    }

    async fn list_range<V>(&self, key: &str, start: i64, stop: i64) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        #[allow(clippy::cast_possible_truncation)]
        let raw: Vec<Bytes> = with_conn!(self.pool, conn, {
            conn.lrange(key, start as isize, stop as isize)
                .await
                .map_err(RedisCacheError::redis)?
        });
        raw.iter().map(|b| de(b.as_ref())).collect()
    }

    async fn list_len(&self, key: &str) -> Result<u64, Self::Error> {
        let len: u64 = with_conn!(self.pool, conn, {
            conn.llen(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(len)
    }

    // --- Set ---

    async fn set_add<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let added: u64 = with_conn!(self.pool, conn, {
            conn.sadd(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(added > 0)
    }

    async fn set_remove<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let removed: u64 = with_conn!(self.pool, conn, {
            conn.srem(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(removed > 0)
    }

    async fn set_is_member<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let is_member: bool = with_conn!(self.pool, conn, {
            conn.sismember(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(is_member)
    }

    async fn set_members<V>(&self, key: &str) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let raw: Vec<Bytes> = with_conn!(self.pool, conn, {
            conn.smembers(key).await.map_err(RedisCacheError::redis)?
        });
        raw.iter().map(|b| de(b.as_ref())).collect()
    }

    async fn set_card(&self, key: &str) -> Result<u64, Self::Error> {
        let card: u64 = with_conn!(self.pool, conn, {
            conn.scard(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(card)
    }

    // --- Hash ---

    async fn hash_set<V>(&self, key: &str, field: &str, value: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(value)?;
        let added: u64 = with_conn!(self.pool, conn, {
            conn.hset(key, field, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(added > 0)
    }

    async fn hash_set_many<V>(&self, key: &str, fields: &[(&str, &V)]) -> Result<(), Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let mut pipe = redis::pipe();
        for (field, value) in fields {
            let data = ser(value)?;
            pipe.cmd("HSET")
                .arg(key)
                .arg(field)
                .arg(data.as_ref())
                .ignore();
        }
        with_conn!(self.pool, conn, {
            let (): () = pipe
                .query_async(conn)
                .await
                .map_err(RedisCacheError::redis)?;
        });
        Ok(())
    }

    async fn hash_get<V>(&self, key: &str, field: &str) -> Result<Option<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let bytes: Option<Bytes> = with_conn!(self.pool, conn, {
            conn.hget(key, field)
                .await
                .map_err(RedisCacheError::redis)?
        });
        match bytes {
            None => Ok(None),
            Some(b) => de(b.as_ref()).map(Some),
        }
    }

    async fn hash_get_many<V>(
        &self,
        key: &str,
        fields: &[&str],
    ) -> Result<Vec<Option<V>>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let raw: Vec<Option<Bytes>> = with_conn!(self.pool, conn, {
            redis::cmd("HMGET")
                .arg(key)
                .arg(fields)
                .query_async(conn)
                .await
                .map_err(RedisCacheError::redis)?
        });
        raw.into_iter()
            .map(|maybe| match maybe {
                None => Ok(None),
                Some(b) => de(b.as_ref()).map(Some),
            })
            .collect()
    }

    async fn hash_get_all<V>(&self, key: &str) -> Result<Vec<(String, V)>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let raw: Vec<(String, Bytes)> = with_conn!(self.pool, conn, {
            conn.hgetall(key).await.map_err(RedisCacheError::redis)?
        });
        raw.into_iter()
            .map(|(f, b)| de(b.as_ref()).map(|v| (f, v)))
            .collect()
    }

    async fn hash_delete(&self, key: &str, field: &str) -> Result<bool, Self::Error> {
        let removed: u64 = with_conn!(self.pool, conn, {
            conn.hdel(key, field)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(removed > 0)
    }

    async fn hash_exists(&self, key: &str, field: &str) -> Result<bool, Self::Error> {
        let exists: bool = with_conn!(self.pool, conn, {
            conn.hexists(key, field)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(exists)
    }

    async fn hash_len(&self, key: &str) -> Result<u64, Self::Error> {
        let len: u64 = with_conn!(self.pool, conn, {
            conn.hlen(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(len)
    }

    // --- Sorted set ---

    async fn zset_add<V>(&self, key: &str, member: &V, score: f64) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let added: u64 = with_conn!(self.pool, conn, {
            conn.zadd(key, data.as_ref(), score)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(added > 0)
    }

    async fn zset_remove<V>(&self, key: &str, member: &V) -> Result<bool, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let removed: u64 = with_conn!(self.pool, conn, {
            conn.zrem(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(removed > 0)
    }

    async fn zset_score<V>(&self, key: &str, member: &V) -> Result<Option<f64>, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let score: Option<f64> = with_conn!(self.pool, conn, {
            conn.zscore(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(score)
    }

    async fn zset_rank<V>(&self, key: &str, member: &V) -> Result<Option<u64>, Self::Error>
    where
        V: Serialize + Send + Sync,
    {
        let data = ser(member)?;
        let rank: Option<u64> = with_conn!(self.pool, conn, {
            conn.zrank(key, data.as_ref())
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(rank)
    }

    async fn zset_range<V>(&self, key: &str, start: i64, stop: i64) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        #[allow(clippy::cast_possible_truncation)]
        let raw: Vec<Bytes> = with_conn!(self.pool, conn, {
            conn.zrange(key, start as isize, stop as isize)
                .await
                .map_err(RedisCacheError::redis)?
        });
        raw.iter().map(|b| de(b.as_ref())).collect()
    }

    async fn zset_range_by_score<V>(
        &self,
        key: &str,
        min: f64,
        max: f64,
    ) -> Result<Vec<V>, Self::Error>
    where
        V: DeserializeOwned,
    {
        let raw: Vec<Bytes> = with_conn!(self.pool, conn, {
            conn.zrangebyscore(key, min, max)
                .await
                .map_err(RedisCacheError::redis)?
        });
        raw.iter().map(|b| de(b.as_ref())).collect()
    }

    async fn zset_card(&self, key: &str) -> Result<u64, Self::Error> {
        let card: u64 = with_conn!(self.pool, conn, {
            conn.zcard(key).await.map_err(RedisCacheError::redis)?
        });
        Ok(card)
    }

    // --- Lua scripting ---

    async fn eval<V>(&self, script: &str, keys: &[&str], args: &[&str]) -> Result<V, Self::Error>
    where
        V: DeserializeOwned,
    {
        let bytes: Bytes = with_conn!(self.pool, conn, {
            redis::Script::new(script)
                .key(keys)
                .arg(args)
                .invoke_async(conn)
                .await
                .map_err(RedisCacheError::redis)?
        });
        de(bytes.as_ref())
    }

    async fn eval_sha<V>(&self, sha: &str, keys: &[&str], args: &[&str]) -> Result<V, Self::Error>
    where
        V: DeserializeOwned,
    {
        let bytes: Bytes = with_conn!(self.pool, conn, {
            redis::cmd("EVALSHA")
                .arg(sha)
                .arg(keys.len())
                .arg(keys)
                .arg(args)
                .query_async(conn)
                .await
                .map_err(RedisCacheError::redis)?
        });
        de(bytes.as_ref())
    }

    async fn script_load(&self, script: &str) -> Result<String, Self::Error> {
        let sha: String = with_conn!(self.pool, conn, {
            redis::cmd("SCRIPT")
                .arg("LOAD")
                .arg(script)
                .query_async(conn)
                .await
                .map_err(RedisCacheError::redis)?
        });
        Ok(sha)
    }

    // --- Health ---

    async fn ping(&self) -> Result<(), Self::Error> {
        with_conn!(self.pool, conn, {
            redis::cmd("PING")
                .query_async::<String>(conn)
                .await
                .map_err(RedisCacheError::redis)?;
        });
        Ok(())
    }
}
