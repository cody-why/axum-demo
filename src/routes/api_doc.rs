/*
 * @Author: anger
 * @Date: 2023-12-11 21:18:26
 * @LastEditTime: 2023-12-12 21:17:49
 */

use utoipa::{OpenApi, Modify, openapi::security::{SecurityScheme, ApiKey, ApiKeyValue}};

use crate::RespVO;

use super::user;

#[derive(OpenApi)]
#[openapi(
    paths(
        user::router::login,
    ),
    components(
        schemas(user::dto::LoginPayload, RespVO<String>),
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hello", description = "hello API")
    ),
    servers(
        (url = "http://localhost:8080/api", description = "dev"),
        (url = "https://api.utoipa.com/api/v1", description = "prod")
    ),
    
)]
pub(crate) struct ApiDoc;

pub(crate) struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("token"))),
            )
        }
    }
}