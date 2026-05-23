//! Load the merged `SystemConfig` from the hierarchical sources.

use anyhow::{Context, Result};

use domain::config::SystemConfig;
use infrastructure::config;

pub fn load() -> Result<SystemConfig> {
    config::load().context("loading system config")
}
