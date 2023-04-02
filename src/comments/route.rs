use crate::api_error::ApiError;
use crate::comments::*;
use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use serde_json::to_string_pretty;
use uuid::Uuid;
use validator::Validate;

use chrono::prelude::*;

#[get("/posts/{post_id}/comments")]
async fn find_all(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let post_id: Uuid = find_query_id(&req, "post_id");
    let comments = Comment::find_all(post_id).await?;
    let comments = CommentFindAll::new(comments.len(), comments);
    let comments = to_string_pretty(&comments).unwrap();
    Ok(HttpResponse::Ok().body(comments))
}

#[post("/posts/{post_id}/comments")]
async fn create(
    req: HttpRequest,
    comment: web::Json<CommentParams>,
) -> Result<HttpResponse, ApiError> {
    comment.validate()?;
    let post_id: Uuid = find_query_id(&req, "post_id");
    let comment = Comment::create(post_id, comment.into_inner()).await?;
    let comment = to_string_pretty(&comment).unwrap();
    Ok(HttpResponse::Ok().body(comment))
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
