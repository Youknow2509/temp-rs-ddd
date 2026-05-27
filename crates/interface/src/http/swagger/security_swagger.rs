use utoipa::{
    Modify,
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
};
/// Sercurity scheme for the API
pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Bearer",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}
