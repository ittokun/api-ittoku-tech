use crate::domain::posts::usecase;
use crate::error_handler::CustomError;

use actix_web::{get, web, HttpResponse};

#[get("/posts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let posts = usecase::find_all().await?;
    Ok(HttpResponse::Ok().json(posts))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
}
