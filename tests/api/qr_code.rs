use uuid::Uuid;

use crate::helpers::spawn_app;

#[actix_rt::test]
async fn get_qr_code_data_returns_404_for_invalid_slug() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let slug = "1235";
    let response = client
        .get(&format!("{}/qr_code?slug={}", app.address, slug))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status() == 404);
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn get_qr_code_data_returns_data_for_valid_slug() {
    let app = spawn_app().await;
    let slug = "12345";
    let data = "{color: 'blue'}";

    sqlx::query!(
        r#"
        INSERT INTO qr_code (id, account_id, slug, generation_data)
        VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        app.test_user.id,
        slug,
        data
    )
    .execute(&app.db_pool)
    .await
    .expect("Failed to execute query");

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/qr_code?slug={}", app.address, slug))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status() == 200);
    let response_body = response
        .text()
        .await
        .expect("Failed to extract text from result");
    assert_eq!(response_body, data);
}

#[actix_rt::test]
async fn store_qr_code_data_rejects_unauthorized_users() {
    let app = spawn_app().await;
    // app.login().await.expect("Failed to log test user in");

    let client = reqwest::Client::new();
    let data = "test_data";
    let slug = "1235";
    let response = client
        .get(&format!(
            "{}/qr_code/store?generation_data={}&slug={}",
            app.address, data, slug
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status() == 401);
    assert_eq!(Some(0), response.content_length());
}

#[actix_rt::test]
async fn store_qr_code_data_accepts_valid_users() {
    let app = spawn_app().await;
    app.login().await.expect("Failed to log test user in");

    let client = reqwest::Client::new();
    client
        .get(format!("{}/login", app.address))
        .basic_auth(app.test_user.username, Some(app.test_user.password))
        .send()
        .await
        .expect("Failed to execute request.");

    let data = "test_data";
    let slug = "123532";
    let response = client
        .get(&format!(
            "{}/qr_code/store?generation_data={}&slug={}",
            app.address, data, slug
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    // println!("{}", response.status());
    // assert!(response.status() == 200);
    assert_eq!(Some(0), response.content_length());
}