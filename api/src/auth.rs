//! Contains methods used for user authentication and authorization.
use std::str::FromStr;

use actix_web::http::HeaderMap;
use actix_web::HttpRequest;
use anyhow::Context;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{db::User, handlers::ApplicationError};

/// Fetches a user from the database with the given `user_id`.
pub async fn get_user_by_id(user_id: String, db_pool: &PgPool) -> Result<User, anyhow::Error> {
    let user_id = Uuid::from_str(&user_id)?;
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE user_id=$1", user_id)
        .fetch_one(db_pool)
        .await
        .context(format!(
            "Failed to fetch user with user_id {}",
            user_id.to_string()
        ))?;
    Ok(user)
}

/// Validates an HTTP request with request headers
/// conforming to the [Basic Auth RFC](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication).
pub async fn validate_request_with_basic_auth(
    request: HttpRequest,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthenticationError> {
    let credentials =
        extract_from_headers(request.headers()).map_err(|_| AuthenticationError::InvalidHeaders)?;
    let user_id = validate_credentials(credentials, pool).await?;
    Ok(user_id)
}

async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthenticationError> {
    let mut user_id = None;
    let mut expected_password_hash = "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
        .to_string();

    if let Some((stored_user_id, stored_password_hash)) =
        get_stored_credentials(&credentials.username, pool)
            .await
            .map_err(AuthenticationError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    actix_web::rt::task::spawn_blocking(move || {
        verify_password_hash(expected_password_hash, credentials.password)
    })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(AuthenticationError::UnexpectedError)??;

    user_id.ok_or(AuthenticationError::InvalidCredentials)
}

async fn get_stored_credentials(
    username: &str,
    pool: &PgPool,
) -> Result<Option<(uuid::Uuid, String)>, anyhow::Error> {
    let row = sqlx::query_as!(
        User,
        r#"
        SELECT *
        FROM users
        WHERE username = $1
        "#,
        username,
    )
    .fetch_optional(pool)
    .await
    .context("Failed to performed a query to retrieve stored credentials.")?
    .map(|row| (row.user_id, row.password));
    Ok(row)
}

fn verify_password_hash(
    expected_password_hash: String,
    password_candidate: String,
) -> Result<(), AuthenticationError> {
    let expected_password_hash = PasswordHash::new(&expected_password_hash)
        .context("Failed to parse hash in PHC string format.")
        .map_err(AuthenticationError::UnexpectedError)?;

    Argon2::default()
        .verify_password(password_candidate.as_bytes(), &expected_password_hash)
        .context("Invalid password.")
        .map_err(|_| AuthenticationError::InvalidCredentials)
}

fn extract_from_headers(headers: &HeaderMap) -> Result<Credentials, anyhow::Error> {
    let header_value = headers
        .get("Authorization")
        .context("The 'Authorization' header was missing.")?
        .to_str()
        .context("The 'Authorization' header was not a valid UTF-8 string.")?;
    let base64_encoded_segment = header_value
        .strip_prefix("Basic ")
        .context("The authorization scheme was not 'Basic'.")?;
    let decoded_bytes = base64::decode_config(base64_encoded_segment, base64::STANDARD)
        .context("Failed to base64-decode 'Basic' credentials.")?;
    let decoded_credentials = String::from_utf8(decoded_bytes)
        .context("The decoded credential string is not valid UTF-8.")?;

    let mut credentials = decoded_credentials.splitn(2, ':');
    let username = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A username must be provided in 'Basic' auth."))?
        .to_string();
    let password = credentials
        .next()
        .ok_or_else(|| anyhow::anyhow!("A password must be provided in 'Basic' auth."))?
        .to_string();

    Ok(Credentials { username, password })
}

struct Credentials {
    username: String,
    password: String,
}

/// Error derived while handling an authentication request
#[derive(thiserror::Error)]
pub enum AuthenticationError {
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("Invalid headers.")]
    InvalidHeaders,
    #[error("Invalid credentials.")]
    InvalidCredentials,
}

impl std::fmt::Debug for AuthenticationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error::error_chain_fmt(self, f)
    }
}

impl From<AuthenticationError> for ApplicationError {
    fn from(e: AuthenticationError) -> Self {
        Self::AuthError(e)
    }
}
