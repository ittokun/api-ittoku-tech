// #[macro_use]
// extern crate log;

// use actix_web::middleware::Logger;
// use actix_web::{App, HttpServer};
// use std::env;

// mod api_error;
// mod comments;
// mod config;
// mod db;
// mod posts;
// mod schema;
// #[cfg(test)]
// mod test;

fn main() {
    api::main();
    // env_logger::init();
    // db::init();

    // let host = env::var("HOST").expect("Host not set");
    // let port = env::var("PORT").expect("Port not set");
    // let app = || {
    //     App::new()
    //         .wrap(Logger::default())
    //         .service(ittoku_api::index)
    //         .configure(config::init)
    //         .configure(posts::init_routes)
    //         .configure(comments::init_routes)
    // };

    // info!("Starting Server: http://{host}:{port}");
    // HttpServer::new(app)
    //     .bind(format!("{host}:{port}"))?
    //     .run()
    //     .await
}
