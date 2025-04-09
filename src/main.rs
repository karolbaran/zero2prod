use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, web};

#[get("/{name}")]
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

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(greet_world)
            .service(health_check)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
