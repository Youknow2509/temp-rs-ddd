//! OS-signal handling — await SIGINT or SIGTERM.

use anyhow::{Context, Result};

pub async fn wait() -> Result<()> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};

        let mut sigint =
            signal(SignalKind::interrupt()).context("registering SIGINT handler")?;
        let mut sigterm =
            signal(SignalKind::terminate()).context("registering SIGTERM handler")?;

        tokio::select! {
            _ = sigint.recv()  => tracing::info!("received SIGINT"),
            _ = sigterm.recv() => tracing::info!("received SIGTERM"),
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .context("registering Ctrl-C handler")?;
        tracing::info!("received Ctrl-C");
    }

    Ok(())
}
