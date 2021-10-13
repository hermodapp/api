//! Contains code neccessary to bootstrap the application and run the server.
use actix_cors::Cors;

use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::{ConnectOptions, PgPool};
use std::net::TcpListener;
use tracing::log::LevelFilter;
use tracing_actix_web::TracingLogger;

use crate::clients::twilio::TwilioClient;
use crate::configuration::{DatabaseSettings, Settings};
use crate::handlers::{
    delete_qr_code, edit_qr_code, get_form, get_qr_code_data, health_check, list_qr_codes, login,
    logout, register, store_form, store_qr_code, who_am_i,
};
use crate::jwt::JwtClient;

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

        let server = run(listener, connection_pool, jwt_client, twilio_client)?;

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
) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let jwt_client = Data::new(jwt_client);
    let twilio_client = Data::new(twilio_client);

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(TracingLogger::default())
            .wrap(cors)
            .route("/login", web::get().to(login))
            .route("/logout", web::get().to(logout))
            .route("/whoami", web::get().to(who_am_i))
            .route("/register", web::post().to(register))
            .route("/health_check", web::get().to(health_check))
            .route("/qr_code", web::get().to(get_qr_code_data))
            .route("/qr_codes", web::get().to(list_qr_codes))
            .route("/qr_code/store", web::get().to(store_qr_code))
            .route("/qr_code/edit", web::get().to(edit_qr_code))
            .route("/qr_code/delete", web::get().to(delete_qr_code))
            .route("/form", web::get().to(get_form))
            .route("/form/store", web::post().to(store_form))
            .app_data(db_pool.clone())
            .app_data(jwt_client.clone())
            .app_data(twilio_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
