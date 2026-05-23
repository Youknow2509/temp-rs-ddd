//! Cronjob runner: owns config + the schedule of background jobs.

use anyhow::{Context, Result};

use domain::config::SystemConfig;
use infrastructure::config;

#[derive(Debug)]
pub struct Cronjob {
    #[allow(unused)]
    config: SystemConfig,
}

impl Cronjob {
    /// Build the runner: load config, then wire collaborators.
    /// All I/O happens here so `main` stays a one-liner.
    pub fn bootstrap() -> Result<Self> {
        let config = config::load().context("loading system config")?;
        Ok(Self { config })
    }

    /// Register the schedule and block until shutdown.
    pub fn run(self) -> Result<()> {
        // TODO: register + run scheduled jobs using `self.config`.
        println!("cronjob bootstrapped: {:#?}", self.config);
        Ok(())
    }
}
