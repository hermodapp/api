use actix_web::HttpResponse;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};

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
        "russweas",
        "russweas@gmail.com",
        "109234r1uijfasdcn",
        "Russell Weas"
    )
    .execute(&connection_pool)
    .await;
    println!("{:?}", insert);
    let users = sqlx::query!(
        r#"
        SELECT (username, email, password_hash, full_name)
        FROM users;
        "#,
    )
    .fetch_one(&connection_pool)
    .await;
    HttpResponse::Ok().body(&format!("{:?}", users))
}
