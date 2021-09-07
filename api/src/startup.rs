use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::configuration::{DatabaseSettings, Settings};
use crate::handlers::{health_check, hello, login, logout};

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
            .route("/health_check", web::get().to(health_check))
            .route("/", web::get().to(hello))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.with_db())
        .await
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
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

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
