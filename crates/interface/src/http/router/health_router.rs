use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{http::handle::health_handle, state::AppState};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/health", get(health_handle::get_health))
}
