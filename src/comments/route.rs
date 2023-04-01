use crate::api_error::ApiError;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use uuid::Uuid;

use chrono::prelude::*;

#[get("/posts/{post_id}/comments")]
async fn find_all(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let post_id: Uuid = find_query_id(&req, "post_id");
    Ok(HttpResponse::Ok().json(json!({
        "total_count": 1,
        "comments": [
        {
            "id": Uuid::new_v4(),
            "body": "Find All Comments",
            "post_id": post_id,
            "created_at": Some(Utc::now().naive_utc()),
            "updated_at": Some(Utc::now().naive_utc()),
        }
    ]
    })))
}

#[post("/posts/{post_id}/comments/{comment_id}")]
async fn create(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let post_id: Uuid = find_query_id(&req, "post_id");
    let comment_id: Uuid = find_query_id(&req, "comment_id");
    Ok(HttpResponse::Ok().json(json!({
        "id": comment_id,
        "body": "Create Comment",
        "post_id": post_id,
        "created_at": Some(Utc::now().naive_utc()),
        "updated_at": Some(Utc::now().naive_utc())
    })))
}

#[delete("/posts/{post_id}/comments/{comment_id}")]
async fn delete(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let post_id: Uuid = find_query_id(&req, "post_id");
    let comment_id: Uuid = find_query_id(&req, "comment_id");
    Ok(HttpResponse::Ok().json(json!({
        "id": comment_id,
        "body": "Delete Comment",
        "post_id": post_id,
        "created_at": Some(Utc::now().naive_utc()),
        "updated_at": Some(Utc::now().naive_utc())
    })))
}

fn find_query_id(req: &HttpRequest, params: &str) -> Uuid {
    req.match_info().query(params).parse().unwrap_or_default()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(create);
    cfg.service(delete);
}
