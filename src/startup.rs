//! Contains code neccessary to bootstrap the application and run the server.
use actix_cors::Cors;

use crate::clients::postmark::PostmarkClient;
use crate::clients::twilio::TwilioClient;
use crate::handlers::{
    delete_qr_code, edit_form, edit_qr_code, forgot_password, generate_qr_code, get_form,
    health_check, list_qr_codes, login, logout, register, reset_password, scan, store_form,
    store_form_response, test_email, view_forms, who_am_i,
};
use crate::services::configuration::DatabaseSettings;
use crate::services::configuration::Settings;
use crate::services::jwt::JwtClient;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::{ConnectOptions, PgPool};
use std::net::TcpListener;
use tracing::log::LevelFilter;
use tracing_actix_web::TracingLogger;

/// Represents the server application.
pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    /// Given a configuration, build application dependencies and return a configured application.
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to Postgres.");

        let jwt_client = JwtClient::new(
            configuration.application.jwt_signing_key,
            connection_pool.clone(),
        );

        let twilio_client = TwilioClient::new(
            configuration.twilio.base_url,
            std::time::Duration::from_secs(5),
            configuration.twilio.account_sid,
            configuration.twilio.auth_token,
            configuration.twilio.from,
        );

        let postmark_client = PostmarkClient::new(
            configuration.postmark.base_url,
            std::time::Duration::from_secs(5),
            configuration.postmark.server_auth_token,
            configuration.postmark.from,
        );

        sqlx::migrate!("./migrations")
            .run(&connection_pool)
            .await
            .expect("Failed to migrate the database");

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            jwt_client,
            twilio_client,
            postmark_client,
        )?;

        Ok(Self { port, server })
    }

    /// The port that the server is listening on.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Run the HTTP server until interupted.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

/// Given a configuration, returns a pool of Postgres database connections.
pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    let db_connect_options = configuration
        .with_db()
        .log_statements(LevelFilter::Trace)
        .to_owned();

    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(db_connect_options)
        .await
}

fn run(
    listener: TcpListener,
    db_pool: PgPool,
    jwt_client: JwtClient,
    twilio_client: TwilioClient,
    postmark_client: PostmarkClient,
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let jwt_client = Data::new(jwt_client);
    let twilio_client = Data::new(twilio_client);
    let postmark_client = Data::new(postmark_client);

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(TracingLogger::default())
            .route("/login", web::get().to(login))
            .route("/scan", web::get().to(scan))
            .route("/logout", web::get().to(logout))
            .route("/whoami", web::get().to(who_am_i))
            .route("/register", web::post().to(register))
            .route("/password/forgot", web::post().to(forgot_password))
            .route("/password/reset", web::post().to(reset_password))
            .route("/health_check", web::get().to(health_check))
            .route("/qr_codes", web::get().to(list_qr_codes))
            .route("/qr_code/generate", web::post().to(generate_qr_code))
            .route("/qr_code/edit", web::get().to(edit_qr_code))
            .route("/qr_code/delete", web::get().to(delete_qr_code))
            .route("/form/new", web::post().to(store_form))
            .route("/form/submit", web::get().to(get_form))
            .route("/form/submit", web::post().to(store_form_response))
            .route("/form/view", web::get().to(view_forms))
            .route("/form/edit", web::get().to(get_form))
            .route("/form/edit", web::post().to(edit_form))
            .route("/form/test", web::get().to(test_email))
            .app_data(db_pool.clone())
            .app_data(jwt_client.clone())
            .app_data(twilio_client.clone())
            .app_data(postmark_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
