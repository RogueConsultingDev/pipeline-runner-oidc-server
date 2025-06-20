use axum::Router;
use axum::extract::{FromRef, MatchedPath};
use axum::http::Request;
use axum::routing::get;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, info_span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::{ApiConfig, get_config};
use crate::errors::Result;
use crate::models::{JWKSchema, JWKSetSchema, OpenIDConfigurationSchema};
use crate::routes::{get_jwk_set, get_openid_configuration};

mod config;
mod errors;
mod models;
mod routes;

#[derive(Debug, Clone)]
struct AppState {
    openid_config: OpenIDConfigurationSchema,
    jwks: JWKSetSchema,
}

impl FromRef<AppState> for OpenIDConfigurationSchema {
    fn from_ref(state: &AppState) -> Self {
        state.openid_config.clone()
    }
}

impl FromRef<AppState> for JWKSetSchema {
    fn from_ref(state: &AppState) -> Self {
        state.jwks.clone()
    }
}

fn create_app(config: Option<ApiConfig>) -> Result<Router> {
    let config = match config {
        Some(c) => c,
        None => get_config()?,
    };

    let openid_config = OpenIDConfigurationSchema::new(
        config.issuer.clone(),
        format!("{}/.well-known/jwks", &config.issuer),
    );

    let jwks = JWKSetSchema::new(vec![JWKSchema::from(config.public_key.as_str())]);

    let state = AppState {
        openid_config,
        jwks,
    };

    let app = Router::new()
        .route(
            "/.well-known/openid-configuration",
            get(get_openid_configuration),
        )
        .route("/.well-known/jwks", get(get_jwk_set))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    some_other_field = tracing::field::Empty,
                )
            }),
        )
        .with_state(state);

    Ok(app)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,oidc_server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = get_config()?;
    debug!("Config: {:?}", config);

    let app = create_app(Some(config))?;

    let addr = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(addr).await?;

    info!("Listening on {}", addr);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
