use super::exporter::build_exporter;
use anyhow::Result;
use domain::config::TracingSetting;
use opentelemetry::KeyValue;
use opentelemetry_sdk::{
    Resource, runtime,
    trace::{
        BatchConfigBuilder, Sampler, SdkTracerProvider,
        span_processor_with_async_runtime::BatchSpanProcessor,
    },
};
use std::time::Duration;

pub(super) fn build_provider(cfg: &TracingSetting) -> Result<SdkTracerProvider> {
    let resource = Resource::builder_empty()
        .with_attributes(vec![
            KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                cfg.service_name.clone(),
            ),
            KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_VERSION,
                cfg.service_version.clone(),
            ),
        ])
        .build();

    let batch_cfg = BatchConfigBuilder::default()
        .with_max_queue_size(cfg.batch.max_queue_size)
        .with_max_export_batch_size(cfg.batch.max_export_batch_size)
        .with_scheduled_delay(Duration::from_millis(cfg.batch.schedule_delay_ms))
        .with_max_export_timeout(Duration::from_millis(cfg.batch.export_timeout_ms))
        .build();

    let exporter = build_exporter(cfg)?;

    let batch_processor = BatchSpanProcessor::builder(exporter, runtime::Tokio)
        .with_batch_config(batch_cfg)
        .build();

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_sampler(Sampler::TraceIdRatioBased(cfg.sampling_ratio))
        .with_span_processor(batch_processor)
        .build();

    Ok(provider)
}
