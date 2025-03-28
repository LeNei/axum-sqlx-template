use axum::{
    Extension, RequestPartsExt,
    body::Body,
    extract::{FromRequestParts, Request},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_login::{AuthnBackend, UserId};
use http::request::Parts;
use password_auth::verify_password;
use serde::Deserialize;
use tokio::task;
use ts_rs::TS;

use crate::models::{InertiaError, user::User};

use super::ApiContext;

// This allows us to extract the authentication fields from forms. We use this
// to authenticate requests with the backend.
#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct Credentials {
    pub username: String,
    pub password: String,
    pub next: Option<String>,
}

#[async_trait::async_trait]
impl AuthnBackend for ApiContext {
    type User = User;
    type Credentials = Credentials;
    type Error = InertiaError;

    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            "select * from users where username = $1",
            creds.username
        )
        .fetch_optional(&self.db)
        .await?;

        // Verifying the password is blocking and potentially slow, so we'll do so via
        // `spawn_blocking`.
        task::spawn_blocking(|| {
            // We're using password-based authentication--this works by comparing our form
            // input with an argon2 password hash.
            Ok(user.filter(|user| verify_password(creds.password, &user.password).is_ok()))
        })
        .await?
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(User, "select * from users where id = $1", user_id)
            .fetch_optional(&self.db)
            .await?;

        Ok(user)
    }
}

pub async fn get_user(auth_session: AuthSession, mut req: Request<Body>, next: Next) -> Response {
    match auth_session.user {
        Some(user) => {
            req.extensions_mut().insert(user);
            next.run(req).await
        }
        None => Redirect::to("/login").into_response(),
    }
}

impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    type Rejection = Response;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Response> {
        let Extension(user) = parts
            .extract::<Extension<User>>()
            .await
            .map_err(|_| Redirect::to("/login").into_response())?;
        Ok(user)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend here.
pub type AuthSession = axum_login::AuthSession<ApiContext>;
