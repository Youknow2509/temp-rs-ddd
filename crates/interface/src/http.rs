pub mod dto;
pub mod handle;
pub mod middleware;
pub mod response;
pub mod router;
pub mod swagger;

use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    self::router::mount(state)
}
