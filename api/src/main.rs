use hermod::startup::run;
use std::net::TcpListener;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", "0.0.0.0", 8000);
    let listener = TcpListener::bind(address.clone())?;
    println!("Starting server");
    run(listener)?.await?;
    Ok(())
}
