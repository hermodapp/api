use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::field::Empty;
use uuid::Uuid;

use super::ApplicationResponse;
use crate::{db::NewForm, handlers::ApplicationError, jwt::JwtClient};

#[tracing::instrument(name = "handlers::form::list", skip(pool, jwt), fields(username=Empty, user_id=Empty))]
/// get(form/list) runs an SQL query to retrieve all the forms belonging to the user who sent the request
pub async fn list_forms(
    pool: web::Data<PgPool>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let current_user = jwt.user_or_403(request).await?;
    let forms = sqlx::query!(
        r#"
            SELECT * FROM form
            WHERE account_id=$1"#,
        current_user.id,
    )
    .fetch_all(pool.as_ref())
    .await?;
    Ok(HttpResponse::Ok().body(format!("{:?},", forms)))
}

/*
#[derive(serde::Serialize, serde::Deserialize)]
pub enum FieldType {
    Checkbox,
    Image,
    Radio,
    Text,
    Email,
    Phone,
    NumField,
    NumSlider,
    DateTime,
}
 */

#[derive(Deserialize)]
pub struct FormGetRequest {
    pub id: Uuid,
}

#[derive(Serialize)]
pub struct FieldGetResponse {
    pub caption: String,
    pub r#type: String,
}

#[derive(Serialize)]
pub struct FormGetResponse {
    pub title: String,
    pub fields: Vec<FieldGetResponse>,
}

#[tracing::instrument(name = "handlers::form::get", skip(query, pool))]
/// get(form) runs an SQL query on a provided form id and returns a JSON object of the fields
pub async fn get_form(
    query: web::Query<FormGetRequest>,
    pool: web::Data<PgPool>,
) -> ApplicationResponse {
    // Validate that such a requested form exists
    if let Some(form) = sqlx::query!("SELECT * FROM form WHERE id=$1", &query.id)
        .fetch_optional(pool.as_ref())
        .await?
    {
        // Retrieve fields associated with form
        let fields = sqlx::query!("SELECT * FROM form_input WHERE form_id=$1", form.id)
            .fetch_all(pool.as_ref())
            .await?;

        // Gather field types into a struct
        let form_response_data = FormGetResponse {
            title: form.title.unwrap(),
            fields: fields.iter().map(|f| FieldGetResponse{
                caption: String::from(f.caption.as_ref().unwrap()), 
                r#type: String::from(&f.r#type),
            }).collect(),
        };

        Ok(HttpResponse::Ok().body(serde_json::to_string(&form_response_data).unwrap()))
    } else {
        Err(ApplicationError::NotFoundError(format!(
            "No form found with id {}.",
            &query.id
        )))
    }
}

#[derive(Deserialize)]
pub struct FieldCreationRequest {
    pub caption: String,
    pub r#type: String,
}

#[derive(Deserialize)]
pub struct FormCreationRequest {
    pub title: String,
    pub fields: Vec<FieldCreationRequest>,
}

#[tracing::instrument(name = "handlers::form::store", skip(json, pool, request, jwt), fields(username=Empty, user_id=Empty))]
/// post(form/new) runs an SQL query to store a new form and all its associated fields
pub async fn store_form(
    json: web::Json<FormCreationRequest>,
    pool: web::Data<PgPool>,
    request: HttpRequest,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let current_user = jwt.user_or_403(request).await?;

    // Store form first to avoid foreign key constrain
    let mut new_form = NewForm::default();
    new_form.title = json.title.clone();
    new_form.account_id = current_user.id;
    new_form.store(pool.as_ref()).await?;

    // Create a transaction to store each form input
    let mut tx = pool.begin().await?;

    // Queue a SQL query for each form input
    for field in json.fields.iter() {
        sqlx::query!(
            r#"INSERT INTO form_input (id, form_id, type, caption) 
                VALUES ($1, $2, $3, $4)"#,
            Uuid::new_v4(),
            new_form.id,
            field.r#type.clone(),
            field.caption.clone()
        )
        .execute(&mut tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(HttpResponse::Ok().body(format!("Stored new form with id {}.", new_form.id)))
}
