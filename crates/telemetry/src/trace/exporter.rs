use anyhow::{Context, Result};
use opentelemetry_otlp::{WithExportConfig, WithHttpConfig, WithTonicConfig};

use domain::config::TracingSetting;

pub(super) fn build_exporter(cfg: &TracingSetting) -> Result<opentelemetry_otlp::SpanExporter> {
    match cfg.protocol.as_str() {
        crate::constants::OTLP_PROTOCOL_GRPC => {
            let mut meta = tonic::metadata::MetadataMap::new();
            for (k, v) in &cfg.headers {
                match (
                    k.parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>(),
                    v.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>(),
                ) {
                    (Ok(key), Ok(val)) => {
                        meta.insert(key, val);
                    }
                    _ => tracing::warn!(header_key = %k, "skipping invalid gRPC metadata header"),
                }
            }
            opentelemetry_otlp::SpanExporter::builder()
                .with_tonic()
                .with_endpoint(&cfg.otlp_endpoint)
                .with_metadata(meta)
                .build()
                .context("failed to build gRPC OTLP exporter")
        }
        crate::constants::OTLP_PROTOCOL_HTTP | crate::constants::OTLP_PROTOCOL_HTTP_PROTOBUF => {
            opentelemetry_otlp::SpanExporter::builder()
                .with_http()
                .with_endpoint(&cfg.otlp_endpoint)
                .with_headers(cfg.headers.clone())
                .build()
                .context("failed to build HTTP OTLP exporter")
        }
        other => {
            anyhow::bail!("unsupported OTLP protocol: {other:?}; expected \"grpc\" or \"http\"")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::config::{TracingBatchSetting, TracingSetting};
    use std::collections::HashMap;

    fn cfg(protocol: &str) -> TracingSetting {
        TracingSetting {
            enabled: true,
            service_name: "test".into(),
            service_version: "0.0.1".into(),
            otlp_endpoint: "http://127.0.0.1:4317".into(),
            protocol: protocol.into(),
            sampling_ratio: 1.0,
            propagation: vec![],
            batch: TracingBatchSetting {
                max_queue_size: 2048,
                max_export_batch_size: 512,
                schedule_delay_ms: 5000,
                export_timeout_ms: 30000,
            },
            headers: HashMap::new(),
        }
    }

    fn with_rt<F, T>(f: F) -> T
    where
        F: FnOnce() -> T,
    {
        // tonic requires a Tokio reactor even for lazy channel creation.
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async { f() })
    }

    #[test]
    fn grpc_exporter_builds() {
        with_rt(|| assert!(build_exporter(&cfg(crate::constants::OTLP_PROTOCOL_GRPC)).is_ok()));
    }

    #[test]
    fn http_exporter_builds() {
        assert!(build_exporter(&cfg(crate::constants::OTLP_PROTOCOL_HTTP)).is_ok());
    }

    #[test]
    fn http_protobuf_exporter_builds() {
        assert!(build_exporter(&cfg(crate::constants::OTLP_PROTOCOL_HTTP_PROTOBUF)).is_ok());
    }

    #[test]
    fn unsupported_protocol_returns_error() {
        let err = build_exporter(&cfg("tcp")).err().unwrap();
        assert!(err.to_string().contains("unsupported OTLP protocol"));
    }

    #[test]
    fn grpc_invalid_header_key_is_skipped() {
        // Invalid header key must not cause an error — it is warned and skipped.
        let mut c = cfg("grpc");
        c.headers.insert("invalid header!!!".into(), "value".into());
        with_rt(|| assert!(build_exporter(&c).is_ok()));
    }
}
