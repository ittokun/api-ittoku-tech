use crate::db::schema::posts::dsl::*;
use crate::db::establish_connection;
use crate::db::models::Post;

use actix_web::{get, post, patch, delete, web, HttpResponse};
use diesel::prelude::*;

#[get("/posts")]
async fn list() -> HttpResponse {
    let connection = &mut establish_connection();
    let results: Vec<Post> = posts
        .load::<Post>(connection)
        .expect("Error loading posts");

    HttpResponse::Ok().json(results)
}

#[post("/posts")]
async fn create() -> HttpResponse {
    HttpResponse::Ok().body("posts create")
}

#[get("/posts/{post_id}")]
async fn detail(path: web::Path<u32>) -> HttpResponse {
    let post_id = path.into_inner();

    HttpResponse::Ok().body(format!("posts detail {}", post_id))
}

#[patch("/posts/{post_id}")]
async fn update(path: web::Path<u32>) -> HttpResponse {
    let post_id = path.into_inner();

    HttpResponse::Ok().body(format!("posts update {}", post_id))
}

#[delete("/posts/{post_id}")]
async fn delete(path: web::Path<u32>) -> HttpResponse {
    let post_id = path.into_inner();

    HttpResponse::Ok().body(format!("posts delete {}", post_id))
}
