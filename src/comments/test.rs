#[cfg(test)]
mod routes {
    use crate::test::*;
    use crate::comments::*;
    use crate::config;
    use actix_web::test::{init_service, read_body_json, TestRequest};
    use actix_web::App;
    use serde_json::Value;

    #[actix_web::test]
    async fn find_all() {
        let app = init_service(App::new().configure(init_routes)).await;
        // Post not found
        let resp = TestRequest::get()
            .uri("/posts/asdf/comments")
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_NOTFOUND.to_owned();
        assert_eq!(resp, json_resp);
        // success test
        let post = create_post();
        let resp = TestRequest::get()
            .uri(&format!("/posts/{}/comments", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        delete_post(post);
    }

    #[actix_web::test]
    async fn create() {
        let app = init_service(App::new().configure(config::init).configure(init_routes)).await;
        // Post not found
        let resp = TestRequest::get()
            .uri("/posts/asdf/comments")
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_NOTFOUND.to_owned();
        assert_eq!(resp, json_resp);
        // No send post data
        let post = create_post();
        let resp = TestRequest::post()
            .uri(&format!("/posts/{}/comments", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_INCORRECT.to_owned();
        assert_eq!(resp, json_resp);
        // Empty POST data
        let request_body = COMMENT_INVALID_PARAMS.to_owned();
        let resp = TestRequest::post()
            .uri(&format!("/posts/{}/comments", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = COMMENT_RESP_REQUIRED.to_owned();
        assert_eq!(resp, json_resp);
        // Too long POST data
        let request_body = COMMENT_INVALID_PARAMS_2.to_owned();
        let resp = TestRequest::post()
            .uri(&format!("/posts/{}/comments", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = COMMENT_RESP_TOOLONG.to_owned();
        assert_eq!(resp, json_resp);
        // Success test
        let request_body = COMMENT_VALID_PARAMS.to_owned();
        let resp = TestRequest::post()
            .uri(&format!("/posts/{}/comments", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let resp: Comment = read_body_json(resp).await;
        assert_eq!(resp.body, "Test Comment");
        // initialize post
        delete_post(post)
    }

    #[actix_web::test]
    async fn delete() {
        let app = init_service(App::new().configure(config::init).configure(init_routes)).await;
        // Post not found
        let resp = TestRequest::delete()
            .uri("/posts/asdf/comments/asdf")
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_NOTFOUND.to_owned();
        assert_eq!(resp, json_resp);
        // Comment not found
        let post = create_post();
        let comment = create_comment(post);
        let resp = TestRequest::delete()
            .uri(&format!("/posts/{}/comments/asdf", comment.post_id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_NOTFOUND.to_owned();
        assert_eq!(resp, json_resp);
        // Success test
        let resp = TestRequest::delete()
            .uri(&format!(
                "/posts/{}/comments/{}",
                comment.post_id, comment.id
            ))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let resp: Comment = read_body_json(resp).await;
        assert_eq!(resp, comment);
    }
}
