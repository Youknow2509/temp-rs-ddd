//! Concrete repository implementations bound to the connection pools.

use std::sync::Arc;

use anyhow::Result;

use domain::cache::{DistributedCache, LocalCache};
use domain::config::SystemConfig;
use infrastructure::cache::{MokaCache, RedisCache};
use infrastructure::connection::RedisPool;

#[derive(Debug)]
pub struct Repositories<A: LocalCache, B: DistributedCache> {
    pub local_cache: Arc<A>,
    pub distributed_cache: Arc<B>,
    // TODO: pg_user_repo, scylla_event_repo, s3_blob_repo, ...
}

pub fn build(
    config: &SystemConfig,
    redis_pool: RedisPool,
) -> Result<Repositories<MokaCache, RedisCache>> {
    let local_cache = Arc::new(MokaCache::new(&config.repository.local_cache));
    let distributed_cache = Arc::new(RedisCache::new(redis_pool));

    Ok(Repositories {
        local_cache,
        distributed_cache,
    })
}
