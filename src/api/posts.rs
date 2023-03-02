use crate::db::schema::posts::dsl::*;
use crate::db::establish_connection;
use crate::db::models::{Post, NewPost};

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
async fn create(info: web::Json<NewPost>) -> HttpResponse {
    let conn = &mut establish_connection();

    let new_post = NewPost {
        title: info.title.clone(),
        body: info.body.clone(),
        published: info.published,
    };

    let post: Post = diesel::insert_into(posts)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post");

    HttpResponse::Created().json(post)
}

#[get("/posts/{post_id}")]
async fn detail(path: web::Path<i32>) -> HttpResponse {
    let post_id = path.into_inner();
    let conn = &mut establish_connection();

    let post = posts.filter(id.eq(post_id))
        .get_result::<Post>(conn)
        .expect("Post Not Found");

    HttpResponse::Ok().json(post)
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
