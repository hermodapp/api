use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::{
    auth::validate_request_with_basic_auth,
    db::NewUser,
    jwt::{encode_token, user_or_403},
};

use super::ApplicationResponse;

#[tracing::instrument(name = "handlers::login", skip(request, pool))]
/// Get(/login) attempts to log a user in, and if successful returns a JWT token
pub async fn login(request: web::HttpRequest, pool: web::Data<PgPool>) -> ApplicationResponse {
    let user = validate_request_with_basic_auth(request, &pool).await?;
    let token = encode_token(user.id)?;
    Ok(HttpResponse::Ok().body(token))
}

#[tracing::instrument(name = "handlers::logout", skip())]
/// Get(/logout) logs a user out if they are currently logged in
pub async fn logout() -> ApplicationResponse {
    Ok(HttpResponse::BadRequest().body("Logouts with JWT's are performed client-side"))
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
    let mut new_user = NewUser::new(query.username.clone(), query.password.clone());
    new_user.store(&pool).await?;

    Ok(HttpResponse::Ok().body("New user stored.".to_string()))
}

pub async fn who_am_i(request: HttpRequest, pool: web::Data<PgPool>) -> ApplicationResponse {
    Ok(HttpResponse::Ok().body(user_or_403(request, &pool).await?.username))
}
