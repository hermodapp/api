use core::time;
use std::thread;

use uuid::Uuid;

use crate::helpers::{login, spawn_app};

#[actix_rt::test]
async fn auth_request_with_invalid_credentials_is_rejected() {
    let app = spawn_app().await;

    let response = login(&app, "".to_string(), "".to_string()).await;

    assert_eq!(401, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[actix_rt::test]
async fn auth_request_with_nonexisting_username_is_rejected() {
    let app = spawn_app().await;
    let username = "john".to_string();
    let password = "cs495".to_string();

    let response = login(&app, username, password).await;

    assert_eq!(401, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[actix_rt::test]
async fn auth_request_with_invalid_password_is_rejected() {
    let app = spawn_app().await;
    let username = app.test_user.username.to_string();
    let password = "cs495".to_string();

    let response = login(&app, username, password).await;

    assert_eq!(401, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[actix_rt::test]
async fn auth_request_with_valid_credentials_is_accepted() {
    let app = spawn_app().await;
    let username = app.test_user.username.to_string();
    let password = app.test_user.password.to_string();

    let response = login(&app, username.clone(), password).await;

    assert_eq!(200, response.status().as_u16());

    let token = response.text().await.unwrap();
    let claim = app.jwt_client.decode_token(token.as_str());
    assert_eq!(claim.unwrap().sub, app.test_user.id.to_string());
}

#[actix_rt::test]
async fn who_am_i_provides_correct_username_if_logged_in() {
    let app = spawn_app().await;
    let username = app.test_user.username.to_string();
    let password = app.test_user.password.to_string();

    let response = login(&app, username.clone(), password).await;
    assert_eq!(200, response.status().as_u16());
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/whoami", app.address))
        .header("Authorization", response.text().await.unwrap())
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.text().await.unwrap(), username);
}

#[actix_rt::test]
async fn who_am_i_responds_with_error_if_not_logged_in() {
    let app = spawn_app().await;
    let username = app.test_user.username.to_string();
    let password = app.test_user.password.to_string();

    let response = login(&app, username.clone(), password).await;
    assert_eq!(200, response.status().as_u16());
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/whoami", app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert_ne!(response.status(), 200);
    assert_ne!(response.text().await.unwrap(), username);
}

#[actix_rt::test]
async fn expired_jwt_tokens_are_rejected() {
    let app = spawn_app().await;
    let username = app.test_user.username.to_string();
    let client = reqwest::Client::new();
    let token = app
        .jwt_client
        .encode_token_with_exp(app.test_user.id, 1)
        .unwrap();
    thread::sleep(time::Duration::from_secs(2));
    let response = client
        .get(format!("{}/whoami", app.address))
        .header("Authorization", token)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_ne!(response.status(), 200);
    assert_ne!(response.text().await.unwrap(), username);
}

#[actix_rt::test]
async fn test_encode_and_decode_token() {
    let app = spawn_app().await;
    let user_id = Uuid::new_v4();
    let token = app.jwt_client.encode_token(user_id).unwrap();
    let result = app.jwt_client.decode_token(&token).unwrap();
    assert_eq!(result.sub, user_id.to_string());
}

#[actix_rt::test]
async fn registration_request_with_valid_credentials_is_accepted() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/register", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("username=russ&password=russ")
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[actix_rt::test]
async fn registration_request_with_malformed_input_is_rejected() {
    let app = spawn_app().await;

    let malformed_inputs = vec!["username=russ", "password=russ"];

    for body in malformed_inputs {
        let client = reqwest::Client::new();
        let response = client
            .post(format!("{}/register", app.address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(400, response.status().as_u16());
    }
}

#[actix_rt::test]
async fn registration_request_with_duplicate_username_is_rejected() {
    let app = spawn_app().await;
    let username = app.test_user.username.to_string();
    let body = format!("username={}&password=russ", username);

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/register", app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(500, response.status().as_u16());
}
