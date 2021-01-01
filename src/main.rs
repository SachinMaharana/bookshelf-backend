use bookshelf_backend::run;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;

use bookshelf_backend::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
