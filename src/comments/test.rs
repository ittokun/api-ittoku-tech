#[cfg(test)]
mod routes {
    use crate::comments::*;
    use crate::posts::{Post, PostParams};
    use actix_web::test::{init_service, TestRequest};
    use actix_web::App;

    #[actix_web::test]
    async fn find_all() {
        let app = init_service(App::new().configure(init_routes)).await;
        // Post not found
        let resp = TestRequest::get()
            .uri("/posts/asdf/comments")
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        // success test
        let post = create_post();
        let resp = TestRequest::get()
            .uri(&format!("/posts/{}/comments", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
        delete_post(post);
    }

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
