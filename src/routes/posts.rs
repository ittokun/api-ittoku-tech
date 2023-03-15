use crate::domain::posts::{usecase, Post};
use crate::error_handler::CustomError;

use actix_web::{get, post, patch, delete, web, HttpResponse, HttpRequest};

#[get("/posts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let posts = usecase::find_all().await?;
    Ok(HttpResponse::Ok().body(posts))
}

#[post("/posts")]
async fn create(post: web::Json<Post>) -> Result<HttpResponse, CustomError> {
    let post = usecase::create(post.into_inner()).await?;
    Ok(HttpResponse::Created().body(post))
}

#[get("/posts/{id}")]
async fn detail(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let id: i32 = req.match_info().query("id").parse().unwrap_or_default();
    let post = usecase::find_by_id(id).await?;
    Ok(HttpResponse::Ok().body(post))
}

#[patch("/posts/{id}")]
async fn update(req: HttpRequest, post: web::Json<Post>) -> Result<HttpResponse, CustomError> {
    let id: i32 = req.match_info().query("id").parse().unwrap_or_default();
    let post = usecase::update(id, post.into_inner()).await?;
    Ok(HttpResponse::Ok().body(post))
}

#[delete("/posts/{id}")]
async fn delete(req: HttpRequest) -> Result<HttpResponse, CustomError> {
    let id: i32 = req.match_info().query("id").parse().unwrap_or_default();
    let post = usecase::delete(id).await?;
    Ok(HttpResponse::Ok().body(post))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(create);
    cfg.service(detail);
    cfg.service(update);
    cfg.service(delete);
}
