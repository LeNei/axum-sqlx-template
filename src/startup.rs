use crate::api::routes as api_routes;
use crate::config::inertia::get_inertia_config;
use crate::config::{ApiContext, Settings};
use crate::pages::routes as page_routes;
use tower_http::services::ServeDir;

use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub async fn build(settings: Settings) -> anyhow::Result<()> {
    let api_context = ApiContext {
        db: settings.database.get_connection_pool(),
        inertia: get_inertia_config(settings.is_dev),
    };

    tracing::info!("Creating router...");
    let mut router = Router::new()
        .merge(page_routes(api_context.clone()))
        .nest("/api", api_routes(api_context))
        .layer(TraceLayer::new_for_http());

    if !settings.is_dev {
        // Serve static assets in production from the frontend/dist/assets directory instead of Vite
        let service = ServeDir::new("frontend/dist/assets").precompressed_gzip();
        router = router.nest_service("/assets", service);
    }

    let public_service = match settings.is_dev {
        true => ServeDir::new("frontend/public"),
        false => ServeDir::new("frontend/dist"),
    };
    router = router.fallback_service(public_service);

    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );

    tracing::info!("Binding to address: {}", address);
    let listener = TcpListener::bind(address)
        .await
        .context("Failed to bind to port")?;

    run(router, listener).await
}

async fn run(router: Router, listener: TcpListener) -> anyhow::Result<()> {
    axum::serve(listener, router)
        .await
        .context("Failed to start server")
}
