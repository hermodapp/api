use actix_identity::Identity;
use actix_web::web;
use sqlx::PgPool;

use crate::db::User;

/// Get(/) runs a sample SQL query and checks if the user is logged in
pub async fn hello(pool: web::Data<PgPool>, id: Identity) -> String {
    if let Some(id) = id.identity() {
        let current_user: User = serde_json::from_str(&id).unwrap();
        current_user.username
    } else {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT *
            FROM users;
            "#,
        )
        .fetch_all(pool.as_ref())
        .await
        .expect("Failed to execute query on database");

        format!("{:?}", users)
    }
}
