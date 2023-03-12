use crate::domain::posts::{usecase, Post};
use crate::error_handler::CustomError;

use actix_web::{get, post, web, HttpResponse};

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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(create);
}
