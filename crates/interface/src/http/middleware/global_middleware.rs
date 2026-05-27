use std::time::Duration;

use axum::Router;
use axum::middleware::from_fn;
use domain::config::setting_interface::{CorsSetting, HttpServerSetting};
use shared::constant::path::RUN_MODE_DEVELOPMENT;
use tower_http::{
    cors::CorsLayer,
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::http::middleware::metric_middleware::metric_middleware;

/// Apply all global middleware to the router.
/// `mode` controls request logging: only "development" enables TraceLayer at INFO.
pub fn apply<S>(router: Router<S>, cfg: &HttpServerSetting, mode: &str) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let router = router
        .layer(RequestBodyLimitLayer::new(cfg.limits.max_body_size))
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_millis(cfg.timeouts.read_timeout_ms),
        ))
        .layer(build_cors(&cfg.security.cors))
        .layer(from_fn(metric_middleware));

    if mode == RUN_MODE_DEVELOPMENT {
        router.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
    } else {
        router
    }
}

fn build_cors(cfg: &CorsSetting) -> CorsLayer {
    use std::str::FromStr;
    use tower_http::cors::AllowOrigin;

    if !cfg.enabled {
        return CorsLayer::new();
    }
    if cfg.origin.allow_any_origin {
        return CorsLayer::permissive();
    }

    let origins: Vec<axum::http::HeaderValue> = cfg
        .origin
        .allowed_origins
        .iter()
        .filter_map(|o| axum::http::HeaderValue::from_str(o).ok())
        .collect();

    let methods: Vec<axum::http::Method> = if cfg.methods.allow_any_method {
        vec![
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
            axum::http::Method::HEAD,
        ]
    } else {
        cfg.methods
            .allowed_methods
            .iter()
            .filter_map(|m| axum::http::Method::from_str(m).ok())
            .collect()
    };

    let headers: Vec<axum::http::header::HeaderName> = if cfg.headers.allow_any_header {
        return CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods(methods)
            .allow_headers(tower_http::cors::Any)
            .allow_credentials(cfg.allow_credentials)
            .max_age(Duration::from_secs(cfg.max_age_secs));
    } else {
        cfg.headers
            .allowed_headers
            .iter()
            .filter_map(|h| axum::http::header::HeaderName::from_str(h).ok())
            .collect()
    };

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods(methods)
        .allow_headers(headers)
        .allow_credentials(cfg.allow_credentials)
        .max_age(Duration::from_secs(cfg.max_age_secs))
}
