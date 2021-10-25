use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::field::Empty;
use uuid::Uuid;

use super::ApplicationResponse;
use crate::db::NewResponse;

#[derive(Deserialize)]
pub struct FeedbackCreationRequest {
    pub field_id: Uuid,
    pub content: String,
}

#[derive(Deserialize)]
pub struct ResponseCreationRequest {
    pub responses: Vec<FeedbackCreationRequest>,
}

#[derive(Deserialize)]
pub struct ResponseCreationQuery {
    pub id: Uuid,
}

#[tracing::instrument(name = "handlers::form::store_response", skip(query, json, pool), fields(username=Empty, user_id=Empty))]
/// post(form/submit) runs an SQL query to store a new response and all its associated fields
pub async fn store_form_response(
    json: web::Json<ResponseCreationRequest>,
    pool: web::Data<PgPool>,
    query: web::Query<ResponseCreationQuery>,
) -> ApplicationResponse {
    // Store response first to avoid foreign key constrain
    let mut new_response = NewResponse::default();
    new_response.form_id = query.id.clone();
    new_response.store(pool.as_ref()).await?;

    // Create a transaction to store each form input
    let mut tx = pool.begin().await?;

    // Queue a SQL query for each form input
    for response in json.responses.iter() {
        sqlx::query!(
            r#"INSERT INTO feedback (id, form_input_id, response_id, content) 
                VALUES ($1, $2, $3, $4)"#,
            Uuid::new_v4(),
            response.field_id.clone(),
            new_response.id.clone(),
            response.content.clone()
        )
        .execute(&mut tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(HttpResponse::Ok().body(format!("Stored new response with id {}.", new_response.id)))
}