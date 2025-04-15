use std::net::TcpListener;

use sqlx::{Connection, PgConnection, PgPool, postgres::PgPoolOptions};
use zero2prod::{configuration::get_configuration, startup::run};
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_settings())
        .await
        .expect("Failed to connect to Postgres");
    // let connection = PgConnection::connect(&configuration.database.connection_settings())
    //     .await
    //     .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
