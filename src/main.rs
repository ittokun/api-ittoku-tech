#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sea_orm::DatabaseConnection;

use std::env;

mod config;
mod db;
mod errors;
mod routes;

use db::{database_connection, migration};

#[derive(Debug, Clone)]
pub struct AppState {
    conn: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // get env variables
    let host = env::var("HOST").expect("HOST must be set");
    let port = env::var("PORT").expect("PORT must be set");
    let base_url = env::var("BASE_URL").expect("BASE_URL must be set");
    let server_url = format!("{}:{}", host, port);

    // setup database
    let conn = database_connection().await.unwrap();
    migration(&conn).await.expect("Failed to migrate");
    let state = AppState { conn };

    env_logger::init();

    let app = move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .wrap(Logger::default())
            .configure(config::init)
            .configure(routes::init)
    };

    info!("Starting server at: {}", base_url);
    HttpServer::new(app).bind(server_url)?.run().await
}
