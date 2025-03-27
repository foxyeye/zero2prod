//use secrecy::ExposeSecret;
//use env_logger::Env;
use sqlx::postgres::PgPoolOptions;
//use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::telemetry::get_subscriber;
use zero2prod::telemetry::init_subscriber;
use zero2prod::{configuration::get_configuration, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let subscriber = get_subscriber("zero2prod".into(), String::from("info"), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        //.connect_lazy(&configuration.database.connection_string().expose_secret())
        .connect_lazy_with(configuration.database.with_db());
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
