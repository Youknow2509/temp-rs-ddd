pub mod health_router;
// pub mod user_router;
// pub mod user_post_router;
// pub mod admin_router;
// pub mod admin_setting_router;

use std::sync::Arc;

use axum::Router;

use crate::state::AppState;

use super::middleware::global_middleware;

// Compose all routes and apply global middleware.
//
// Request execution order (outer to inner):
//   TraceLayer -> CorsLayer -> BodyLimitLayer -> [route_layer] -> handler
//
// Current route tree:
//   GET /health                       (public)
//
// Extension patterns (uncomment each block when ready):
//   GET  /api/v1/users                (protected - require_auth)
//   POST /api/v1/users
//   GET  /api/v1/users/:id
//   GET  /api/v1/users/:id/posts      (nested sub-resource)
//   POST /api/v1/users/:id/posts
//   GET  /api/v1/admin/settings       (protected - require_auth + require_admin)
pub fn mount(state: Arc<AppState>) -> Router {
    let cfg = &state.config.interfaces.http_server;

    // Level 1: Public routes - no auth, global middleware only.
    // Suitable for health, metrics, docs.
    let public = Router::new().merge(health_router::routes()); // GET /health

    // Level 2: Protected - single middleware.
    // route_layer ensures OPTIONS preflight is not blocked by auth.
    //
    // let user_routes = Router::new()
    //     .merge(user_router::routes())
    //     .nest("/users/:id", user_post_router::routes()) // sub-resource
    //     .route_layer(axum::middleware::from_fn(
    //         super::middleware::auth_middleware::require_auth,
    //     ));

    // Level 3: Protected - double middleware (auth then role check).
    // require_auth runs first; require_admin runs only if auth passes.
    //
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
    //
    // let v1 = Router::new()
    //     .merge(user_routes)
    //     .nest("/admin", admin_routes);

    // Level 5: Multi-version - run v1 and v2 in parallel.
    // Each version is an independent sub-tree with its own middleware.
    //
    // let v2 = Router::new()
    //     .merge(user_v2_router::routes())
    //     .route_layer(axum::middleware::from_fn(
    //         super::middleware::auth_middleware::require_auth,
    //     ));

    // Assemble the full tree.
    let router = Router::new().merge(public);
    // .nest("/api/v1", v1)
    // .nest("/api/v2", v2)

    // Global middleware wraps the entire tree. with_state finalizes the state type.
    global_middleware::apply(router, cfg, &state.config.system.mode).with_state(state)
}
