//use secrecy::ExposeSecret;
//use env_logger::Env;
use zero2prod::startup::Application;
//use sqlx::PgPool;
use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use zero2prod::configuration::get_configuration;
use zero2prod::issue_delivery_worker::run_worker_until_stopped;
use zero2prod::telemetry::get_subscriber;
use zero2prod::telemetry::init_subscriber;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), String::from("info"), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    // let application = Application::build(configuration).await?;
    // application.run_until_stopped().await?;
    // Ok(())

    // let application = Application::build(configuration.clone())
    //     .await?
    //     .run_until_stopped();
    // let worker = run_worker_until_stopped(configuration);
    // tokio::select! {
    // _ = application => {},
    // _ = worker => {},
    // };
    let application = Application::build(configuration.clone()).await?;
    let application_task = tokio::spawn(application.run_until_stopped());
    let worker_task = tokio::spawn(run_worker_until_stopped(configuration));
    // tokio::select! {
    // _ = application_task => {},
    // _ = worker_task => {},
    // }
    tokio::select! {
o = application_task => report_exit("API", o),
o = worker_task => report_exit("Background worker", o),
}
    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            "{} failed",
            task_name
            )
        }
        Err(e) => {
            tracing::error!(
            error.cause_chain = ?e,
            error.message = %e,
            "{}' task failed to complete",
            task_name
            )
        }

    }
}
