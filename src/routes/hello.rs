use actix_web::{HttpResponse, Responder, get, post, web};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}
