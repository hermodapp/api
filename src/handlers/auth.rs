use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use tracing::field::Empty;
use uuid::Uuid;

use crate::{auth::validate_request_with_basic_auth, db::NewUser, db::User, db::NewForgottenPasswordRequest, jwt::JwtClient, clients::postmark::PostmarkClient};

use super::ApplicationResponse;

#[tracing::instrument(name = "handlers::auth::login", skip(request, pool, jwt_client), fields(username=Empty, user_id=Empty))]
/// Get(/login) attempts to log a user in, and if successful returns a JWT token
pub async fn login(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    jwt_client: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = validate_request_with_basic_auth(request, &pool).await?;
    tracing::Span::current().record("username", &tracing::field::display(&user.username));
    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));
    let token = jwt_client.encode_token(user.id)?;
    Ok(HttpResponse::Ok().body(token))
}

#[tracing::instrument(name = "handlers::auth::logout")]
/// Get(/logout) logs a user out if they are currently logged in
pub async fn logout() -> ApplicationResponse {
    Ok(HttpResponse::BadRequest().body("Logouts with JWT's are performed client-side"))
}

#[derive(serde::Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[tracing::instrument(name = "handlers::auth::register", skip(pool, query), fields(username=%query.username, user_id=Empty))]
pub async fn register(
    pool: web::Data<PgPool>,
    query: web::Form<RegistrationRequest>,
) -> ApplicationResponse {
    let new_user = NewUser::new(query.username.clone(), query.password.clone(), query.email.clone());
    new_user.store(&pool).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&new_user.id));

    Ok(HttpResponse::Ok().body("New user stored.".to_string()))
}

#[tracing::instrument(name = "handlers::auth::whoami", skip(request, jwt_client), fields(username=Empty, user_id=Empty))]
pub async fn who_am_i(
    request: HttpRequest,
    jwt_client: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = jwt_client.user_or_403(request).await?;
    Ok(HttpResponse::Ok().body(user.username))
}

#[derive(Debug, serde::Deserialize)]
pub struct ForgottenPasswordQuery {
    pub username: String,
}

#[tracing::instrument(name = "handlers::auth::forgot_password", skip(postmark_client, request, pool), fields(username=Empty, user_id=Empty))]
/// get(/password/forgot) lets a user request a password reset link to their e-mail.
pub async fn forgot_password(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<ForgottenPasswordQuery>,
    postmark_client: web::Data<PostmarkClient>,
) -> ApplicationResponse {
    let user = sqlx::query!(
        r#"SELECT * FROM account
           WHERE username = $1"#,
        query.username
    ).fetch_one(pool.as_ref()).await?;

    let newfpr = NewForgottenPasswordRequest::new(user.id);
    newfpr.store(pool.as_ref()).await?;
    postmark_client.send_email(&user.email, &format!("http://hermodapp.com/password/reset?id={}", &newfpr.id)).await?;

    Ok(HttpResponse::Ok().body(format!("Successfully generated forgotten password request for user {}.", query.username)))
}

#[derive(serde::Deserialize, Debug)]
pub struct ResetPasswordQuery {
    pub id: Uuid,
}

#[derive(serde::Deserialize, Debug)]
pub struct ResetPasswordRequest {
    pub new_password: String,
}

#[tracing::instrument(name = "handlers::auth::reset_password", skip(query, json, request, pool), fields(username=Empty, user_id=Empty))]
/// get(/password/reset) lets a user reset their password.
pub async fn reset_password(
    request: web::HttpRequest,
    pool: web::Data<PgPool>,
    query: web::Query<ResetPasswordQuery>,
    json: web::Json<ResetPasswordRequest>,
) -> ApplicationResponse {
    let forgotten_password_request = sqlx::query!(
        "SELECT * FROM forgotten_password_request
         WHERE id = $1",
         query.id
    ).fetch_one(pool.as_ref()).await?;

    let user = sqlx::query_as!(User, 
        "SELECT * FROM account
         WHERE id = $1",
        forgotten_password_request.account_id).fetch_one(pool.as_ref()).await?;

    user.change_password(&pool, &json.new_password).await?;

    Ok(HttpResponse::Ok().body(format!("Successfully reset password for user {}.", user.username)))
}