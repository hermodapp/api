use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::auth::{get_user_by_id, validate_request_with_basic_auth};

use super::{ApplicationError, ApplicationResponse};

#[tracing::instrument(name = "handlers::login", skip(request, pool, id))]
/// Get(/login) attempts to log a user in, and if successful stores the user in a session variable
pub async fn login(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    id: Identity,
) -> ApplicationResponse {
    let user_id = validate_request_with_basic_auth(request, &pool).await?;
    let user = get_user_by_id(user_id.to_string(), &pool).await;
    let s = serde_json::to_string(&user?)
        .map_err(|e| ApplicationError::UnexpectedError(anyhow::anyhow!(e)))?;
    id.remember(s);
    Ok(HttpResponse::Ok().finish())
}

#[tracing::instrument(name = "handlers::logout", skip(id))]
/// Get(/logout) logs a user out if they are currently logged in
pub async fn logout(id: Identity) -> ApplicationResponse {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}

pub async fn register() -> ApplicationResponse {
    Ok(HttpResponse::Ok().finish())
}
