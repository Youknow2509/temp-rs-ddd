//! Application server: orchestrates the four lifecycle phases.
//!
//!   bootstrap -> wiring -> run -> shutdown

pub mod bootstrap;
pub mod run;
pub mod shutdown;
pub mod wiring;

use anyhow::Result;

pub struct Server;

impl Server {
    /// All four lifecycle phases under the caller's Tokio runtime.
    pub async fn run() -> Result<()> {
        let bootstrap = bootstrap::init().await?;
        let wired = wiring::wire(bootstrap)?;
        run::start(&wired)?;
        shutdown::drain(wired).await
    }
}
