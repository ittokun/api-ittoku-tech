use actix_web::{delete, get, patch, post, web, HttpResponse};

use crate::posts::Post;

#[get("/posts")]
async fn find_all() -> HttpResponse {
    HttpResponse::Ok().json(vec![
        Post {
            id: 1,
            title: "First Post".to_string(),
            body: "Hello World!".to_string(),
        },
        Post {
            id: 2,
            title: "Second Post".to_string(),
            body: "Good Morning!".to_string(),
        },
    ])
}

#[get("/posts/{id}")]
async fn find() -> HttpResponse {
    HttpResponse::Ok().json(Post {
        id: 1,
        title: "First Post".to_string(),
        body: "Hello World!".to_string(),
    })
}

#[post("/posts")]
async fn create(post: web::Json<Post>) -> HttpResponse {
    HttpResponse::Ok().json(post.into_inner())
}

#[patch("/posts/{id}")]
async fn update(post: web::Json<Post>) -> HttpResponse {
    HttpResponse::Ok().json(post.into_inner())
}

#[delete("/posts/{id}")]
async fn delete() -> HttpResponse {
    HttpResponse::Ok().json(Post {
        id: 3,
        title: "Third Post".to_string(),
        body: "Good Night!".to_string(),
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
