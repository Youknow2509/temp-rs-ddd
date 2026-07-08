//! Hierarchical config loader.
//!
//! Layers (later overrides earlier):
//!   1. `config/default.toml`         — committed baseline
//!   2. `config/<run_mode>.toml`      — picked from `APP_MODE` (default: "production")
//!   3. `config/local.toml`           — gitignored, machine-local overrides
//!   4. environment variables         — `APP__SECTION__KEY=...` (double underscore)
//!
//! `APP_MODE` is used only as a selector and is NOT applied as an override
//! (the env source uses `__` as the prefix separator, so single-underscore
//! variables like `APP_MODE` are ignored when parsing overrides).

use anyhow::{Context, Result};
use config::{Config, Environment, File};
use domain::config::SystemConfig;
use shared::constant::{
    env::ENV_RUN_MODE,
    path::{CONFIG_BASE_DEFAULT, CONFIG_BASE_LOCAL, CONFIG_DIR, RUN_MODE_PRODUCTION},
};
use std::env;

const ENV_PREFIX: &str = "APP";
const ENV_SEPARATOR: &str = "__";

/// Load `SystemConfig` from the hierarchical sources described above.
pub fn load() -> Result<SystemConfig> {
    let run_mode = env::var(ENV_RUN_MODE).unwrap_or_else(|_| RUN_MODE_PRODUCTION.into());
    let mode_base = format!("{CONFIG_DIR}/{run_mode}");

    Config::builder()
        .add_source(File::with_name(CONFIG_BASE_DEFAULT))
        .add_source(File::with_name(&mode_base).required(false))
        .add_source(File::with_name(CONFIG_BASE_LOCAL).required(false))
        .add_source(
            Environment::with_prefix(ENV_PREFIX)
                .prefix_separator(ENV_SEPARATOR)
                .separator(ENV_SEPARATOR),
        )
        .build()
        .with_context(|| format!("building config (run_mode = {run_mode})"))?
        .try_deserialize::<SystemConfig>()
        .context("deserializing SystemConfig from merged config")
}
