use actix_web::{get, web, HttpResponse};
use serde::{self, Serialize};
use serde_json::to_string_pretty;

use std::env;

use crate::errors::CustomError;

mod posts;

#[derive(Serialize)]
#[serde(crate = "self::serde")]
struct Urls {
    post_detail_url: String,
    post_list_url: String,
}

async fn not_found() -> Result<HttpResponse, CustomError> {
    Err(CustomError::NotFound)
}

#[get("/")]
async fn index() -> Result<HttpResponse, CustomError> {
    let base_url = env::var("BASE_URL").expect("Base URL not set");
    let urls = Urls {
        post_list_url: format!("{base_url}/posts"),
        post_detail_url: format!("{base_url}/posts/{{id}}"),
    };
    let urls = to_string_pretty(&urls).unwrap();

    Ok(HttpResponse::Ok().body(urls))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);
    cfg.configure(posts::init_routes);
    cfg.default_service(web::route().to(not_found));
}
