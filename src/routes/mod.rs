use axum::{routing::get, Router};
use http::Method;
use hyper::StatusCode;
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tracing::instrument(name = "Ping")]
async fn ping() -> StatusCode {
    StatusCode::OK
}

#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}

pub fn build_routes(api_context: ApiContext) -> Router {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    Router::new()
        .route("/", get(ping))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(api_context)
}
