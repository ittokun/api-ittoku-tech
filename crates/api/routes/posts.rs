use ::db::entity::post::Model as Post;
// use ::db::entity::post::NewModel as NewPost;
use ::db::entity::serde::{self, Serialize};
use ::db::chrono::Utc;

use actix_web::{delete, get, patch, post, web, HttpResponse};
use serde_json::to_string_pretty;

#[derive(Serialize)]
#[serde(crate = "self::serde")]
struct PostList {
    total_count: i32,
    posts: Vec<Post>,
}

#[get("/posts")]
async fn list() -> HttpResponse {
    let posts = PostList {
        total_count: 2,
        posts: vec![
            Post {
                id: 1,
                title: "Hello, world!".to_string(),
                text: "This is my first post.".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
            Post {
                id: 2,
                title: "Hello again!".to_string(),
                text: "This is my second post.".to_string(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            },
        ],
    };
    let posts = to_string_pretty(&posts).unwrap();

    HttpResponse::Ok().body(posts)
}

#[post("/posts")]
async fn create() -> HttpResponse {
    let post = Post {
        id: 1,
        title: "created post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let post = to_string_pretty(&post).unwrap();

    HttpResponse::Created().body(post)
}

#[get("/posts/{id}")]
async fn detail() -> HttpResponse {
    let post = Post {
        id: 1,
        title: "detail post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let post = to_string_pretty(&post).unwrap();

    HttpResponse::Ok().body(post)
}

#[patch("/posts/{id}")]
async fn update() -> HttpResponse {
    let post = Post {
        id: 1,
        title: "updated post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let post = to_string_pretty(&post).unwrap();

    HttpResponse::Ok().body(post)
}

#[delete("/posts/{id}")]
async fn delete() -> HttpResponse {
    let post = Post {
        id: 1,
        title: "deleted post".to_string(),
        text: "This is my first post.".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    let post = to_string_pretty(&post).unwrap();

    HttpResponse::Ok().body(post)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(list);
    cfg.service(create);
    cfg.service(detail);
    cfg.service(update);
    cfg.service(delete);
}
