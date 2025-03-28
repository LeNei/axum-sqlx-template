use axum::{Router, routing::get};
use http::{Method, StatusCode};
use tower_http::cors::{Any, CorsLayer};

use crate::config::ApiContext;

#[tracing::instrument(name = "Ping")]
async fn ping() -> StatusCode {
    StatusCode::OK
}

pub fn routes(api_context: ApiContext) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    Router::new()
        .route("/ping", get(ping))
        .layer(cors)
        .with_state(api_context)
}
