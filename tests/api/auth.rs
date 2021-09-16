use crate::helpers::{login, spawn_app};

#[actix_rt::test]
async fn auth_request_with_invalid_credentials_is_rejected() {
    let app = spawn_app().await;

    let response = login(&app, "".to_string(), "".to_string()).await;

    assert_eq!(401, response.status().as_u16());
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

    let response = login(&app, username, password).await;

    assert_eq!(200, response.status().as_u16());
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
