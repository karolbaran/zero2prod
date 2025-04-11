use std::net::TcpListener;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder, dev::Server, get, post, web,
};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("Received a subscription request: {:?}", form);
    dbg!(form);
    HttpResponse::Ok().finish()
}

pub fn run(listner: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(greet_world)
            .service(health_check)
            .service(subscribe)
    })
    .listen(listner)?
    .run();
    Ok(server)
}
