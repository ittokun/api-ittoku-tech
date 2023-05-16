use actix_web::test::{init_service, read_body_json, TestRequest};
use actix_web::App;
use serde_json::Value;

use api_ittoku_tech::{config, routes};

mod utils;

use utils::prelude::*;

#[actix_web::test]
async fn root() {
    let app = init_service(App::new().configure(routes::init)).await;
    let resp = TestRequest::get().uri("/").send_request(&app).await;
    assert!(resp.status().is_success());
    let resp_root = RESP_ROOT.to_owned();
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp, resp_root);
}

#[actix_web::test]
async fn not_found() {
    let app = init_service(App::new().configure(routes::init)).await;
    let resp = TestRequest::get().uri("/hoge/bar").send_request(&app).await;
    assert!(resp.status().is_client_error());
    let resp_not_found = RESP_NOTFOUND.to_owned();
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp, resp_not_found);
}

#[actix_web::test]
async fn invalid_json() {
    let app = init_service(
        App::new()
            .app_data(app_state().await)
            .configure(config::init)
            .configure(routes::init),
    )
    .await;
    let resp = TestRequest::post().uri("/posts").send_request(&app).await;
    assert!(resp.status().is_client_error());
    let resp_invalid_json = RESP_INVALID_JSON.to_owned();
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp, resp_invalid_json);
}
