#[cfg(test)]
mod routes {
    use crate::users::*;
    use actix_web::test::{init_service, TestRequest};
    use actix_web::App;

    #[actix_web::test]
    async fn find_all() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::get().uri("/users").send_request(&app).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn find() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::get()
            .uri(&format!("/users/{}", uuid::Uuid::new_v4()))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn create() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::post().uri("/users").send_request(&app).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn update() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::patch()
            .uri(&format!("/users/{}", uuid::Uuid::new_v4()))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn delete() {
        let app = init_service(App::new().configure(init_routes)).await;
        let resp = TestRequest::delete()
            .uri(&format!("/users/{}", uuid::Uuid::new_v4()))
            .send_request(&app)
            .await;
        assert!(resp.status().is_success());
    }
}
