use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Post {
    id: i32,
    title: String,
    text: String,
    created_at: String,
    updated_at: String,
}

#[derive(Serialize)]
struct PostList {
    total_count: i32,
    posts: Vec<Post>,
}

#[get("/posts")]
async fn list() -> HttpResponse {
    HttpResponse::Ok().json(PostList {
        total_count: 2,
        posts: vec![
            Post {
                id: 1,
                title: "Hello, world!".to_string(),
                text: "This is my first post.".to_string(),
                created_at: "2021-01-01T00:00:00".to_string(),
                updated_at: "2021-01-01T00:00:00".to_string(),
            },
            Post {
                id: 2,
                title: "Hello again!".to_string(),
                text: "This is my second post.".to_string(),
                created_at: "2021-01-02T00:00:00".to_string(),
                updated_at: "2021-01-02T00:00:00".to_string(),
            },
        ],
    })
}

#[post("/posts")]
async fn create() -> HttpResponse {
    HttpResponse::Created().json(Post {
        id: 1,
        title: "created post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: "2021-01-01T00:00:00".to_string(),
        updated_at: "2021-01-01T00:00:00".to_string(),
    })
}

#[get("/posts/{id}")]
async fn detail() -> HttpResponse {
    HttpResponse::Ok().json(Post {
        id: 1,
        title: "detail post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: "2021-01-01T00:00:00".to_string(),
        updated_at: "2021-01-01T00:00:00".to_string(),
    })
}

#[patch("/posts/{id}")]
async fn update() -> HttpResponse {
    HttpResponse::Ok().json(Post {
        id: 1,
        title: "updated post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: "2021-01-01T00:00:00".to_string(),
        updated_at: "2021-01-01T00:00:00".to_string(),
    })
}

#[delete("/posts/{id}")]
async fn delete() -> HttpResponse {
    HttpResponse::Ok().json(Post {
        id: 1,
        title: "deleted post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: "2021-01-01T00:00:00".to_string(),
        updated_at: "2021-01-01T00:00:00".to_string(),
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(detail);
    cfg.service(update);
    cfg.service(delete);
}
