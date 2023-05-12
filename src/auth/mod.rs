pub mod claims;
pub mod token;

use crate::routes::ApiContext;
use axum::extract::State;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use claims::{Claims, KeycloakClaims};
use hyper::StatusCode;

pub(crate) async fn auth_user<B>(
    State(ctx): State<ApiContext>,
    Claims(claims): Claims<KeycloakClaims>,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    /*
    let user = AuthUser::get_from_db(&claims.sub, &ctx.db, None)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(user);
    */
    Ok(next.run(req).await)
}
