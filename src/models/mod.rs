pub mod user;

use axum::response::{IntoResponse, Redirect, Response};
use tokio::task;

#[derive(Debug, thiserror::Error)]
pub enum InertiaError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),

    #[error("Something went wrong")]
    Unknown,
}

impl IntoResponse for InertiaError {
    fn into_response(self) -> Response {
        tracing::error!("Error: {:?}", self);
        Redirect::to("/error").into_response()
    }
}
