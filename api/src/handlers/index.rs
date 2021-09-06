use actix_identity::Identity;
use actix_web::web;
// use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;

use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password: String,
}

pub async fn hello(pool: web::Data<PgPool>, id: Identity) -> String {
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
    println!("\nThere are {} users in the database", users.len());
    let mut response = "".to_string();
    for entry in users {
        response.push_str(&format!("{:?}\n", entry));
    }
    if let Some(id) = id.identity() {
        format!("Welcome! {}", id)
    } else {
        response
    }

    // HttpResponse::Ok().body(response)
}
