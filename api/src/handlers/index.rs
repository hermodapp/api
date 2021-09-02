use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub full_name: Option<String>,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn hello() -> HttpResponse {
    let settings = PgConnectOptions::new()
        .host("localhost")
        .username("postgres")
        .password("password")
        .port(5432)
        .database("hermod")
        .ssl_mode(sqlx::postgres::PgSslMode::Disable);
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(settings)
        .await
        .expect("Failed to connect to Posgres database");

    let insert = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash, full_name)
        VALUES ($1, $2, $3, $4);
        "#,
        "russweas22",
        "russweas2@gmail.com",
        "109234r1uijfasdcn",
        "Russell Weas"
    )
    .execute(&connection_pool)
    .await;
    println!("{:?}", insert);
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT *
        FROM users;
        "#,
    )
    .fetch_all(&connection_pool)
    .await
    .expect("Failed to execute query on database");
    println!("There are {} users in the database", users.len());

    for entry in users {
        println!("{:?}", entry);
    }
    HttpResponse::Ok().finish()
}
