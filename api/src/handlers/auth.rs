use actix_identity::Identity;
use actix_web::http::{header, HeaderValue, StatusCode};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::auth::validate_request_with_basic_auth;

pub async fn login(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    id: Identity,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = validate_request_with_basic_auth(request, &pool).await?;
    id.remember(user_id.to_string());
    Ok(HttpResponse::Ok().finish())
}

pub async fn logout(id: Identity) -> Result<HttpResponse, ApplicationError> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

#[derive(thiserror::Error)]
pub enum ApplicationError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::UnexpectedError(_) => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
            Self::AuthError(_) => {
                let mut response = HttpResponse::new(StatusCode::UNAUTHORIZED);
                let header_value = HeaderValue::from_str(r#"Basic realm="publish""#).unwrap();
                response
                    .headers_mut()
                    .insert(header::WWW_AUTHENTICATE, header_value);
                response
            }
        }
    }
}

impl std::fmt::Debug for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::handlers::error_chain_fmt(self, f)
    }
}
