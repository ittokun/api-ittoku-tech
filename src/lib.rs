use actix_web::{get, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

use std::env;

#[derive(Serialize, Deserialize)]
struct Urls {
    post_list_url: String,
    post_detail_url: String,
}

#[get("/")]
pub async fn index() -> HttpResponse {
    let base_url = env::var("BASE_URL").expect("Base URL not set");
    let urls = Urls {
        post_list_url: format!("{base_url}/posts"),
        post_detail_url: format!("{base_url}/posts/{{id}}"),
    };
    let urls = to_string_pretty(&urls).unwrap();
    HttpResponse::Ok().body(urls)
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::test::{init_service, TestRequest};
    use actix_web::App;

    #[actix_web::test]
    async fn test_index() {
        let app = init_service(App::new().service(index)).await;
        let resp = TestRequest::get().uri("/").send_request(&app).await;
        assert!(resp.status().is_success());
    }
}
