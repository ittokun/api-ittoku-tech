#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::env;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    info!("Starting Server: http://{host}:{port}");
    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind(format!("{host}:{port}"))?
        .run()
        .await
}
