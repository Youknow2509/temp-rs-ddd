use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use scylla::frame::Compression;
use scylla::frame::types::{Consistency, SerialConsistency};
use scylla::load_balancing::DefaultPolicy;
use scylla::retry_policy::{DefaultRetryPolicy, FallthroughRetryPolicy};
use scylla::speculative_execution::SimpleSpeculativeExecutionPolicy;
use scylla::transport::downgrading_consistency_retry_policy::DowngradingConsistencyRetryPolicy;
use scylla::transport::session::PoolSize;
use scylla::{ExecutionProfile, Session, SessionBuilder};

use domain::config::ScyllaDbSettingRepository;

pub type ScyllaSession = Arc<Session>;

pub fn create_session(setting: &ScyllaDbSettingRepository) -> Result<ScyllaSession> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building scylla runtime")?;

    rt.block_on(build_session(setting))
}

async fn build_session(setting: &ScyllaDbSettingRepository) -> Result<ScyllaSession> {
    let exec_profile = build_execution_profile(setting)?;

    let mut builder = SessionBuilder::new()
        .known_nodes(&setting.cluster.contact_points)
        .connection_timeout(Duration::from_millis(setting.timeouts.connect_timeout_ms))
        .default_execution_profile_handle(exec_profile.into_handle());

    if !setting.authentication.username.is_empty() {
        builder = builder.user(
            &setting.authentication.username,
            &setting.authentication.password,
        );
    }

    if !setting.cluster.keyspace.is_empty() {
        builder = builder.use_keyspace(&setting.cluster.keyspace, false);
    }

    if let Some(compression) = parse_compression(&setting.cluster.compression)? {
        builder = builder.compression(Some(compression));
    }

    let pool_size = NonZeroUsize::new(setting.pool.connections_per_host_local as usize)
        .unwrap_or(NonZeroUsize::MIN);
    builder = builder.pool_size(PoolSize::PerHost(pool_size));

    if setting.pool.keepalive_interval_ms > 0 {
        builder =
            builder.keepalive_interval(Duration::from_millis(setting.pool.keepalive_interval_ms));
    }

    if setting.ssl.enabled {
        let ssl_ctx = build_ssl_context(setting)?;
        builder = builder.ssl_context(Some(ssl_ctx));
    }

    let session = builder.build().await.context("building ScyllaDB session")?;

    session
        .query_unpaged("SELECT now() FROM system.local", &[])
        .await
        .context("ScyllaDB health-check failed at startup")?;

    Ok(Arc::new(session))
}

fn build_execution_profile(setting: &ScyllaDbSettingRepository) -> Result<ExecutionProfile> {
    let consistency = parse_consistency(&setting.consistency.default)?;
    let serial_consistency = parse_serial_consistency(&setting.consistency.serial)?;

    let load_balancing = DefaultPolicy::builder()
        .prefer_datacenter(setting.cluster.local_dc.clone())
        .token_aware(true)
        .build();

    let retry_policy: Box<dyn scylla::retry_policy::RetryPolicy> =
        match setting.retry.policy.as_str() {
            "default" => Box::new(DefaultRetryPolicy::new()),
            "downgrading" => Box::new(DowngradingConsistencyRetryPolicy::new()),
            "fallthrough" => Box::new(FallthroughRetryPolicy::new()),
            other => bail!(
                "unknown retry policy '{}'; expected default|downgrading|fallthrough",
                other
            ),
        };

    let mut profile_builder = ExecutionProfile::builder()
        .consistency(consistency)
        .serial_consistency(Some(serial_consistency))
        .request_timeout(Some(Duration::from_millis(
            setting.timeouts.request_timeout_ms,
        )))
        .load_balancing_policy(load_balancing)
        .retry_policy(retry_policy);

    if setting.speculative.enabled {
        let spec_policy = Arc::new(SimpleSpeculativeExecutionPolicy {
            max_retry_count: setting.speculative.max_speculative_executions as usize,
            retry_interval: Duration::from_millis(setting.speculative.delay_ms),
        });
        profile_builder = profile_builder.speculative_execution_policy(Some(spec_policy));
    }

    Ok(profile_builder.build())
}

fn build_ssl_context(setting: &ScyllaDbSettingRepository) -> Result<openssl::ssl::SslContext> {
    use openssl::ssl::{SslContextBuilder, SslFiletype, SslMethod, SslVerifyMode};

    let mut ctx_builder =
        SslContextBuilder::new(SslMethod::tls()).context("creating SSL context builder")?;

    if !setting.ssl.ca_cert_path.is_empty() {
        ctx_builder
            .set_ca_file(&setting.ssl.ca_cert_path)
            .context("loading CA certificate")?;
    }

    if !setting.ssl.user_cert_path.is_empty() {
        ctx_builder
            .set_certificate_file(&setting.ssl.user_cert_path, SslFiletype::PEM)
            .context("loading client certificate")?;
    }

    if !setting.ssl.user_key_path.is_empty() {
        ctx_builder
            .set_private_key_file(&setting.ssl.user_key_path, SslFiletype::PEM)
            .context("loading client private key")?;
    }

    if setting.ssl.validate_hostname {
        ctx_builder.set_verify(SslVerifyMode::PEER);
    } else {
        ctx_builder.set_verify(SslVerifyMode::NONE);
    }

    Ok(ctx_builder.build())
}

fn parse_compression(s: &str) -> Result<Option<Compression>> {
    match s {
        "none" | "" => Ok(None),
        "snappy" => Ok(Some(Compression::Snappy)),
        "lz4" => Ok(Some(Compression::Lz4)),
        other => bail!("unknown compression '{}'; expected none|snappy|lz4", other),
    }
}

fn parse_consistency(s: &str) -> Result<Consistency> {
    match s.to_ascii_uppercase().as_str() {
        "ANY" => Ok(Consistency::Any),
        "ONE" => Ok(Consistency::One),
        "TWO" => Ok(Consistency::Two),
        "THREE" => Ok(Consistency::Three),
        "QUORUM" => Ok(Consistency::Quorum),
        "ALL" => Ok(Consistency::All),
        "LOCAL_QUORUM" => Ok(Consistency::LocalQuorum),
        "EACH_QUORUM" => Ok(Consistency::EachQuorum),
        "LOCAL_ONE" => Ok(Consistency::LocalOne),
        other => bail!(
            "unknown consistency '{}'; expected ANY|ONE|TWO|THREE|QUORUM|ALL|LOCAL_QUORUM|EACH_QUORUM|LOCAL_ONE",
            other
        ),
    }
}

fn parse_serial_consistency(s: &str) -> Result<SerialConsistency> {
    match s.to_ascii_uppercase().as_str() {
        "SERIAL" => Ok(SerialConsistency::Serial),
        "LOCAL_SERIAL" => Ok(SerialConsistency::LocalSerial),
        other => bail!(
            "unknown serial consistency '{}'; expected SERIAL|LOCAL_SERIAL",
            other
        ),
    }
}
