use crate::config::jwks::Jwks;
use crate::config::Settings;
use crate::routes::{build_routes, ApiContext};
use anyhow::Context;
use axum::Router;
use std::net::TcpListener;

pub async fn build(settings: Settings) -> anyhow::Result<()> {
    let api_context = ApiContext {
        db: settings.database.get_connection_pool(),
        jwks: Jwks::from_authority(&settings.jwks.url, settings.jwks.authority).await?,
    };
    let api_router = build_routes(api_context);
    let address = format!(
        "{}:{}",
        settings.application.host, settings.application.port
    );
    let listener = TcpListener::bind(address).context("Failed to bind to port")?;

    run(api_router, listener).await
}

async fn run(router: Router, listener: TcpListener) -> anyhow::Result<()> {
    axum::Server::from_tcp(listener)?
        .serve(router.into_make_service())
        .await
        .context("Failed to start server")
}
