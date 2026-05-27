mod exporter;
mod propagator;
mod provider;

use anyhow::Result;
use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_sdk::trace::SdkTracerProvider;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::Registry;

use domain::config::TracingSetting;

pub(crate) struct TraceOutput {
    pub(crate) provider: SdkTracerProvider,
    pub(crate) layer: OpenTelemetryLayer<Registry, opentelemetry_sdk::trace::Tracer>,
}

pub(crate) fn build(cfg: &TracingSetting) -> Result<Option<TraceOutput>> {
    if !cfg.enabled {
        return Ok(None);
    }

    let p = provider::build_provider(cfg)?;

    propagator::install_propagators(&cfg.propagation);

    let tracer = p.tracer(cfg.service_name.clone());
    let layer = tracing_opentelemetry::layer().with_tracer(tracer);

    global::set_tracer_provider(p.clone());

    Ok(Some(TraceOutput { provider: p, layer }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::config::{TracingBatchSetting, TracingSetting};
    use std::collections::HashMap;

    fn disabled_cfg() -> TracingSetting {
        TracingSetting {
            enabled: false,
            service_name: "test".into(),
            service_version: "0.0.1".into(),
            otlp_endpoint: "http://127.0.0.1:4317".into(),
            protocol: "grpc".into(),
            sampling_ratio: 1.0,
            propagation: vec!["tracecontext".into()],
            batch: TracingBatchSetting {
                max_queue_size: 2048,
                max_export_batch_size: 512,
                schedule_delay_ms: 5000,
                export_timeout_ms: 30000,
            },
            headers: HashMap::new(),
        }
    }

    #[test]
    fn disabled_returns_none() {
        let result = build(&disabled_cfg());
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn enabled_builds_output() {
        let mut cfg = disabled_cfg();
        cfg.enabled = true;
        // The exporter connects lazily — build() succeeds even without a live collector.
        let result = build(&cfg);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn http_protocol_builds() {
        let mut cfg = disabled_cfg();
        cfg.enabled = true;
        cfg.protocol = "http".into();
        let result = build(&cfg);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn http_protobuf_protocol_builds() {
        let mut cfg = disabled_cfg();
        cfg.enabled = true;
        cfg.protocol = "http/protobuf".into();
        let result = build(&cfg);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[test]
    fn invalid_protocol_returns_error() {
        let mut cfg = disabled_cfg();
        cfg.enabled = true;
        cfg.protocol = "tcp".into();
        let result = build(&cfg);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .to_string()
                .contains("unsupported OTLP protocol")
        );
    }
}
