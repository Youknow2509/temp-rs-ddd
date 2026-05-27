pub(crate) mod constants;
pub mod log;
pub mod metric;
pub(crate) mod trace;

use anyhow::Result;
use metrics_exporter_prometheus::PrometheusHandle;
use opentelemetry_sdk::trace::SdkTracerProvider;
use tokio::runtime::Runtime;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, layer::SubscriberExt, util::SubscriberInitExt};

use domain::config::TelemetrySystemSetting;

pub struct TelemetryGuard {
    _log_guard: Option<WorkerGuard>,
    _metrics_handle: Option<PrometheusHandle>,
    _tracer_provider: Option<SdkTracerProvider>,
    _tokio_rt: Option<Runtime>,
}

impl std::fmt::Debug for TelemetryGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TelemetryGuard").finish_non_exhaustive()
    }
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        if let Some(provider) = self._tracer_provider.take()
            && let Err(e) = provider.shutdown()
        {
            eprintln!("OTEL tracer provider shutdown error: {e}");
        }
        self._tokio_rt.take();
        // _log_guard drops last — joins the file-appender thread.
    }
}

pub fn init(cfg: &TelemetrySystemSetting, extra_fields: &[(&str, &str)]) -> Result<TelemetryGuard> {
    let (fmt_layers, log_guard) =
        log::build_layers::<tracing_subscriber::Registry>(&cfg.logger, extra_fields)?;
    let trace_output = trace::build(&cfg.tracing)?;
    let metrics_handle = metric::build(&cfg.metrics)?;

    let mut all_layers: Vec<Box<dyn Layer<tracing_subscriber::Registry> + Send + Sync + 'static>> =
        fmt_layers;

    let (tracer_provider, tokio_rt) = if let Some(output) = trace_output {
        all_layers.push(output.layer.boxed());
        (Some(output.provider), Some(output.rt))
    } else {
        (None, None)
    };
    tracing_subscriber::registry().with(all_layers).try_init()?;
    Ok(TelemetryGuard {
        _log_guard: log_guard,
        _metrics_handle: metrics_handle,
        _tracer_provider: tracer_provider,
        _tokio_rt: tokio_rt,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::config::{
        LoggerFileSetting, LoggerSetting, MetricsSetting, TelemetrySystemSetting,
        TracingBatchSetting, TracingSetting,
    };
    use std::collections::HashMap;
    use std::net::IpAddr;
    use std::str::FromStr;

    fn test_cfg() -> TelemetrySystemSetting {
        TelemetrySystemSetting {
            logger: LoggerSetting {
                level: "info".into(),
                output: vec!["stdout".into()],
                caller: false,
                stacktrace_level: "error".into(),
                file: LoggerFileSetting {
                    enabled: false,
                    folder: "/tmp".into(),
                    filename: "test.log".into(),
                    max_size_mb: 10,
                    max_backups: 3,
                    max_age_days: 7,
                    compress: false,
                },
            },
            metrics: MetricsSetting {
                enabled: false,
                path: "/metrics".into(),
                port: 19092,
                host: IpAddr::from_str("127.0.0.1").unwrap(),
                namespace: "test".into(),
                collect_interval_secs: 15,
            },
            tracing: TracingSetting {
                enabled: false,
                service_name: "test".into(),
                service_version: "0.0.1".into(),
                otlp_endpoint: "http://127.0.0.1:4317".into(),
                protocol: "grpc".into(),
                sampling_ratio: 1.0,
                propagation: vec![],
                batch: TracingBatchSetting {
                    max_queue_size: 2048,
                    max_export_batch_size: 512,
                    schedule_delay_ms: 5000,
                    export_timeout_ms: 30000,
                },
                headers: HashMap::new(),
            },
        }
    }

    #[test]
    fn init_all_disabled_succeeds() {
        // All subsystems disabled — no ports bound, no OTEL runtime.
        let result = init(&test_cfg(), &[]);
        assert!(result.is_ok());
        // Guard drops cleanly.
    }
}
