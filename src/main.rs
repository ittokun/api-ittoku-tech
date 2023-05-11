#[macro_use]
extern crate log;

// use db::{get_db_connection, migrate};
// use sea_orm::DatabaseConnection;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

use std::env;

mod routes;

#[derive(Debug, Clone)]
pub struct AppState {
    // conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get env variables
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let server_url = format!("{}:{}", host, port);

    env_logger::init();
    // migrate().await.expect("Failed to migrate");

    // let conn = get_db_connection().await.unwrap();
    // let state = AppState { conn };

    let app = move || {
        App::new()
            // .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .configure(routes::init)
    };

    info!("Starting server at: {}", base_url);
    HttpServer::new(app).bind(server_url)?.run().await
}
