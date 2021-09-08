use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::{auth::validate_request_with_basic_auth, db::NewUser};

use super::{ApplicationError, ApplicationResponse};

#[tracing::instrument(name = "handlers::login", skip(request, pool, id))]
/// Get(/login) attempts to log a user in, and if successful stores the user in a session variable
pub async fn login(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    id: Identity,
) -> ApplicationResponse {
    let user = validate_request_with_basic_auth(request, &pool).await?;
    let s = serde_json::to_string(&user)
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

#[derive(serde::Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,
}

pub async fn register(
    pool: web::Data<PgPool>,
    query: web::Form<RegistrationRequest>,
) -> ApplicationResponse {
    let mut new_user = NewUser::default();
    new_user.username = query.username.clone();
    new_user.password = query.password.clone();
    new_user.store(&pool).await?;

    Ok(HttpResponse::Ok().body("New user stored.".to_string()))
}
