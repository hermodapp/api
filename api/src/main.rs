use hermod::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", "localhost", 8000);
    let listener = TcpListener::bind(address.clone())?;

    run(listener)?.await?;
    Ok(())
}
