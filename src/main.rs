use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use log::debug;

mod app;
mod db;
mod domain;
mod error_handler;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    db::init();

    let app = || {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .configure(app::init)
    };

    debug!("Starting server: http://0.0.0.0:8080");
    HttpServer::new(app).bind(("0.0.0.0", 8080))?.run().await
}
