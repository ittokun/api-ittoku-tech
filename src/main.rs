use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::debug;

mod api;
mod error_handler;
mod db;

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
            .service(api::hello)
            .service(api::echo)
            .route("/hey", web::get().to(api::manual_hello))
            // .service(api::posts::list)
            // .service(api::posts::create)
            // .service(api::posts::detail)
            // .service(api::posts::update)
            // .service(api::posts::delete)
    };

    debug!("Starting server: http://0.0.0.0:8080");
    HttpServer::new(app).bind(("0.0.0.0", 8080))?.run().await
}
