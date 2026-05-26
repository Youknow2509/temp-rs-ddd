use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

/// Per-group authentication middleware.
/// Mount on a route group with `.route_layer(axum::middleware::from_fn(require_auth))`.
pub async fn require_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    // TODO: validate bearer token / session from req.headers()
    Ok(next.run(req).await)
}
