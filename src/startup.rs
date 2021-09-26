//! Contains code neccessary to bootstrap the application and run the server.
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::configuration::{DatabaseSettings, Settings};
use crate::handlers::{
    get_qr_code_data, health_check, hello, login, logout, register, store_qr_code,
};

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

        // let sender_email = configuration
        //     .email_client
        //     .sender()
        //     .expect("Invalid sender email address.");
        // let timeout = configuration.email_client.timeout();
        // let email_client = EmailClient::new(
        //     configuration.email_client.base_url,
        //     sender_email,
        //     configuration.email_client.authorization_token,
        //     timeout,
        // );
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
            // email_client,
            // configuration.application.base_url,
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
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(30 * 3600))
        .connect_with(configuration.with_db())
        .await
}

fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false),
            ))
            .route("/login", web::get().to(login))
            .route("/logout", web::get().to(logout))
            .route("/register", web::post().to(register))
            .route("/health_check", web::get().to(health_check))
            .route("/qr_code", web::get().to(get_qr_code_data))
            .route("/qr_code/store", web::get().to(store_qr_code))
            .route("/", web::get().to(hello))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
