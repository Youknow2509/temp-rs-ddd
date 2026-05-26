use std::time::Duration;

use anyhow::{Context, Result, bail};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::{Config as PgConfig, config::SslMode};

use domain::config::PostgresqlSettingRepository;

pub type PgPool = Pool;

pub fn create_pool(setting: &PostgresqlSettingRepository) -> Result<PgPool> {
    let pg_config = build_pg_config(setting)?;
    let tls = build_tls(setting)?;

    let mgr = Manager::from_config(
        pg_config,
        tls,
        ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        },
    );

    #[allow(clippy::cast_possible_truncation)]
    let pool = PgPool::builder(mgr)
        .runtime(Runtime::Tokio1)
        .max_size(setting.pool.max_conns as usize)
        .wait_timeout(Some(Duration::from_secs(
            setting.timeouts.connection_timeout,
        )))
        .create_timeout(Some(Duration::from_secs(
            setting.timeouts.connection_timeout,
        )))
        .recycle_timeout(Some(Duration::from_secs(setting.pool.max_conn_idle_time)))
        .build()
        .context("building postgres connection pool")?;

    ping(&pool).context("PostgreSQL health-check failed at startup")?;

    Ok(pool)
}

fn ping(pool: &PgPool) -> Result<()> {
    use crate::repository::pg_healthy_repo::PgHealthyRepo;
    use domain::repository::healthy_repo::HealthyRepository as _;

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building ping runtime")?;

    rt.block_on(PgHealthyRepo::new(pool).is_healthy())
        .context("postgres ping failed")
}

fn build_pg_config(setting: &PostgresqlSettingRepository) -> Result<PgConfig> {
    let addrs = &setting.connection.address;

    if addrs.is_empty() {
        bail!("postgresql.connection.address must not be empty");
    }

    if addrs.len() > 1 {
        tracing::warn!(
            extra = addrs.len() - 1,
            "tokio-postgres supports one host per Config; only the first address is used"
        );
    }

    let (host, port_str) = addrs[0]
        .rsplit_once(':')
        .with_context(|| format!("address '{}' must be host:port", addrs[0]))?;
    let port: u16 = port_str
        .parse()
        .with_context(|| format!("port '{}' is not a valid u16", port_str))?;

    let t = &setting.timeouts;
    let options = format!(
        "-c statement_timeout={} -c idle_in_transaction_session_timeout={} -c TimeZone={}",
        t.statement_timeout * 1000,
        t.idle_in_transaction_timeout * 1000,
        setting.tz,
    );

    let ssl_mode = if setting.tls.is_enabled {
        SslMode::Require
    } else {
        SslMode::Disable
    };

    let mut cfg = PgConfig::new();
    cfg.host(host)
        .port(port)
        .dbname(&setting.connection.database)
        .user(&setting.connection.username)
        .password(setting.connection.password.as_str())
        .application_name(&setting.appname)
        .options(&options)
        .connect_timeout(Duration::from_secs(t.connection_timeout))
        .ssl_mode(ssl_mode);

    Ok(cfg)
}

fn build_tls(setting: &PostgresqlSettingRepository) -> Result<MakeTlsConnector> {
    let tls = &setting.tls;
    let mut builder = TlsConnector::builder();

    if tls.is_enabled {
        let cert_pem = std::fs::read(&tls.cert_file)
            .with_context(|| format!("reading TLS cert: {}", tls.cert_file.display()))?;
        let key_pem = std::fs::read(&tls.key_file)
            .with_context(|| format!("reading TLS key: {}", tls.key_file.display()))?;
        let identity = native_tls::Identity::from_pkcs8(&cert_pem, &key_pem)
            .context("building TLS identity from cert+key")?;
        builder.identity(identity);

        if let Some(ref ca_path) = tls.client_ca_file {
            let ca_pem = std::fs::read(ca_path)
                .with_context(|| format!("reading CA cert: {}", ca_path.display()))?;
            let ca =
                native_tls::Certificate::from_pem(&ca_pem).context("parsing CA certificate")?;
            builder.add_root_certificate(ca);
        }
    }

    let connector = builder.build().context("building native TLS connector")?;
    Ok(MakeTlsConnector::new(connector))
}
