use crate::config::Settings;
use crate::routes::{build_routes, ApiContext};
use anyhow::Context;
use axum::Router;
use tokio::net::TcpListener;

pub async fn build(settings: Settings) -> anyhow::Result<()> {
    let api_context = ApiContext {
        db: settings.database.get_connection_pool(),
    };
    let api_router = build_routes(api_context);
    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let listener = TcpListener::bind(address).await.context("Failed to bind to port")?;

    run(api_router, listener).await
}

async fn run(router: Router, listener: TcpListener) -> anyhow::Result<()> {
    axum::serve(listener, router)
        .await
        .context("Failed to start server")
}
