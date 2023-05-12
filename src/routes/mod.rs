use axum::middleware;
use axum::{routing::get, Router};
use http::Method;
use hyper::StatusCode;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::auth::auth_user;
use crate::config::jwks::Jwks;

#[tracing::instrument(name = "Ping")]
async fn ping() -> StatusCode {
    StatusCode::OK
}

#[derive(Clone)]
pub(crate) struct ApiContext {
    pub db: PgPool,
    pub jwks: Jwks,
}

pub(crate) fn build_routes(api_context: ApiContext) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    Router::new()
        .route("/ping", get(ping))
        .nest("/auth", build_auth_routes(&api_context))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(api_context)
}

fn build_auth_routes(api_context: &ApiContext) -> Router<ApiContext> {
    Router::new()
        .route("/ping", get(ping))
        .route_layer(middleware::from_fn_with_state(
            api_context.clone(),
            auth_user,
        ))
}
