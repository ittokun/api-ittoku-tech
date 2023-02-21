#[macro_use]
extern crate log;

use std::{env, io};

use actix_web::{web, App, HttpServer};

mod api;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "ittoku_api=debug,actix_web=info");
    env_logger::init();

    let app = || {
        debug!("Constructing the App");

        App::new()
            .service(api::hello)
            .service(api::echo)
            .route("/hey", web::get().to(api::manual_hello))
    };

    debug!("Starting server: http://0.0.0.0:8080");
    HttpServer::new(app).bind(("0.0.0.0", 8080))?.run().await
}
