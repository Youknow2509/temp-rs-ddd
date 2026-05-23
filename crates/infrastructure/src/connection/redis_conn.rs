use std::fmt;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use deadpool_redis::{Config, Pool, Runtime};

use domain::config::RedisSettingRepository;

pub enum RedisPool {
    Single(Pool),
    Cluster(deadpool_redis::cluster::Pool),
}

impl fmt::Debug for RedisPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Single(_) => f.write_str("RedisPool::Single"),
            Self::Cluster(_) => f.write_str("RedisPool::Cluster"),
        }
    }
}

pub fn create_pool(setting: &RedisSettingRepository) -> Result<RedisPool> {
    let pool = match setting.r#type.as_str() {
        "standalone" | "sentinel" => RedisPool::Single(build_single_pool(setting)?),
        "cluster" => RedisPool::Cluster(build_cluster_pool(setting)?),
        other => bail!(
            "unknown redis type '{}'; expected standalone|sentinel|cluster",
            other
        ),
    };

    match &pool {
        RedisPool::Single(p) => ping_single(p),
        RedisPool::Cluster(p) => ping_cluster(p),
    }
    .context("Redis health-check failed at startup")?;

    Ok(pool)
}

fn build_single_pool(setting: &RedisSettingRepository) -> Result<Pool> {
    let url = if setting.r#type == "sentinel" {
        build_sentinel_url(setting)?
    } else {
        build_standalone_url(setting)?
    };

    let mut cfg = Config::from_url(url);
    #[allow(clippy::cast_possible_truncation)]
    {
        cfg.pool = Some(deadpool_redis::PoolConfig {
            max_size: setting.pool.pool_size as usize,
            timeouts: deadpool_redis::Timeouts {
                wait: Some(Duration::from_secs(setting.timeouts.pool_timeout)),
                create: Some(Duration::from_secs(setting.timeouts.dial_timeout)),
                recycle: Some(Duration::from_secs(setting.pool.conn_max_idle_time)),
            },
            ..Default::default()
        });
    }

    cfg.create_pool(Some(Runtime::Tokio1))
        .context("building redis single pool")
}

fn build_standalone_url(setting: &RedisSettingRepository) -> Result<String> {
    let standalone = setting
        .standalone
        .as_ref()
        .ok_or_else(|| anyhow!("standalone config missing for type=standalone"))?;

    let scheme = if setting.tls.is_enabled { "rediss" } else { "redis" };

    Ok(if setting.password.is_empty() {
        format!(
            "{}://{}:{}/{}",
            scheme, standalone.host, standalone.port, setting.db
        )
    } else {
        format!(
            "{}://{}:{}@{}:{}/{}",
            scheme,
            setting.username,
            setting.password,
            standalone.host,
            standalone.port,
            setting.db
        )
    })
}

fn build_sentinel_url(setting: &RedisSettingRepository) -> Result<String> {
    let sentinel = setting
        .sentinel
        .as_ref()
        .ok_or_else(|| anyhow!("sentinel config missing for type=sentinel"))?;

    let scheme = if setting.tls.is_enabled {
        "redis+sentinels"
    } else {
        "redis+sentinel"
    };

    let addrs = sentinel.sentinel_addrs.join(",");

    Ok(if sentinel.sentinel_password.is_empty() {
        format!(
            "{}://{}/{}/{}",
            scheme, addrs, sentinel.master_name, setting.db
        )
    } else {
        format!(
            "{}://{}:{}@{}/{}/{}",
            scheme,
            sentinel.sentinel_username,
            sentinel.sentinel_password,
            addrs,
            sentinel.master_name,
            setting.db
        )
    })
}

fn build_cluster_pool(setting: &RedisSettingRepository) -> Result<deadpool_redis::cluster::Pool> {
    let cluster = setting
        .cluster
        .as_ref()
        .ok_or_else(|| anyhow!("cluster config missing for type=cluster"))?;

    let scheme = if setting.tls.is_enabled { "rediss" } else { "redis" };

    let nodes: Vec<String> = cluster
        .cluster_addrs
        .iter()
        .map(|addr| {
            if setting.password.is_empty() {
                format!("{}://{}", scheme, addr)
            } else {
                format!(
                    "{}://{}:{}@{}",
                    scheme, setting.username, setting.password, addr
                )
            }
        })
        .collect();

    let mgr = deadpool_redis::cluster::Manager::new(nodes, cluster.read_only)
        .context("building redis cluster manager")?;

    #[allow(clippy::cast_possible_truncation)]
    let pool = deadpool_redis::cluster::Pool::builder(mgr)
        .runtime(Runtime::Tokio1)
        .max_size(setting.pool.pool_size as usize)
        .wait_timeout(Some(Duration::from_secs(setting.timeouts.pool_timeout)))
        .create_timeout(Some(Duration::from_secs(setting.timeouts.dial_timeout)))
        .recycle_timeout(Some(Duration::from_secs(setting.pool.conn_max_idle_time)))
        .build()
        .context("building redis cluster pool")?;

    Ok(pool)
}

fn ping_single(pool: &Pool) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building redis ping runtime")?;

    rt.block_on(async {
        let mut conn = pool.get().await.context("acquiring redis connection")?;
        redis::cmd("PING")
            .query_async::<String>(&mut *conn)
            .await
            .context("redis PING failed")?;
        Ok(())
    })
}

fn ping_cluster(pool: &deadpool_redis::cluster::Pool) -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building redis cluster ping runtime")?;

    rt.block_on(async {
        let mut conn = pool.get().await.context("acquiring redis cluster connection")?;
        redis::cmd("PING")
            .query_async::<String>(&mut *conn)
            .await
            .context("redis cluster PING failed")?;
        Ok(())
    })
}
