use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{auth::AuthenticationError, db::User, handlers::ApplicationError};
use serde::Deserialize;

use super::ApplicationResponse;

#[derive(Deserialize)]
pub struct QrCodeRequest {
    pub slug: String,
}

#[tracing::instrument(name = "qr_code::get", skip(pool, query))]
/// get(qr_code?slug={SLUG}) runs a sample SQL query and checks if the user is logged in
pub async fn get_qr_code_data(
    pool: web::Data<PgPool>,
    query: web::Query<QrCodeRequest>,
) -> ApplicationResponse {
    if let Some(qr_code) = sqlx::query!("SELECT * FROM qr_code WHERE slug=$1", &query.slug)
        .fetch_optional(pool.as_ref())
        .await
        .map_err(|e| ApplicationError::UnexpectedError(anyhow::anyhow!(e)))?
    {
        Ok(HttpResponse::Ok().body(qr_code.generation_data))
    } else {
        Err(ApplicationError::UnexpectedError(anyhow::anyhow!(
            "No QR code found with slug {}",
            &query.slug
        )))
    }
}

#[derive(Deserialize, Clone)]
pub struct NewQrCodeRequest {
    pub generation_data: String,
    pub slug: String,
}

#[tracing::instrument(name = "qr_code::store", skip(pool, id, query))]
/// get(store_qr_code?generation_data={DATA}&slug={SLUG}) stores a QR code with the relevant information
pub async fn store_qr_code(
    pool: web::Data<PgPool>,
    id: Identity,
    query: web::Query<NewQrCodeRequest>,
) -> ApplicationResponse {
    if let Some(id) = id.identity() {
        let current_user: User = serde_json::from_str(&id).unwrap();

        sqlx::query!(
            r#"
            INSERT INTO qr_code (id, account_id, slug, generation_data)
            VALUES ($1, $2, $3, $4)"#,
            Uuid::new_v4(),
            current_user.id,
            query.slug,
            query.generation_data
        )
        .execute(pool.as_ref())
        .await
        .map_err(|e| ApplicationError::UnexpectedError(anyhow::anyhow!(e)))?;
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApplicationError::AuthError(
            AuthenticationError::Unauthorized,
        ))
    }
}
