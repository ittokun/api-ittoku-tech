use actix_web::{get, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use std::env;

#[derive(Serialize, Deserialize)]
struct Urls {
    post_list_url: String,
    post_detail_url: String,
}

#[get("/")]
async fn index() -> HttpResponse {
    let base_url = env::var("BASE_URL").expect("Base URL not set");
    let urls = Urls {
        post_list_url: format!("{base_url}/posts"),
        post_detail_url: format!("{base_url}/posts/{{id}}"),
    };
    let urls = to_string_pretty(&urls).unwrap();
    HttpResponse::Ok().body(urls)
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");
    let app = || {
        App::new()
            .service(index)
    };

    HttpServer::new(app)
        .bind(format!("{host}:{port}"))?
        .run()
        .await
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {:?}", err);
    }
}
