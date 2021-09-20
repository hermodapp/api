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
