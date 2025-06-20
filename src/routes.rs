use axum::Json;
use axum::extract::State;

use crate::models::{JWKSetSchema, OpenIDConfigurationSchema};

pub(super) async fn get_openid_configuration(
    State(openid_config): State<OpenIDConfigurationSchema>,
) -> Json<OpenIDConfigurationSchema> {
    Json(openid_config)
}

pub(super) async fn get_jwk_set(State(jwks): State<JWKSetSchema>) -> Json<JWKSetSchema> {
    Json(jwks)
}
