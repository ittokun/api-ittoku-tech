use crate::api_error::ApiError;
use crate::posts::Post;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse};

use chrono::prelude::*;
use uuid::Uuid;

#[get("/posts")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let posts = Post::find_all()?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
async fn find(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let post = Post::find(id)?;
    Ok(HttpResponse::Ok().json(post))
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
        id: Uuid::new_v4(),
        title: "Third Post".to_string(),
        body: "Good Night!".to_string(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
