use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result, bail};
use tonic::transport::{Certificate, Channel, ClientTlsConfig, Endpoint, Identity};
use tonic_health::ServingStatus;
use tonic_health::pb::HealthCheckRequest;
use tonic_health::pb::health_client::HealthClient;
use tracing::warn;

use domain::config::{GrpcClientSetting, GrpcClientsSetting};

/// A map of logical service name → gRPC `Channel`.
///
/// `Channel` is `Clone + Send + Sync` with an `Arc` inside — cloning is cheap.
#[derive(Debug, Clone)]
pub struct GrpcClients {
    channels: HashMap<String, Channel>,
}

impl GrpcClients {
    pub fn get(&self, service: &str) -> Option<&Channel> {
        self.channels.get(service)
    }
}

/// Build one `Channel` per service in `setting.services`, run a health check
/// against each, and return the aggregated `GrpcClients`.
pub async fn create_clients(setting: &GrpcClientsSetting) -> Result<GrpcClients> {
    if !setting.is_enabled {
        return Ok(GrpcClients {
            channels: HashMap::new(),
        });
    }

    let mut channels = HashMap::with_capacity(setting.services.len());

    for (name, cfg) in &setting.services {
        let channel = build_channel(cfg)
            .await
            .with_context(|| format!("building gRPC channel for service '{name}'"))?;

        if cfg.health_check_enabled {
            health_check(&channel, name, cfg)
                .await
                .with_context(|| format!("gRPC health check failed for service '{name}'"))?;
        }

        channels.insert(name.clone(), channel);
    }

    Ok(GrpcClients { channels })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Resolve endpoint address strings from config and assemble a `Channel`.
async fn build_channel(cfg: &GrpcClientSetting) -> Result<Channel> {
    let tls = if cfg.tls.is_enabled {
        Some(build_tls(cfg).context("building TLS config")?)
    } else {
        None
    };

    let scheme = if cfg.tls.is_enabled { "https" } else { "http" };

    match cfg.endpoint.discovery.as_str() {
        "static" => {
            if cfg.endpoint.addresses.is_empty() {
                bail!("endpoint.addresses must not be empty for static discovery");
            }

            let mut endpoints: Vec<Endpoint> = Vec::with_capacity(cfg.endpoint.addresses.len());
            for addr in &cfg.endpoint.addresses {
                let uri = format!("{scheme}://{addr}");
                endpoints.push(make_endpoint(uri, cfg, tls.as_ref())?);
            }

            if endpoints.len() == 1 {
                // Single endpoint: respect lazy_connect.
                let ep = endpoints.remove(0);
                if cfg.connection.lazy_connect {
                    Ok(ep.connect_lazy())
                } else {
                    Ok(ep.connect().await.context("connecting to gRPC endpoint")?)
                }
            } else {
                // Multiple endpoints: balance across all of them.
                // balance_list is always lazy; the health check forces the first real connect.
                Ok(Channel::balance_list(endpoints.into_iter()))
            }
        }

        "dns" => {
            let service_name = cfg
                .endpoint
                .service_name
                .as_deref()
                .context("endpoint.service_name is required for dns discovery")?;

            let uri = format!("{scheme}://{service_name}");
            let ep = make_endpoint(uri, cfg, tls.as_ref())?;

            if cfg.connection.lazy_connect {
                Ok(ep.connect_lazy())
            } else {
                Ok(ep.connect().await.context("connecting to gRPC endpoint")?)
            }
        }

        other => bail!("unsupported endpoint discovery mode '{other}'; expected 'static' or 'dns'"),
    }
}

/// Configure a single `Endpoint` with all connection/keepalive options.
fn make_endpoint(
    addr: String,
    cfg: &GrpcClientSetting,
    tls: Option<&ClientTlsConfig>,
) -> Result<Endpoint> {
    let mut ep = Channel::from_shared(addr).context("parsing gRPC endpoint URI")?;

    ep = ep
        .connect_timeout(Duration::from_millis(cfg.connection.connect_timeout_ms))
        .tcp_nodelay(cfg.connection.tcp_nodelay)
        .tcp_keepalive(Some(Duration::from_millis(cfg.keepalive.tcp_keepalive_ms)))
        .http2_keep_alive_interval(Duration::from_millis(
            cfg.keepalive.http2_keepalive_interval_ms,
        ))
        .keep_alive_timeout(Duration::from_millis(
            cfg.keepalive.http2_keepalive_timeout_ms,
        ))
        .keep_alive_while_idle(cfg.keepalive.keepalive_while_idle)
        .http2_adaptive_window(true);

    if let Some(tls_cfg) = tls {
        ep = ep
            .tls_config(tls_cfg.clone())
            .context("applying TLS config to endpoint")?;
    }

    Ok(ep)
}

/// Build a `ClientTlsConfig` from the service's TLS settings.
fn build_tls(cfg: &GrpcClientSetting) -> Result<ClientTlsConfig> {
    let mut tls = ClientTlsConfig::new();

    if let Some(authority) = &cfg.endpoint.authority {
        tls = tls.domain_name(authority.clone());
    }

    if cfg.tls.require_client_cert {
        let cert_pem = std::fs::read(&cfg.tls.cert_file)
            .with_context(|| format!("reading TLS cert: {}", cfg.tls.cert_file.display()))?;
        let key_pem = std::fs::read(&cfg.tls.key_file)
            .with_context(|| format!("reading TLS key: {}", cfg.tls.key_file.display()))?;
        let identity = Identity::from_pem(cert_pem, key_pem);
        tls = tls.identity(identity);
    }

    if let Some(ca_path) = &cfg.tls.client_ca_file {
        let ca_pem = std::fs::read(ca_path)
            .with_context(|| format!("reading CA cert: {}", ca_path.display()))?;
        let ca = Certificate::from_pem(ca_pem);
        tls = tls.ca_certificate(ca);
    }

    Ok(tls)
}

/// Run `grpc.health.v1.Health/Check` against the channel.
///
/// - `UNIMPLEMENTED`: the server hasn't opted in to the health protocol — warn and skip.
/// - Timeout: hard error.
/// - Any other RPC error: hard error.
/// - `NOT_SERVING` / unexpected status: hard error.
async fn health_check(channel: &Channel, name: &str, cfg: &GrpcClientSetting) -> Result<()> {
    let mut client = HealthClient::new(channel.clone());
    let service = cfg.endpoint.service.as_deref().unwrap_or("").to_owned();
    let timeout = Duration::from_millis(cfg.connection.connect_timeout_ms);

    let result = tokio::time::timeout(timeout, client.check(HealthCheckRequest { service })).await;

    match result {
        Err(_elapsed) => {
            bail!("gRPC health check timed out for '{name}'");
        }
        Ok(Err(status)) => match status.code() {
            // Server is reachable but doesn't implement grpc.health.v1 — skip.
            tonic::Code::Unimplemented | tonic::Code::Cancelled | tonic::Code::NotFound => {
                warn!(
                    service = name,
                    code = ?status.code(),
                    "gRPC health check skipped: server does not support health protocol"
                );
                Ok(())
            }
            _ => bail!("gRPC health check RPC failed for '{name}': {status}"),
        },
        Ok(Ok(resp)) => {
            let status = resp.into_inner().status;
            if status == ServingStatus::Serving as i32 {
                Ok(())
            } else {
                bail!("gRPC health check for '{name}' returned non-SERVING status code {status}");
            }
        }
    }
}
