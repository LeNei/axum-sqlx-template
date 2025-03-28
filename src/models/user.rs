use axum_login::AuthUser;
use chrono::{DateTime, Utc};
use password_auth::generate_hash;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgConnection, prelude::FromRow, types::Uuid};
use ts_rs::TS;

use super::InertiaError;

#[derive(FromRow, Serialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip)]
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Here we've implemented `Debug` manually to avoid accidentally logging the
// password hash.
impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("username", &self.username)
            .field("password", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes() // We use the password hash as the auth
        // hash--what this means
        // is when the user changes their password the
        // auth session becomes invalid.
    }
}

#[derive(Deserialize, Clone, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub async fn register(self, conn: &mut PgConnection) -> Result<User, Error> {
        let password = generate_hash(&self.password);

        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING *
            "#,
            self.username,
            password,
        )
        .fetch_one(conn)
        .await
    }
}
