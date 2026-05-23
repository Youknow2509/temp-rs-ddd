use std::path::PathBuf;

use serde::Deserialize;

/// System setting
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SystemSetting {
    pub name: String,
    pub version: String,
    pub region: String,
    pub shard_id: String,
    pub timezone: String,
    pub mode: String,
}

/// --- TLS Setting (shared) ---
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TLSSetting {
    pub is_enabled: bool,
    pub cert_file: PathBuf,
    pub key_file: PathBuf,
    pub client_ca_file: Option<PathBuf>,
    pub require_client_cert: bool,
    pub min_version: String,
}
