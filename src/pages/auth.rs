use axum::{
    Json, Router,
    extract::State,
    response::{IntoResponse, Redirect},
    routing::get,
};
use axum_inertia::Inertia;
use http::StatusCode;
use serde_json::json;

use crate::{
    config::{
        ApiContext,
        auth::{AuthSession, Credentials},
    },
    models::{InertiaError, user::NewUser},
};

#[tracing::instrument(name = "Login Page", skip(i))]
async fn login_page(i: Inertia) -> impl IntoResponse {
    i.render("Login", json!({}))
}

#[tracing::instrument(name = "Login attempt", skip(auth_session, creds))]
async fn login(mut auth_session: AuthSession, Json(creds): Json<Credentials>) -> impl IntoResponse {
    let user = match auth_session.authenticate(creds.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            let mut login_url = "/login".to_string();
            if let Some(next) = creds.next {
                login_url = format!("{}?next={}", login_url, next);
            };

            return Redirect::to(&login_url).into_response();
        }
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&user).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    if let Some(ref next) = creds.next {
        Redirect::to(next)
    } else {
        Redirect::to("/")
    }
    .into_response()
}

#[tracing::instrument(name = "Register page", skip(i))]
async fn register_page(i: Inertia) -> impl IntoResponse {
    i.render("Register", json!({}))
}

#[tracing::instrument(name = "Registration attempt", skip(ctx, new_user))]
async fn register(
    State(ctx): State<ApiContext>,
    Json(new_user): Json<NewUser>,
) -> Result<impl IntoResponse, InertiaError> {
    let mut conn = ctx.db.acquire().await?;
    new_user.register(&mut conn).await?;
    Ok(Redirect::to("/login").into_response())
}

#[tracing::instrument(name = "Logout", skip(auth_session))]
async fn logout(mut auth_session: AuthSession) -> Result<impl IntoResponse, InertiaError> {
    match auth_session.logout().await {
        Ok(_) => Ok(Redirect::to("/login").into_response()),
        Err(_) => Err(InertiaError::Unknown),
    }
}
pub fn routes() -> Router<ApiContext> {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/register", get(register_page).post(register))
        .route("/logout", get(logout))
}
