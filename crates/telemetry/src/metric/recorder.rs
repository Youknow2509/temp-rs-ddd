use anyhow::{Context, Result};
use metrics_exporter_prometheus::PrometheusBuilder;
use tokio::task::JoinHandle;

use domain::config::MetricsSetting;
use tracing;

pub(super) fn install(cfg: &MetricsSetting) -> Result<Option<JoinHandle<()>>> {
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
    let (recorder, exporter) = PrometheusBuilder::new()
        .with_http_listener((cfg.host, cfg.port))
        .build()
        .context("failed to install Prometheus metrics recorder")?;

    metrics::set_global_recorder(recorder).context("failed to set global Prometheus recorder")?;

    Ok(Some(tokio::spawn(async move {
        let _ = exporter.await;
    })))
}
