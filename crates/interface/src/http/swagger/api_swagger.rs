use crate::http::swagger::security_swagger::SecurityAddon;
use utoipa::OpenApi;

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
