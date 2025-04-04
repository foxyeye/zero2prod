//use secrecy::ExposeSecret;
//use env_logger::Env;
use zero2prod::startup::Application;
//use sqlx::PgPool;
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::get_subscriber;
use zero2prod::telemetry::init_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), String::from("info"), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
