use hermod::telemetry::{get_subscriber, init_subscriber};
use hermod::{configuration::get_configuration, startup::Application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let subscriber = get_subscriber("hermod".into(), "info".into(), std::io::stdout);
    // init_subscriber(subscriber);

    // let configuration = get_configuration().expect("Failed to read configuration.");
    // let application = Application::build(configuration).await?;
    // application.run_until_stopped().await?;
    // loop {}
    println!("The app deploys");
    Ok(())
}
