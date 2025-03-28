use axum::{Router, response::IntoResponse, routing::get};
use axum_inertia::Inertia;
use http::Method;
use serde_json::json;
use tower_http::cors::CorsLayer;

use crate::config::ApiContext;

#[tracing::instrument(name = "Home Page", skip(i))]
async fn home(i: Inertia) -> impl IntoResponse {
    i.render("Home", json!({}))
}

pub fn routes(api_context: ApiContext) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);
    Router::new()
        .route("/", get(home))
        .layer(cors)
        .with_state(api_context)
}
