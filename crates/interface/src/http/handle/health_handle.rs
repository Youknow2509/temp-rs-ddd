use std::sync::Arc;

use axum::extract::{Query, State};
use serde_json::{Value, json};

use crate::{
    http::{dto::health_dto::HealthQuery, response::ApiResponse},
    state::AppState,
};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Server is healthy"),
    )
)]
pub async fn get_health(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<HealthQuery>,
) -> ApiResponse<Value> {
    // TODO: HealthCheckUseCase::new(&state.connections).execute().await
    if params.verbose.unwrap_or(false) {
        return ApiResponse::ok(json!({
            "status": "ok",
            "version": env!("CARGO_PKG_VERSION"),
        }));
    }
    ApiResponse::ok(json!({ "status": "ok" }))
}
