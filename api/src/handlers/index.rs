use actix_identity::Identity;
use actix_web::web;
use sqlx::PgPool;

use crate::db::User;

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
        let current_user: User = serde_json::from_str(&id).unwrap();
        current_user.username
    } else {
        response
    }
}
