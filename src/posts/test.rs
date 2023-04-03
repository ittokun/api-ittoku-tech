#[cfg(test)]
mod routes {
    use crate::config;
    use crate::posts::*;
    use crate::test::*;
    use actix_web::test::{init_service, read_body_json, TestRequest};
    use actix_web::App;
    use serde_json::Value;

    #[actix_web::test]
    async fn find_all() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::get().uri("/posts").send_request(&app).await;
        assert!(resp.status().is_success());
        // let post = create_post();
        // let resp: PostFindAll = read_body_json(resp).await;
        // assert_eq!(resp.posts, vec![post]);
        // for post in resp.posts.iter() {
        //     delete_post(post);
        // }
    }

    #[actix_web::test]
    async fn find() {
        let app = init_service(App::new().configure(init_routes)).await;
        let post = create_post();
        // failre test
        let resp_not_found = RESP_NOTFOUND.to_owned();
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
        let app = init_service(App::new().configure(config::init).configure(init_routes)).await;
        // No send POST data
        let resp = TestRequest::post().uri("/posts").send_request(&app).await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let incorrect = RESP_INCORRECT.to_owned();
        assert_eq!(resp, incorrect);
        // Empty POST data
        let request_body = POST_INVALID_PARAMS.to_owned();
        let resp = TestRequest::post()
            .uri("/posts")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let required = POST_RESP_REQUIRED.to_owned();
        assert_eq!(resp, required);
        // Too long POST data
        let request_body = POST_INVALID_PARAMS_2.to_owned();
        let resp = TestRequest::post()
            .uri("/posts")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let too_long = POST_RESP_TOOLONG.to_owned();
        assert_eq!(resp, too_long);
        // success test
        let request_body = POST_VALID_PARAMS.to_owned();
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
        let app = init_service(App::new().configure(config::init).configure(init_routes)).await;
        let post = create_post();
        // Post not found
        let request_body = POST_VALID_PARAMS.to_owned();
        let resp = TestRequest::patch()
            .uri("/posts/asdf")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let resp_not_found = RESP_NOTFOUND.to_owned();
        assert_eq!(resp, resp_not_found);
        // No send POST data
        let resp = TestRequest::patch()
            .uri(&format!("/posts/{}", post.id))
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let incorrect = RESP_INCORRECT.to_owned();
        assert_eq!(resp, incorrect);
        // Empty POST data
        let request_body = POST_INVALID_PARAMS.to_owned();
        let resp = TestRequest::patch()
            .uri(&format!("/posts/{}", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let required = POST_RESP_REQUIRED.to_owned();
        assert_eq!(resp, required);
        // Too long POST data
        let request_body = POST_INVALID_PARAMS_2.to_owned();
        let resp = TestRequest::patch()
            .uri(&format!("/posts/{}", post.id))
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_client_error());
        let resp: Value = read_body_json(resp).await;
        let too_long = POST_RESP_TOOLONG.to_owned();
        assert_eq!(resp, too_long);
        // success test
        let request_body = POST_VALID_PARAMS.to_owned();
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
        let resp_not_found = RESP_NOTFOUND.to_owned();
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
}
