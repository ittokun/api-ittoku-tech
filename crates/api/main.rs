#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

use std::env;

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    // get env variables
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let server_url = format!("{}:{}", host, port);

    let app = || App::new().wrap(Logger::default()).configure(routes::init);

    info!("Starting server at: {}", base_url);
    HttpServer::new(app).bind(server_url)?.run().await
}
