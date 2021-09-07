use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::db::{NewUser, User};

use super::ApplicationResponse;
#[tracing::instrument(name = "handlers::hello", skip(pool, id))]
/// Get(/) runs a sample SQL query and checks if the user is logged in
pub async fn hello(pool: web::Data<PgPool>, id: Identity) -> ApplicationResponse {
    if let Some(id) = id.identity() {
        let current_user: User = serde_json::from_str(&id).unwrap();
        Ok(HttpResponse::Ok().body(format!("you are {}", current_user.username)))
    } else {
        let mut new_user = NewUser::default();
        new_user.username = "russweas3".to_string();
        new_user.password = "russweas".to_string();
        new_user.store(&pool).await;
        Ok(HttpResponse::Ok().body("New user stored.".to_string()))
    }
}
