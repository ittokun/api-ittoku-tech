#[cfg(test)]
mod routes {
    use crate::posts::*;
    use actix_web::test::{init_service, read_body_json, TestRequest};
    use actix_web::App;
    use once_cell::sync::Lazy;
    use serde_json::{json, Value};

    #[actix_web::test]
    async fn find_all() {
        let app = init_service(App::new().configure(init_routes)).await;
        // let post = create_post();
        let resp = TestRequest::get().uri("/posts").send_request(&app).await;
        assert!(resp.status().is_success());
        // let resp: PostFindAll = read_body_json(resp).await;
        // assert_eq!(resp.posts, vec![post], "No output find all post");
    }

    #[actix_web::test]
    async fn find() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        // failre test
        let resp_not_found = RESP_NOT_FOUND.to_owned();
        let resp = TestRequest::get()
            .uri("/posts/asdf")
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        assert_eq!(resp, resp_not_found);
        // success test
        let resp = TestRequest::get()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let resp: Post = read_body_json(resp).await;
        assert_eq!(post, resp);
        // initialize post
        delete_post(resp);
    }

    #[actix_web::test]
    async fn create() {
        let app = init_service(App::new().configure(init_routes)).await;
        // success test
        let request_body = VALID_JSON.to_owned();
        let resp = TestRequest::post()
            .uri("/posts")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let resp: Post = read_body_json(resp).await;
        assert_eq!(resp.title, "Test Post");
        // initialize post
        delete_post(resp);
    }

    #[actix_web::test]
    async fn update() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        // success test
        let request_body = VALID_JSON.to_owned();
        let resp = TestRequest::patch()
            .uri("/posts/asdf")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let resp_not_found = RESP_NOT_FOUND.to_owned();
        assert_eq!(resp, resp_not_found);
        // success test
        let resp = TestRequest::patch()
            .uri(&format!("/posts/{}", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let resp: Post = read_body_json(resp).await;
        assert_ne!(post, resp);
        assert_ne!(post.updated_at, resp.updated_at);
        // initialize post
        delete_post(resp);
    }

    #[actix_web::test]
    async fn delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        // failre test
        let resp_not_found = RESP_NOT_FOUND.to_owned();
        let resp = TestRequest::delete()
            .uri("/posts/asdf")
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        assert_eq!(resp, resp_not_found);
        // success test
        let resp = TestRequest::delete()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        let resp: Post = read_body_json(resp).await;
        assert_eq!(post, resp);
        let resp = TestRequest::get()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
    }

    static VALID_JSON: Lazy<Value> = Lazy::new(|| {
        json!({
            "title": "Test Post",
            "body":  "This is a Test",
        })
    });

    static RESP_NOT_FOUND: Lazy<Value> =
        Lazy::new(|| json!({"code": 404, "message": "Not Found".to_string()}));

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
}
