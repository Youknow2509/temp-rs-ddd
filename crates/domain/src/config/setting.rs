use std::path::PathBuf;

use serde::Deserialize;

/// System setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SystemSetting {
    name: String,
    version: String,
    region: String,
    shard_id: String,
    timezone: String,
    mode: String,
}

/// --- TLS Setting (shared) ---
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TLSSetting {
    is_enabled: bool,
    cert_file: PathBuf,
    key_file: PathBuf,
    client_ca_file: Option<PathBuf>,
    require_client_cert: bool,
    min_version: String,
}
