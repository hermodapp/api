use hermod::{configuration::get_configuration, startup::Application};
use hermod::telemetry::{init_subscriber, get_subscriber};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("hermod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
