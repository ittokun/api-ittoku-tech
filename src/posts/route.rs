use crate::api_error::ApiError;
use crate::posts::{Post, PostParams};
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
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
async fn create(post: web::Json<PostParams>) -> Result<HttpResponse, ApiError> {
    let post = Post::create(post.into_inner())?;
    Ok(HttpResponse::Created().json(post))
}

#[patch("/posts/{id}")]
async fn update(req: HttpRequest, post: web::Json<PostParams>) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let post = Post::update(id, post.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[delete("/posts/{id}")]
async fn delete(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let post = Post::delete(id)?;
    Ok(HttpResponse::Ok().json(post))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
