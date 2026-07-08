pub mod health_router;

use super::middleware::global_middleware;
use crate::http::swagger::api_swagger::SwaggerApiDoc;
use axum::Router;
use infrastructure::state::AppState;
use shared::constant::path::RUN_MODE_DEVELOPMENT;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as _};
use utoipa_swagger_ui::SwaggerUi;

// Compose all routes and apply global middleware.
//
pub fn mount(state: Arc<AppState>) -> Router {
    let cfg = &state.config.interfaces.http_server;

    // Level 1: Public routes - no auth, global middleware only.
    // let public = Router::new().merge(health_router::routes());
    let public = health_router::routes(state.clone());

    // Level 2: Protected - single middleware.
    // let user_routes = Router::new()
    //     .merge(user_router::routes())
    //     .nest("/users/:id", user_post_router::routes()) // sub-resource
    //     .route_layer(axum::middleware::from_fn(
    //         super::middleware::auth_middleware::require_auth,
    //     ));

    // Level 3: Protected - double middleware (auth then role check).
    // require_auth runs first; require_admin runs only if auth passes.
    // let admin_routes = Router::new()
    //     .merge(admin_setting_router::routes()) // GET /admin/settings
    //     .route_layer(axum::middleware::from_fn(
    //         super::middleware::auth_middleware::require_admin,
    //     ))
    //     .route_layer(axum::middleware::from_fn(
    //         super::middleware::auth_middleware::require_auth,
    //     ));

    // Level 4: Version prefix - group protected + admin under /api/v1.
    // nest() adds the prefix without changing inner middleware.
    // let v1 = Router::new()
    //     .merge(user_routes)
    //     .nest("/admin", admin_routes);

    // Level 5: Multi-version - run v1 and v2 in parallel.
    // Each version is an independent sub-tree with its own middleware.
    // let v2 = Router::new()
    //     .merge(user_v2_router::routes())
    //     .route_layer(axum::middleware::from_fn(
    //         super::middleware::auth_middleware::require_auth,
    //     ));

    // Assemble the full tree. Create router and ensure OpenAPI includes public OpenApiRouter routes
    let mode = state.config.system.mode.clone();
    let router = create_router(mode.clone(), state.clone()).expect("Failed to create router");
    let router = if mode.as_str() == RUN_MODE_DEVELOPMENT {
        router
    } else {
        router.merge(public)
    };
    // .nest("/api/v1", v1)
    // .nest("/api/v2", v2)

    // Global middleware wraps the entire tree. with_state finalizes the state type.
    global_middleware::apply(router, cfg, &state.config.system.mode)
}

/// Create router with mode system. Dev create OpenApiRouter or Axum router
fn create_router(mode: String, state: Arc<AppState>) -> Result<Router, String> {
    match mode.as_str() {
        RUN_MODE_DEVELOPMENT => {
            // Start with the base OpenApiRouter from the derive
            let base = OpenApiRouter::with_openapi(SwaggerApiDoc::openapi());
            // Merge public OpenApiRouter routes (health, etc.) so the generated OpenAPI
            // contains routes registered via `utoipa_routes!`.
            let openapi_router = base.merge(health_router::routes(state));
            let (router, api) = openapi_router.split_for_parts();
            let router = router
                .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone()))
                .merge(Redoc::with_url("/redoc", api.clone()))
                // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
                // via SwaggerUi instead we only make rapidoc to point to the existing doc.
                .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
                // Alternative to above
                // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", api).path("/rapidoc"))
                .merge(Scalar::with_url("/scalar", api));
            Ok(router)
        }
        _ => {
            let router = Router::new();
            Ok(router)
        }
    }
}
