use utoipa::OpenApi;

use crate::http::swagger::security_swagger::SecurityAddon;

const SYSTEM_TAG: &str = "system";

/// Swagger documentation for the API
#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    tags(
        (name = SYSTEM_TAG, description = "System API endpoints"),
    )
)]
pub struct SwaggerApiDoc;
