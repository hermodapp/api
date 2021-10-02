use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::QrCode,
    handlers::{json_response, ApplicationError},
    jwt::JwtClient,
};
use serde::Deserialize;

use super::ApplicationResponse;

#[derive(Deserialize)]
pub struct GetQrCodeRequest {
    pub slug: String,
}

#[derive(serde::Serialize)]
pub struct GetQrCodeResponse {
    pub generation_data: String,
}

#[tracing::instrument(name = "qr_code::get", skip(pool, query))]
/// get(qr_code?slug={SLUG}) runs a sample SQL query and checks if the user is logged in
pub async fn get_qr_code_data(
    pool: web::Data<PgPool>,
    query: web::Query<GetQrCodeRequest>,
) -> ApplicationResponse {
    if let Some(qr_code) = sqlx::query!("SELECT * FROM qr_code WHERE slug=$1", &query.slug)
        .fetch_optional(pool.as_ref())
        .await?
    {
        json_response(&GetQrCodeResponse {
            generation_data: qr_code.generation_data,
        })
    } else {
        Err(ApplicationError::NotFoundError(format!(
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

#[tracing::instrument(name = "qr_code::store", skip(pool, query, jwt), fields(username=tracing::field::Empty, user_id=tracing::field::Empty))]
/// get(/qr_code/store?generation_data={DATA}&slug={SLUG}) stores a QR code with the relevant information
pub async fn store_qr_code(
    pool: web::Data<PgPool>,
    query: web::Query<NewQrCodeRequest>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = jwt.user_or_403(request).await?;
    tracing::Span::current().record("username", &tracing::field::display(&user.username));
    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));

    sqlx::query!(
        r#"
            INSERT INTO qr_code (id, account_id, slug, generation_data)
            VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        user.id,
        query.slug,
        query.generation_data
    )
    .execute(pool.as_ref())
    .await?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(serde::Serialize)]
pub struct ListQrCodesResponse {
    pub qr_codes: Vec<QrCode>,
}

#[tracing::instrument(name = "qr_code::list", skip(pool, request, jwt))]
/// get(/qr_codes) lists QR codes assosciated with a given user
pub async fn list_qr_codes(
    pool: web::Data<PgPool>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = jwt.user_or_403(request).await?;

    let qr_codes = sqlx::query_as!(
        QrCode,
        r#"
            SELECT * FROM qr_code
            WHERE account_id=$1"#,
        user.id,
    )
    .fetch_all(pool.as_ref())
    .await?;
    json_response(&ListQrCodesResponse { qr_codes })
}
