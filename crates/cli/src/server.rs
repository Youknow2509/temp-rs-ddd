//! Application server: orchestrates the three lifecycle phases.
//!
//!   bootstrap -> run -> shutdown

pub mod bootstrap;
pub mod run;
pub mod shutdown;

use anyhow::Result;

pub struct Server;

impl Server {
    pub async fn run() -> Result<()> {
        let bootstrap = bootstrap::init().await?;
        let handles = run::start(&bootstrap.app_state)?;
        shutdown::drain(bootstrap, handles).await
    }
}
