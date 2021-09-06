use crate::helpers::spawn_app;

#[actix_rt::test]
async fn auth_request_with_invalid_credentials_is_rejected() {
    let app = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/auth", app.address))
        .json(&serde_json::json!({
            "title": "Newsletter Title",
            "content": {
                "text": "bla bla bla",
                "html": "<h1>Hello There</h1>"
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");
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

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/auth", app.address))
        .basic_auth(username, Some(password))
        .json(&serde_json::json!({
            "title": "Newsletter Title",
            "content": {
                "text": "bla bla bla",
                "html": "<h1>Hello There</h1>"
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(401, response.status().as_u16());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[actix_rt::test]
async fn auth_request_with_invalid_password_is_rejected() {
    let app = spawn_app().await;
    let username = &app.test_user.username;
    let password = "cs495".to_string();

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/auth", app.address))
        .basic_auth(username, Some(password))
        .json(&serde_json::json!({
            "title": "Newsletter Title",
            "content": {
                "text": "bla bla bla",
                "html": "<h1>Hello There</h1>"
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(401, response.status().as_u16());
    assert_eq!(
        r#"Basic realm="publish""#,
        response.headers()["WWW-Authenticate"]
    );
}

#[actix_rt::test]
async fn auth_request_with_valid_credentials_is_accepted() {
    let app = spawn_app().await;
    let username = &app.test_user.username;
    let password = &app.test_user.password;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/auth", app.address))
        .basic_auth(username, Some(password))
        .json(&serde_json::json!({
            "title": "Newsletter Title",
            "content": {
                "text": "bla bla bla",
                "html": "<h1>Hello There</h1>"
            }
        }))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(200, response.status().as_u16());
}
