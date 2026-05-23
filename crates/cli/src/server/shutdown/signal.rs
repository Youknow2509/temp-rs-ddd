//! OS-signal handling — block until SIGINT or SIGTERM fires.

use anyhow::{Context, Result};

pub fn wait() -> Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("building signal-listener runtime")?;

    rt.block_on(wait_async())
}

async fn wait_async() -> Result<()> {
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
