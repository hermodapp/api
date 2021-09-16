use actix_identity::Identity;
use actix_web::HttpResponse;

use crate::db::User;

use super::ApplicationResponse;

#[tracing::instrument(name = "handlers::hello", skip(id))]
/// Get(/) runs a sample SQL query and checks if the user is logged in
pub async fn hello(id: Identity) -> ApplicationResponse {
    if let Some(id) = id.identity() {
        let current_user: User = serde_json::from_str(&id).unwrap();
        Ok(HttpResponse::Ok().body(format!("you are {}", current_user.id)))
    } else {
        Ok(HttpResponse::Ok().body("Hello, World!".to_string()))
    }
}
