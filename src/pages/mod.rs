mod auth;
use axum::{Router, middleware, response::IntoResponse, routing::get};
use axum_inertia::Inertia;
use axum_login::AuthManagerLayerBuilder;
use http::Method;
use serde_json::json;
use tower_http::cors::CorsLayer;
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};

use crate::{
    config::{ApiContext, auth::get_user},
    models::user::User,
};

#[tracing::instrument(name = "Home Page", skip(i))]
async fn home(i: Inertia, user: User) -> impl IntoResponse {
    i.render("Home", json!({ "user": user }))
}

pub fn routes(api_context: ApiContext) -> Router<ApiContext> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    // Create a session store and layer
    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::days(1)));
    let auth_layer = AuthManagerLayerBuilder::new(api_context.clone(), session_layer).build();

    Router::new()
        .merge(auth::routes())
        .merge(protected_routes())
        .layer(cors)
        .layer(auth_layer)
}

pub fn protected_routes() -> Router<ApiContext> {
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    Router::new()
        .route("/", get(home))
        .layer(cors)
        .route_layer(middleware::from_fn(get_user))
}
