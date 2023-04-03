#[cfg(test)]
mod routes {
    use crate::comments::*;
    use crate::config;
    use crate::posts::{Post, PostParams};
    use actix_web::test::{init_service, read_body_json, TestRequest};
    use actix_web::App;
    use once_cell::sync::Lazy;
    use serde_json::{json, Value};

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
        let json_resp = RESP_NOT_FOUND.to_owned();
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
        let json_resp = RESP_NOT_FOUND.to_owned();
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
        let request_body = INVALID_JSON.to_owned();
        let resp = TestRequest::post()
            .uri(&format!("/posts/{}/comments", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_REQUIRED.to_owned();
        assert_eq!(resp, json_resp);
        // Too long POST data
        let request_body = INVALID_JSON_2.to_owned();
        let resp = TestRequest::post()
            .uri(&format!("/posts/{}/comments", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_TOO_LONG.to_owned();
        assert_eq!(resp, json_resp);
        // Success test
        let request_body = VALID_JSON.to_owned();
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
        let json_resp = RESP_NOT_FOUND.to_owned();
        assert_eq!(resp, json_resp);
        // Comment not found
        let comment = create_comment();
        let resp = TestRequest::delete()
            .uri(&format!("/posts/{}/comments/asdf", comment.post_id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let json_resp = RESP_NOT_FOUND.to_owned();
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

    static VALID_JSON: Lazy<Value> = Lazy::new(|| json!({ "body": "Test Comment", }));

    static INVALID_JSON: Lazy<Value> = Lazy::new(|| json!({ "body": "", }));

    static INVALID_JSON_2: Lazy<Value> = Lazy::new(|| json!({ "body": "a".repeat(65537) }));

    static RESP_NOT_FOUND: Lazy<Value> =
        Lazy::new(|| json!({"code": 404, "message": "Not Found".to_string()}));

    static RESP_REQUIRED: Lazy<Value> = Lazy::new(|| {
        json!({ "errors": [
            { "code": 422, "message": "body is required"},
        ]})
    });

    static RESP_TOO_LONG: Lazy<Value> = Lazy::new(|| {
        json!({ "errors": [
            { "code": 422, "message": "body is too long"},
        ]})
    });

    static RESP_INCORRECT: Lazy<Value> =
        Lazy::new(|| json!({ "code": 400, "message": "Post Request is Incorrect" }));

    fn create_post() -> Post {
        let post = PostParams {
            title: "create_post()".to_string(),
            body: "This is a create_post()".to_string(),
            updated_at: None,
        };
        Post::create(post).expect("Failed create_post()")
    }

    fn delete_post(post: Post) {
        Post::delete(post.id).expect("Failed delete_post(id)");
    }

    fn create_comment() -> Comment {
        let post = create_post();
        let comment = CommentParams {
            body: "This is a create_comment()".to_string(),
            updated_at: None,
        };
        Comment::create(post.id, comment).expect("Failed create_comment()")
    }

    // async fn delete_comment(comment: Comment) {
    //     Comment::delete(comment.id).expect("Failed delete_comment(comment)")
    // }
}
