use std::net::TcpListener;

use actix_web::web::Data;
use actix_web::{App, HttpServer, dev::Server};
use sqlx::{PgConnection, PgPool};

use crate::routes::health_check;
use crate::routes::subscribe;

pub fn run(listner: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(health_check)
            .service(subscribe)
            .app_data(connection.clone())
    })
    .listen(listner)?
    .run();
    Ok(server)
}
