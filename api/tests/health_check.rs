use std::net::TcpListener;

use hermod::startup::run;

#[actix_rt::test]
async fn health_check() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap().to_string());

    let server = run(listener).expect("Could not run server");
    let _ = tokio::spawn(server);

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
