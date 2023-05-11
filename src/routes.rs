use serde::{self, Serialize};

use actix_web::{get, web, HttpResponse};
use serde_json::to_string_pretty;

use std::env;

// mod posts;

#[derive(Serialize)]
#[serde(crate = "self::serde")]
struct Urls {
    post_detail_url: String,
    post_list_url: String,
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

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(index);

    // cfg.configure(posts::init_routes);
}
