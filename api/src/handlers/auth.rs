use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::auth::{get_user_by_id, validate_request_with_basic_auth};

use super::ApplicationError;

/// Get(/login) attempts to log a user in, and if successful stores the user in a session variable
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

/// Get(/logout) logs a user out if they are currently logged in
pub async fn logout(id: Identity) -> Result<HttpResponse, ApplicationError> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}
