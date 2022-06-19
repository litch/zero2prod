use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;

use zero2prod::startup::{build};
use zero2prod::configuration::get_configuration;
use zero2prod::telemetry::{get_subscriber, init_subscriber};
use zero2prod::email_client::EmailClient;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(get_subscriber("zero2prod".into(), "info".into(), std::io::stdout));

    let configuration = get_configuration().expect("Failed to read configuration");

    let server = build(configuration).await?;
    server.await?;
    
    Ok(())
}