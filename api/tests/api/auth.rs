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
    println!("{}", password);

    let response = login(&app, username, password).await;

    assert_eq!(200, response.status().as_u16());
}
