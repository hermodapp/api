use actix_identity::Identity;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use super::ApplicationResponse;
use crate::{
    auth::AuthenticationError, db::NewForm, db::User, handlers::ApplicationError, jwt::user_or_403,
};

#[tracing::instrument(name = "form::list", skip(pool))]
/// get(form/list) runs an SQL query to retrieve all the forms belonging to the user who sent the request
pub async fn list_forms(pool: web::Data<PgPool>, request: HttpRequest) -> ApplicationResponse {
    let current_user = user_or_403(request, &pool).await?;
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
    pub form_id: Uuid,
}

#[derive(Serialize)]
pub struct FormGetResponse {
    pub fields: Vec<String>,
}

#[tracing::instrument(name = "form::get", skip(query, pool))]
/// get(form) runs an SQL query on a provided form id and returns a JSON object of the fields
pub async fn get_form(
    query: web::Query<FormGetRequest>,
    pool: web::Data<PgPool>,
) -> ApplicationResponse {
    // Validate that such a requested form exists
    if let Some(form) = sqlx::query!("SELECT * FROM form WHERE id=$1", &query.form_id)
        .fetch_optional(pool.as_ref())
        .await?
    {
        // Retrieve fields associated with form
        let fields = sqlx::query!("SELECT * FROM form_input WHERE form_id=$1", form.id)
            .fetch_all(pool.as_ref())
            .await?;

        // Gather field types into a struct
        let form_response_data = FormGetResponse {
            fields: fields.iter().map(|f| String::from(&f.r#type)).collect(),
        };

        Ok(HttpResponse::Ok().body(serde_json::to_string(&form_response_data).unwrap()))
    } else {
        Err(ApplicationError::NotFoundError(format!(
            "No form found with id {}.",
            &query.form_id
        )))
    }
}

#[derive(Deserialize)]
pub struct FormCreationRequest {
    pub qr_code_id: Uuid,
    pub fields: Vec<String>,
}

#[tracing::instrument(name = "form::store", skip(json, pool, request))]
/// post(form/store) runs an SQL query to store a new form and all its associated fields
pub async fn store_form(
    json: web::Json<FormCreationRequest>,
    pool: web::Data<PgPool>,
    request: HttpRequest,
) -> ApplicationResponse {
    let current_user = user_or_403(request, &pool).await?;

    // Store form first to avoid foreign key constrain
    let mut new_form = NewForm::default();
    new_form.qr_code_id = json.qr_code_id;
    new_form.account_id = current_user.id;
    new_form.store(pool.as_ref()).await?;

    // Create a transaction to store each form input
    let mut tx = pool.begin().await?;

    // Queue a SQL query for each form input
    for field in json.fields.iter() {
        sqlx::query!(
            r#"INSERT INTO form_input (id, form_id, type) 
                VALUES ($1, $2, $3)"#,
            Uuid::new_v4(),
            new_form.id,
            field
        )
        .execute(&mut tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(HttpResponse::Ok().body(format!("Stored new form with id {}.", new_form.id)))
}
