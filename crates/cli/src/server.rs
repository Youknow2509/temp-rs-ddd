//! Application server: orchestrates the four lifecycle phases.
//!
//!   bootstrap -> wiring -> run -> shutdown

pub mod bootstrap;
pub mod run;
pub mod shutdown;
pub mod wiring;

use anyhow::Result;

use self::bootstrap::Bootstrap;

#[derive(Debug)]
pub struct Server {
    bootstrap: Bootstrap,
}

impl Server {
    /// Phase 1 — load config, init telemetry, open connection pools.
    pub fn bootstrap() -> Result<Self> {
        let bootstrap = bootstrap::init()?;
        Ok(Self { bootstrap })
    }

    /// Phase 2 -> 3 -> 4 — wire DDD layers, start interfaces, wait for shutdown.
    pub fn run(self) -> Result<()> {
        let wired = wiring::wire(self.bootstrap)?;
        run::start(&wired)?;
        shutdown::wait()?;
        Ok(())
    }
}
