use std::str::FromStr;

use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

use super::ApplicationResponse;
use crate::{
    clients::{postmark::PostmarkClient, twilio::TwilioClient},
    db::QrCode,
    handlers::{json_response, ApplicationError},
    services::auth::AuthenticationError,
    services::jwt::JwtClient,
};
use serde::Deserialize;
use tracing::field::Empty;

#[derive(Deserialize, Clone)]
pub struct EditQrCodeRequest {
    pub id: Uuid,
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub payload: Option<String>,
    pub form_id: Option<Uuid>,
}

#[tracing::instrument(name = "handlers::qr_code::edit", skip(pool, json, jwt), fields(user_id=Empty))]
/// get(/qr_code/edit?id={ID}) edits a QR code with the relevant information
pub async fn edit_qr_code(
    pool: web::Data<PgPool>,
    json: web::Json<EditQrCodeRequest>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = jwt.user_or_403(request).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));

    let query = sqlx::query!(
        r#"
            UPDATE qr_code
            SET phone_number=$2, email=$3, payload=$4, form_id=$5
            WHERE id=$1 AND account_id=$6
            RETURNING true
        "#,
        json.id,
        json.phone_number,
        json.email,
        json.payload,
        json.form_id,
        user.id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    if query.is_some() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApplicationError::AuthError(
            AuthenticationError::Unauthorized,
        ))
    }
}

#[derive(Deserialize, Clone)]
pub struct DeleteQrCodeRequest {
    pub id: Uuid,
}

#[tracing::instrument(name = "handlers::qr_code::delete", skip(pool, query, jwt), fields(username=Empty, user_id=Empty))]
/// get(/qr_code/delete?id={ID}) edits a QR code with the relevant information
pub async fn delete_qr_code(
    pool: web::Data<PgPool>,
    query: web::Query<EditQrCodeRequest>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = jwt.user_or_403(request).await?;
    tracing::Span::current().record("username", &tracing::field::display(&user.username));
    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));

    let query = sqlx::query!(
        r#"
            DELETE FROM qr_code
            WHERE id=$1 AND account_id=$2
            RETURNING true
        "#,
        query.id,
        user.id
    )
    .fetch_optional(pool.as_ref())
    .await?;
    if query.is_some() {
        Ok(HttpResponse::Ok().finish())
    } else {
        Err(ApplicationError::AuthError(
            AuthenticationError::Unauthorized,
        ))
    }
}

#[derive(Deserialize, Clone)]
pub struct GenerateQrCodeRequest {
    pub phone_number: Option<String>,
    pub email: Option<String>,
    pub payload: Option<String>,
    pub form_id: Option<Uuid>,
}

#[derive(serde::Serialize)]
pub struct GenerateQrCodeResponse {
    pub id: Uuid,
}

#[tracing::instrument(name = "hadlers::qr_code::generate", skip(pool, json, jwt), fields(user_id=Empty))]
/// post(/qr_code/generate) generates a QR code with the relevant information
pub async fn generate_qr_code(
    pool: web::Data<PgPool>,
    json: web::Json<GenerateQrCodeRequest>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let user = jwt.user_or_403(request).await?;
    tracing::Span::current().record("user_id", &tracing::field::display(&user.id));

    let qr_code_id = Uuid::new_v4();

    sqlx::query!(
        r#"
            INSERT INTO qr_code (id, account_id, phone_number, email, payload, form_id)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
        qr_code_id,
        user.id,
        json.phone_number,
        json.email,
        json.payload,
        json.form_id,
    )
    .execute(pool.as_ref())
    .await?;

    let qr_code_generate_response = GenerateQrCodeResponse { id: qr_code_id };

    Ok(HttpResponse::Ok().body(serde_json::to_string(&qr_code_generate_response).unwrap()))
}

#[derive(serde::Serialize)]
pub struct ListQrCodesResponse {
    pub qr_codes: Vec<QrCode>,
}

#[tracing::instrument(name = "handlers::qr_code::list", skip(pool, request, jwt))]
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
#[derive(Deserialize, Clone)]
pub struct ScanQrCodeRequest {
    pub id: String,
}

pub async fn scan(
    pool: web::Data<PgPool>,
    query: web::Query<ScanQrCodeRequest>,
    twilio: web::Data<TwilioClient>,
    mail: web::Data<PostmarkClient>,
) -> ApplicationResponse {
    let id = query.id.as_ref();
    let id = Uuid::from_str(id).map_err(|e| anyhow::anyhow!(e))?;
    let qr_code = sqlx::query!("select * from qr_code where id=$1", id)
        .fetch_one(pool.as_ref())
        .await?;

    if qr_code.phone_number.is_some() || qr_code.email.is_some() {
        let message = qr_code.payload.ok_or_else(|| {
            ApplicationError::UnexpectedError(anyhow::anyhow!(
                "expect qr_code.payload when qr_code.phone_number or qr_code.email is defined"
            ))
        })?;

        // Check if there is an assosciated phone number with this QR code
        if let Some(phone_number) = qr_code.phone_number {
            twilio.send_call(phone_number, message.clone()).await?;
        }

        // Check if there is an assosciated email address with this QR code
        if let Some(email) = qr_code.email {
            mail.send_email(&email, message.as_str()).await?;
        }
    }

    // Check if there is an assosciated form with this QR code
    if let Some(form_id) = qr_code.form_id {
        let mut x = HttpResponse::TemporaryRedirect();
        x.append_header((
            "Location",
            format!("test.hermodapp.com/form/submit?id={}", form_id),
        ));
        return Ok(x.finish());
    }
    Ok(HttpResponse::Ok().body("Thank you for scanning a Hermod QR Code."))
}
