pub mod dto;
pub mod handle;
pub mod middleware;
pub mod response;
pub mod router;
pub mod swagger;

use axum::Router;
use infrastructure::state::AppState;
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    self::router::mount(state)
}
