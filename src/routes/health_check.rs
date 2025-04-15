use actix_web::{HttpRequest, HttpResponse, Responder, get};

#[get("/health")]
pub async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("OK")
}
