use crate::api_error::ApiError;
use crate::posts::*;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use serde_json::to_string_pretty;
use uuid::Uuid;
use validator::Validate;

#[get("/posts")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let posts = Post::find_all()?;
    let posts = PostFindAll::new(posts.len(), posts);
    let posts = to_string_pretty(&posts).unwrap();
    Ok(HttpResponse::Ok().body(posts))
}

#[get("/posts/{id}")]
async fn find(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let post = Post::find(id)?;
    let post = to_string_pretty(&post).unwrap();
    Ok(HttpResponse::Ok().body(post))
}

#[post("/posts")]
async fn create(post: web::Json<PostParams>) -> Result<HttpResponse, ApiError> {
    post.validate()?;
    let post = Post::create(post.into_inner())?;
    let post = to_string_pretty(&post).unwrap();
    Ok(HttpResponse::Created().body(post))
}

#[patch("/posts/{id}")]
async fn update(req: HttpRequest, post: web::Json<PostParams>) -> Result<HttpResponse, ApiError> {
    post.validate()?;
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let post = Post::update(id, post.into_inner())?;
    let post = to_string_pretty(&post).unwrap();
    Ok(HttpResponse::Ok().body(post))
}

#[delete("/posts/{id}")]
async fn delete(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let post = Post::delete(id)?;
    let post = to_string_pretty(&post).unwrap();
    Ok(HttpResponse::Ok().body(post))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
