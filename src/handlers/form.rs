use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use tracing::field::Empty;
use uuid::Uuid;

use super::ApplicationResponse;
use crate::{
    clients::postmark::PostmarkClient, db::NewForm, db::NewResponse, handlers::ApplicationError,
    services::jwt::JwtClient,
};

#[derive(Debug, Deserialize)]
pub struct ViewFormQuery {
    pub id: Option<Uuid>,
}

#[tracing::instrument(name = "handlers::form::view", skip(pool, query, jwt), fields(username=Empty, user_id=Empty))]
/// get(form/list) runs an SQL query to retrieve all the forms belonging to the user who sent the request
pub async fn view_forms(
    pool: web::Data<PgPool>,
    request: HttpRequest,
    query: web::Query<ViewFormQuery>,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    match query.id {
        Some(form_id) => view_form_responses(pool, request, form_id, jwt).await,
        None => list_forms(pool, request, jwt).await,
    }
}

#[derive(Serialize)]
pub struct IndividualResponse {
    form_input_id: Uuid,
    content: String,
}

#[derive(Serialize)]
pub struct ResponseGroup {
    pub response_id: Uuid,
    pub replies: Vec<IndividualResponse>,
}

#[derive(Serialize)]
pub struct ViewFormResponse {
    pub questions: HashMap<Uuid, String>,
    pub responses: Vec<ResponseGroup>,
}

/// get(form/view) tunnels here if there is a query present in the URL
/// Runs SQL queries to return responses for the form id given in the query parameter
pub async fn view_form_responses(
    pool: web::Data<PgPool>,
    request: HttpRequest,
    form_id: Uuid,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let _current_user = jwt.user_or_403(request).await?;

    let fields = sqlx::query!(
        r#"SELECT * FROM form_input
           WHERE form_id = $1"#,
        form_id
    )
    .fetch_all(pool.as_ref())
    .await?;

    let mut questions: HashMap<Uuid, String> = HashMap::new();

    for field in fields.iter() {
        questions.insert(field.id, field.caption.as_ref().unwrap().clone());
    }

    let responses_data = sqlx::query!(
        r#"SELECT * FROM response 
        WHERE form_id = $1"#,
        form_id
    )
    .fetch_all(pool.as_ref())
    .await?;

    let mut responses: Vec<ResponseGroup> = vec![];

    for response in responses_data.iter() {
        let replies_data = sqlx::query!(
            r#"SELECT * FROM feedback
               WHERE response_id = $1"#,
            response.id
        )
        .fetch_all(pool.as_ref())
        .await?;

        let mut replies: Vec<IndividualResponse> = vec![];

        for reply in replies_data.iter() {
            replies.push(IndividualResponse {
                form_input_id: reply.form_input_id,
                content: reply.content.clone(),
            });
        }

        responses.push(ResponseGroup {
            response_id: response.id,
            replies,
        });
    }

    let view_form_responses_data = ViewFormResponse {
        questions,
        responses,
    };

    Ok(HttpResponse::Ok().body(serde_json::to_string(&view_form_responses_data).unwrap()))
}

#[derive(Serialize)]
pub struct ListedFormResponse {
    pub title: String,
    pub form_id: Uuid,
}

#[derive(Serialize)]
pub struct FormListResponse {
    pub forms: Vec<ListedFormResponse>,
}

/// get(form/view) tunnels here if no query is present in the URL
/// Runs an SQL query to retrieve all the forms belonging to the user who sent the request
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

    let form_list_response_data = FormListResponse {
        forms: forms
            .iter()
            .map(|f| ListedFormResponse {
                title: String::from(f.title.as_ref().unwrap()),
                form_id: f.id,
            })
            .collect(),
    };

    Ok(HttpResponse::Ok().body(serde_json::to_string(&form_list_response_data).unwrap()))
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
    pub field_id: Uuid,
    pub caption: String,
    pub r#type: String,
}

