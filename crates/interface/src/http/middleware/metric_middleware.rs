use axum::{
    extract::{MatchedPath, Request},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use metrics::{counter, histogram};
use std::time::Instant;

/// Per-group metric middleware.
/// Mount on a route group with `axum::middleware::from_fn(metric_middleware)`.
pub async fn metric_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let method = req.method().as_str().to_owned();

    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|matched| matched.as_str().to_owned())
        .unwrap_or_else(|| req.uri().path().to_owned());

    let start = Instant::now();
    let response = next.run(req).await;

    let status = response.status().as_u16().to_string();
    let elapsed = start.elapsed().as_secs_f64();

    counter!(
        "http_requests_total",
        "method" => method.clone(),
        "path" => path.clone(),
        "status" => status.clone()
    )
    .increment(1);

    histogram!(
        "http_request_duration_seconds",
        "method" => method,
        "path" => path,
        "status" => status
    )
    .record(elapsed);

    Ok(response)
}
