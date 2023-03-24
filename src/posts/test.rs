#[cfg(test)]
mod routes {
    use crate::posts::*;
    use actix_web::test::{init_service, TestRequest};
    use actix_web::App;
    use chrono::prelude::*;
    use serde_json::json;
    use uuid::Uuid;

    #[actix_web::test]
    async fn find_all() {
        let app = init_service(App::new().configure(init_routes)).await;

        let resp = TestRequest::get().uri("/posts").send_request(&app).await;
        assert!(resp.status().is_success(), "Failed to GET /posts");
    }

    #[actix_web::test]
    async fn find() {
        let app = init_service(App::new().configure(init_routes)).await;

        let resp = TestRequest::get().uri("/posts/1").send_request(&app).await;
        assert!(resp.status().is_client_error(), "Failed to GET /posts/1");
    }

    #[actix_web::test]
    async fn create() {
        let request_body = json!(Post {
            id: Uuid::new_v4(),
            title: "Test Post".to_string(),
            body: "This is a Test".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        });
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::post()
            .uri("/posts")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to POST /posts");
    }

    #[actix_web::test]
    async fn update() {
        let request_body = json!(Post {
            id: Uuid::new_v4(),
            title: "Test Post".to_string(),
            body: "This is a Test".to_string(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        });
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::patch()
            .uri("/posts/1")
            .set_json(&request_body)
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to PATCH /posts/1");
    }

    #[actix_web::test]
    async fn delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::delete()
            .uri("/posts/1")
            .send_request(&app)
            .await;
        assert!(resp.status().is_success(), "Failed to DELETE /posts/1");
    }
}