#[derive(Serialize)]
pub struct FormGetResponse {
    pub title: String,
    pub fields: Vec<FieldGetResponse>,
}

#[tracing::instrument(name = "handlers::form::get", skip(query, pool))]
/// get(form/submit) and get(form/edit) runs an SQL query on a provided form id and returns a JSON object of the fields
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
            fields: fields
                .iter()
                .map(|f| FieldGetResponse {
                    field_id: f.id,
                    caption: String::from(f.caption.as_ref().unwrap()),
                    r#type: String::from(&f.r#type),
                })
                .collect(),
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
pub struct FormQuery {
    pub id: Uuid,
}

#[tracing::instrument(name = "handlers::form::store_response", skip(query, json, pool), fields(username=Empty, user_id=Empty))]
/// post(form/submit) runs an SQL query to store a new response and all its associated fields
pub async fn store_form_response(
    json: web::Json<ResponseCreationRequest>,
    pool: web::Data<PgPool>,
    query: web::Query<FormQuery>,
) -> ApplicationResponse {
    // Store response first to avoid foreign key constrain
    let mut new_response = NewResponse::default();
    new_response.form_id = query.id;
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

#[derive(Debug, Deserialize)]
pub struct FieldEditRequest {
    field_id: Option<Uuid>,
    caption: String,
    r#type: String,
    delete: bool,
}

#[derive(Debug, Deserialize)]
pub struct FormEditRequest {
    pub title: String,
    pub fields: Vec<FieldEditRequest>,
}

#[tracing::instrument(name = "handlers::form::edit", skip(query, json, pool, request, jwt), fields(username=Empty, user_id=Empty))]
/// post(form/edit) runs an SQL query to edit a form
pub async fn edit_form(
    json: web::Json<FormEditRequest>,
    pool: web::Data<PgPool>,
    request: HttpRequest,
    query: web::Query<FormQuery>,
    jwt: web::Data<JwtClient>,
) -> ApplicationResponse {
    let _current_user = jwt.user_or_403(request).await?;

    sqlx::query!(
        r#"UPDATE form
           SET title = $1
           WHERE id = $2"#,
        json.title,
        query.id
    )
    .fetch_optional(pool.as_ref())
    .await?;

    let mut tx = pool.begin().await?;

    for edit in json.fields.iter() {
        match edit.field_id {
            Some(field_id) => {
                if edit.delete {
                    sqlx::query!(
                        r#"DELETE FROM feedback
                           WHERE form_input_id = $1"#,
                        field_id
                    )
                    .execute(&mut tx)
                    .await?;

                    sqlx::query!(
                        r#"DELETE FROM form_input
                           WHERE id = $1"#,
                        field_id
                    )
                    .execute(&mut tx)
                    .await?;
                } else {
                    sqlx::query!(
                        r#"UPDATE form_input
                           SET caption = $1, type = $2
                           WHERE id = $3"#,
                        edit.caption,
                        edit.r#type,
                        field_id
                    )
                    .execute(&mut tx)
                    .await?;
                }
            }
            None => {
                sqlx::query!(
                    r#"INSERT INTO form_input (id, form_id, type, caption)
                       VALUES($1, $2, $3, $4)"#,
                    Uuid::new_v4(),
                    query.id.clone(),
                    edit.r#type,
                    edit.caption
                )
                .execute(&mut tx)
                .await?;
            }
        };
    }

    tx.commit().await?;

    Ok(HttpResponse::Ok().body(format!("Successfully edited form with id {}", query.id)))
}

#[tracing::instrument(name = "handlers::form::test_email", skip(postmark_client))]
/// post(form/edit) runs an SQL query to edit a form
pub async fn test_email(postmark_client: web::Data<PostmarkClient>) -> ApplicationResponse {
    postmark_client
        .send_email("japence@crimson.ua.edu", "Hello, Postmark!")
        .await?;
    Ok(HttpResponse::Ok().body("E-mail request made to Postmark".to_string()))
}
