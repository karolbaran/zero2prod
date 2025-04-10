use std::net::TcpListener;

use zero2prod::run;
#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8081").expect("Failed to bind");
    run(listener)?.await
}
