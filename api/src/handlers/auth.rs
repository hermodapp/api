use actix_identity::Identity;
use actix_web::http::{header, HeaderValue, StatusCode};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::auth::{get_user_by_id, validate_request_with_basic_auth};

/// GET(/login) attempts to log a user in, and if successful stores the user in a session variable
pub async fn login(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    id: Identity,
) -> Result<HttpResponse, ApplicationError> {
    let user_id = validate_request_with_basic_auth(request, &pool).await?;
    let user = get_user_by_id(user_id.to_string(), &pool).await;
    let s = serde_json::to_string(&user?)
        .map_err(|e| ApplicationError::UnexpectedError(anyhow::anyhow!(e)))?;
    id.remember(s);
    Ok(HttpResponse::Ok().finish())
}

/// GET(/logout) logs a user out if they are currently logged in
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
