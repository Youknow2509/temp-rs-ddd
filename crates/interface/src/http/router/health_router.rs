use std::sync::Arc;
// use axum::{Router, routing::get};
use crate::{http::handle::health_handle, state::AppState};
use utoipa_axum::{router::OpenApiRouter, routes as utoipa_routes};

pub fn routes(state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(utoipa_routes!(health_handle::get_health))
        .with_state(state)
}

// pub fn routes() -> Router<Arc<AppState>> {
//     Router::new().route("/health", get(health_handle::get_health))
// }
