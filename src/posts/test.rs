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
        assert!(resp.status().is_success(), "Failed to GET /posts");
        // let resp: Vec<Post> = read_body_json(resp).await;
        // assert_eq!(resp, vec![post], "No output find all post");
    }

    #[actix_web::test]
    async fn find() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        let resp = TestRequest::get()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to GET /posts/:id");
        let resp: Post = read_body_json(resp).await;
        assert_eq!(post, resp, "No output find post");
        delete_post(resp);
    }

    #[actix_web::test]
    async fn create() {
        let app = init_service(App::new().configure(init_routes)).await;
        let request_body = REQUEST_BODY.to_owned();
        let resp = TestRequest::post()
            .uri("/posts")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to POST /posts");
        let resp: Post = read_body_json(resp).await;
        assert_eq!(resp.title, "Test Post", "No output create post");
        delete_post(resp);
    }

    #[actix_web::test]
    async fn update() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        let request_body = REQUEST_BODY.to_owned();
        let resp = TestRequest::patch()
            .uri(&format!("/posts/{}", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to PATCH /posts/:id");
        let resp: Post = read_body_json(resp).await;
        assert_ne!(post, resp, "No changed post");
        assert_ne!(post.updated_at, resp.updated_at, "No changed updated at");
        delete_post(resp);
    }

    #[actix_web::test]
    async fn delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        let resp = TestRequest::delete()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to DELETE /posts/:id");
        let resp: Post = read_body_json(resp).await;
        assert_eq!(post, resp, "No output delete post");
        let resp = TestRequest::get()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error(), "Failed delete post");
    }

    static REQUEST_BODY: Lazy<Value> = Lazy::new(|| {
        json!({
            "title": "Test Post",
            "body":  "This is a Test",
        })
    });

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
