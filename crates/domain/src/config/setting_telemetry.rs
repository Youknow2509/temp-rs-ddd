use std::collections::HashMap;
use std::net::IpAddr;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TelemetrySystemSetting {
    pub logger: LoggerSetting,
    pub metrics: MetricsSetting,
    pub tracing: TracingSetting,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LoggerSetting {
    pub level: String,
    pub output: Vec<String>,
    pub caller: bool,
    pub stacktrace_level: String,
    pub file: LoggerFileSetting,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct LoggerFileSetting {
    pub enabled: bool,
    pub folder: String,
    pub filename: String,
    pub max_size_mb: u64,
    pub max_backups: u32,
    pub max_age_days: u32,
    pub compress: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct MetricsSetting {
    pub enabled: bool,
    pub path: String,
    pub port: u16,
    pub host: IpAddr,
    pub namespace: String,
    pub collect_interval_secs: u64,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TracingSetting {
    pub enabled: bool,
    pub service_name: String,
    pub service_version: String,
    pub otlp_endpoint: String,
    pub protocol: String,
    pub sampling_ratio: f64,
    pub propagation: Vec<String>,
    pub batch: TracingBatchSetting,
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TracingBatchSetting {
    pub max_queue_size: usize,
    pub max_export_batch_size: usize,
    pub schedule_delay_ms: u64,
    pub export_timeout_ms: u64,
}
