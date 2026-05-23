use anyhow::{Context, Result};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};

use domain::config::MetricsSetting;
use tracing;

pub(super) fn install(cfg: &MetricsSetting) -> Result<Option<PrometheusHandle>> {
    if !cfg.enabled {
        return Ok(None);
    }

    if !cfg.namespace.is_empty() {
        tracing::warn!(
            namespace = %cfg.namespace,
            "metrics namespace not supported in metrics-exporter-prometheus 0.16; ignored"
        );
    }
    if cfg.path != crate::constants::METRICS_DEFAULT_PATH {
        tracing::warn!(
            configured = %cfg.path,
            effective  = crate::constants::METRICS_DEFAULT_PATH,
            "metrics HTTP path is not configurable; always served at /metrics"
        );
    }
    if cfg.collect_interval_secs > 0 {
        tracing::warn!(
            collect_interval_secs = cfg.collect_interval_secs,
            "collect_interval_secs has no effect; scrape interval is set on the Prometheus server"
        );
    }

    let handle = PrometheusBuilder::new()
        .with_http_listener((cfg.host, cfg.port))
        .install_recorder()
        .context("failed to install Prometheus metrics recorder")?;

    Ok(Some(handle))
}
