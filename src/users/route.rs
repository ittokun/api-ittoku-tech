use crate::api_error::ApiError;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use serde_json::to_string_pretty;
use uuid::Uuid;

use chrono::prelude::*;

#[get("/users")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let users = serde_json::json!({
        "total_count": 1,
        "users": [
        {
            "uid": Uuid::new_v4(),
            "created_at": Utc::now().naive_utc(),
            "updated_at": Utc::now().naive_utc(),
        }
    ]
    });
    let users = to_string_pretty(&users).unwrap();
    Ok(HttpResponse::Ok().body(users))
}

#[get("/users/{id}")]
async fn find(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let user = serde_json::json!({
        "uid": id,
        "created_at": Utc::now().naive_utc(),
        "updated_at": Utc::now().naive_utc(),
    });
    let user = to_string_pretty(&user).unwrap();
    Ok(HttpResponse::Ok().body(user))
}

#[post("/users")]
async fn create(/* user: web::Json<UserParams> */) -> Result<HttpResponse, ApiError> {
    // user.validate()?;
    let user = serde_json::json!({
        "uid": Uuid::new_v4(),
        "created_at": Utc::now().naive_utc(),
        "updated_at": Utc::now().naive_utc(),
    });
    let user = to_string_pretty(&user).unwrap();
    Ok(HttpResponse::Created().body(user))
}

#[patch("/users/{id}")]
async fn update(req: HttpRequest, /* user: web::Json<UserParams> */) -> Result<HttpResponse, ApiError> {
    // user.validate()?;
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let user = serde_json::json!({
        "uid": id,
        "created_at": Utc::now().naive_utc(),
        "updated_at": Utc::now().naive_utc(),
    });
    let user = to_string_pretty(&user).unwrap();
    Ok(HttpResponse::Ok().body(user))
}

#[delete("/users/{id}")]
async fn delete(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let id: Uuid = req.match_info().query("id").parse().unwrap_or_default();
    let user = serde_json::json!({
        "uid": id,
        "created_at": Utc::now().naive_utc(),
        "updated_at": Utc::now().naive_utc(),
    });
    let user = to_string_pretty(&user).unwrap();
    Ok(HttpResponse::Ok().body(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
