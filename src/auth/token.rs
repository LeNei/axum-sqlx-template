use axum::http::request::Parts;
use axum::response::IntoResponse;
use hyper::header::AUTHORIZATION;
use hyper::StatusCode;
use thiserror::Error;

/// A JWT provided as a bearer token in an `Authorization` header.
#[derive(PartialEq)]
pub(crate) struct Token(String);

impl Token {
    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_request_parts(parts: &mut Parts) -> Result<Self, TokenError> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(TokenError::Missing)?
            .to_str()
            .map_err(|_| TokenError::Missing)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(TokenError::Missing)?;

        Ok(Token(token.to_string()))
    }
}

/// An error with a JWT.
#[derive(Debug, Error, PartialEq)]
pub(crate) enum TokenError {
    /// The token is either malformed or did not pass validation.
    #[error("the token is invalid or malformed: {0:?}")]
    Invalid(jsonwebtoken::errors::Error),

    /// The token header could not be decoded because it was malformed.
    #[error("the token header is malformed: {0:?}")]
    InvalidHeader(jsonwebtoken::errors::Error),

    /// No bearer token found in the `Authorization` header.
    #[error("no bearer token found")]
    Missing,

    /// The token's header does not contain the `kid` attribute used to identify
    /// which decoding key should be used.
    #[error("the token header does not specify a `kid`")]
    MissingKeyId,

    /// The token's `kid` attribute specifies a key that is unknown.
    #[error("token uses the unknown key {0:?}")]
    UnknownKeyId(String),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> axum::response::Response {
        StatusCode::UNAUTHORIZED.into_response()
    }
}
