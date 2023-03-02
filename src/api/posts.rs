use actix_web::{get, post, patch, delete, web, HttpResponse};

#[get("/posts")]
async fn list() -> HttpResponse {
    HttpResponse::Ok().body("posts list")
}

#[post("/posts")]
async fn create() -> HttpResponse {
    HttpResponse::Ok().body("posts create")
}

#[get("/posts/{id}")]
async fn detail(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!("posts detail {}", id))
}

#[patch("/posts/{id}")]
async fn update(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!("posts update {}", id))
}

#[delete("/posts/{id}")]
async fn delete(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();

    HttpResponse::Ok().body(format!("posts delete {}", id))
}
