use hermod_api::{
    configuration::{get_configuration, DatabaseSettings},
    db::NewUser,
    jwt::JwtClient,
    startup::{get_connection_pool, Application},
    telemetry::{get_subscriber_test, init_subscriber},
};
use once_cell::sync::Lazy;
use reqwest::{Method, Response};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            get_subscriber_test(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber_test(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    // Launch a mock server to stand in for Postmark's API
    // let email_server = MockServer::start().await;

    // Randomise configuration to ensure test isolation
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use a different database for each test case
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application.port = 0;
        // Use the mock server as email API
        // c.email_client.base_url = email_server.uri();
        c
    };

    // Create and migrate the database
    configure_database(&configuration.database).await;

    // Launch the application as a background task
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build application.");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let pool = get_connection_pool(&configuration.database)
        .await
        .expect("Failed to connect to the database");

    let jwt_client = JwtClient::new(configuration.application.jwt_signing_key, pool.clone());

    let test_app = TestApp {
        address: format!("http://localhost:{}", application_port),
        port: application_port,
        db_pool: pool,
        jwt_client,
        test_user: NewUser::default(),
        jwt_token: "".to_string(),
    };

    test_app.test_user.store(&test_app.db_pool).await.unwrap();

    test_app
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection_pool
}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
    pub test_user: NewUser,
    pub jwt_client: JwtClient,
    jwt_token: String,
}

impl TestApp {
    pub async fn login(&mut self) -> anyhow::Result<()> {
        let response = login(
            self,
            self.test_user.username.to_string(),
            self.test_user.password.to_string(),
        )
        .await;
        self.jwt_token = response.text().await?;
        Ok(())
    }

    pub async fn send_request_with_auth(
        &self,
        method: Method,
        url: String,
    ) -> Result<Response, anyhow::Error> {
        let client = reqwest::Client::new();
        Ok(client
            .request(method, url)
            .header("Authorization", self.jwt_token.clone())
            .send()
            .await?)
    }
}

pub async fn login(app: &TestApp, username: String, password: String) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/login", app.address))
        .basic_auth(username, Some(password))
        .send()
        .await
        .expect("Failed to execute request.")
}
