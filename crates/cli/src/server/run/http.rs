//! HTTP server runtime.

use anyhow::Result;
use infrastructure::state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, task::JoinHandle};
use tracing::info;

pub fn start(app_state: &Arc<AppState>) -> Result<JoinHandle<()>> {
    let state = Arc::clone(app_state);
    let addr = SocketAddr::from((
        state.config.interfaces.http_server.network.host,
        state.config.interfaces.http_server.network.port,
    ));
    let router = interface::http::router(Arc::clone(&state));

    Ok(tokio::spawn(async move {
        let listener = TcpListener::bind(addr)
            .await
            .expect("failed to bind HTTP listener");
        info!(%addr, "HTTP server listening");
        axum::serve(listener, router)
            .await
            .expect("HTTP server error");
    }))
}
