use hermod_api::services::jwt;

use crate::helpers::{login, spawn_app};

async fn seed_data(jwt_token: String, address: String) {
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/form/view", address))
        .header("Authorization", jwt_token)
        .send()
        .await
        .unwrap();
}

#[actix_rt::test]
async fn list_forms_returns_forms_for_logged_in_user() {
    let app = spawn_app().await;
    let response = login(
        &app,
        app.test_user.username.to_string(),
        app.test_user.password.to_string(),
    )
    .await;
    assert_eq!(200, response.status().as_u16());

    let jwt_token = response.text().await.unwrap();
    seed_data(jwt_token.clone(), app.address.clone()).await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/form/view", app.address))
        .header("Authorization", jwt_token)
        .send()
        .await
        .unwrap();

    dbg!(response);
}
