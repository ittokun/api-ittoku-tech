use crate::routes;
use actix_web::{get, post, web, Responder, HttpResponse};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
    cfg.service(echo);
    cfg.route("/hey", web::get().to(manual_hello));
    routes::post_routes(cfg);
}
