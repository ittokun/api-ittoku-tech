use actix_web::test::{init_service, read_body_json, TestRequest};
use actix_web::App;
use serde_json::Value;

use api_ittoku_tech::db::post;
use api_ittoku_tech::routes;

mod utils;

use utils::prelude::*;

#[actix_web::test]
async fn post_list() {
    let app = init_service(
        App::new()
            .app_data(app_state().await)
            .configure(routes::init),
    )
    .await;
    let resp = TestRequest::get().uri("/posts").send_request(&app).await;
    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn post_create() {
    let app = init_service(
        App::new()
            .app_data(app_state().await)
            .configure(routes::init),
    )
    .await;
    // Empty post data
    let post_data = NEW_POST_EMPTY.to_owned();
    let resp = TestRequest::post()
        .uri("/posts")
        .set_json(post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp: Value = read_body_json(resp).await;
    let resp_empty = RESP_POST_EMPTY.to_owned();
    assert_eq!(resp, resp_empty);
    // Too long post data
    let post_data = NEW_POST_TOO_LONG.to_owned();
    let resp = TestRequest::post()
        .uri("/posts")
        .set_json(post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp: Value = read_body_json(resp).await;
    let resp_too_long = RESP_POST_TOO_LONG.to_owned();
    assert_eq!(resp, resp_too_long);
    // success to create post
    let post_data = NEW_POST.to_owned();
    let resp = TestRequest::post()
        .uri("/posts")
        .set_json(&post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_success());
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp["title"], post_data["title"]);
    delete_post(resp["id"].as_i64().unwrap() as i32).await;
}

#[actix_web::test]
async fn post_detail() {
    let app = init_service(
        App::new()
            .app_data(app_state().await)
            .configure(routes::init),
    )
    .await;
    let post = create_post().await;
    // post not found
    let resp = TestRequest::get()
        .uri("/posts/hoge")
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp_not_found = RESP_NOTFOUND.to_owned();
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp, resp_not_found);
    // success to get post
    let resp = TestRequest::get()
        .uri(&format!("/posts/{}", post.id))
        .send_request(&app)
        .await;
    assert!(resp.status().is_success());
    let resp: post::Model = read_body_json(resp).await;
    assert_eq!(resp, post);
    delete_post(resp.id).await;
}

#[actix_web::test]
async fn post_update() {
    let app = init_service(
        App::new()
            .app_data(app_state().await)
            .configure(routes::init),
    )
    .await;
    let post = create_post().await;
    // Empty post data
    let post_data = NEW_POST_EMPTY.to_owned();
    let resp = TestRequest::patch()
        .uri(&format!("/posts/{}", post.id))
        .set_json(post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp: Value = read_body_json(resp).await;
    let resp_empty = RESP_POST_EMPTY.to_owned();
    assert_eq!(resp, resp_empty);
    // Too long post data
    let post_data = NEW_POST_TOO_LONG.to_owned();
    let resp = TestRequest::patch()
        .uri(&format!("/posts/{}", post.id))
        .set_json(post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp: Value = read_body_json(resp).await;
    let resp_too_long = RESP_POST_TOO_LONG.to_owned();
    assert_eq!(resp, resp_too_long);
    // post not found
    let post_data = NEW_POST_UPDATED.to_owned();
    let resp = TestRequest::patch()
        .uri("/posts/hoge")
        .set_json(&post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp_not_found = RESP_NOTFOUND.to_owned();
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp, resp_not_found);
    // success to update post
    let resp = TestRequest::patch()
        .uri(&format!("/posts/{}", post.id))
        .set_json(&post_data)
        .send_request(&app)
        .await;
    assert!(resp.status().is_success());
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp["title"], post_data["title"]);
    delete_post(resp["id"].as_i64().unwrap() as i32).await;
}

#[actix_web::test]
async fn post_delete() {
    let app = init_service(
        App::new()
            .app_data(app_state().await)
            .configure(routes::init),
    )
    .await;
    let post = create_post().await;
    // post not found
    let resp = TestRequest::delete()
        .uri("/posts/hoge")
        .send_request(&app)
        .await;
    assert!(resp.status().is_client_error());
    let resp_not_found = RESP_NOTFOUND.to_owned();
    let resp: Value = read_body_json(resp).await;
    assert_eq!(resp, resp_not_found);
    // success to delete post
    let resp = TestRequest::delete()
        .uri(&format!("/posts/{}", post.id))
        .send_request(&app)
        .await;
    assert!(resp.status().is_success());
    let resp: post::Model = read_body_json(resp).await;
    assert_eq!(resp, post);
}
