//! HTTP server runtime.

use anyhow::Result;

use crate::server::wiring::Wired;

pub fn start(_wired: &Wired) -> Result<()> {
    // TODO: bind axum/hyper to `wired.bootstrap.config.interfaces.http_server`,
    // mount routes that call into `wired.use_cases`.
    println!("[run::http] (stub) HTTP server would start here");
    Ok(())
}
