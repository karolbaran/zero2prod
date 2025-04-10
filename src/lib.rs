use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, get, web};

#[get("/say/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/")]
async fn greet_world() -> impl Responder {
    "Hello Aneta!"
}

#[get("/health")]
async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("OK")
}

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(greet_world)
            .service(health_check)
    })
    .listen(listner)?
    .run();
    Ok(server)
}
