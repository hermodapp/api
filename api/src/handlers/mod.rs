//! Contains HTTP Handlers that directly receive and respond to requests to the server.
mod auth;
mod health_check;
mod index;

use actix_web::{
    http::{header, HeaderValue, StatusCode},
    HttpResponse, ResponseError,
};
pub use auth::*;
pub use health_check::*;
pub use index::*;

use crate::auth::AuthenticationError;

pub type ApplicationResponse = Result<HttpResponse, ApplicationError>;

/// Error derived while handling an HTTP request
#[derive(thiserror::Error)]
pub enum ApplicationError {
    #[error("Authentication failed.")]
    AuthError(#[source] AuthenticationError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UnexpectedError(e) => {
                tracing::error!("{:?}", e);
                HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
            }
            Self::AuthError(e) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                tracing::error!("{:?}", e);
                response
            }
        }
    }
}

impl std::fmt::Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error::error_chain_fmt(self, f)
    }
}
