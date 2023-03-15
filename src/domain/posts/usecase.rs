use crate::domain::posts::{Posts, Post};
use crate::error_handler::CustomError;

use serde_json::to_string_pretty;

pub async fn find_all() -> Result<String, CustomError> {
    let posts = Posts::find_all()?;
    let posts = to_string_pretty(&posts).unwrap();
    Ok(posts)
}

pub async fn create(post: Post) -> Result<String, CustomError> {
    let post = Posts::create(post)?;
    let post = to_string_pretty(&post).unwrap();
    Ok(post)
}

pub async fn find_by_id(id: i32) -> Result<String, CustomError> {
    let post = Posts::find(id)?;
    let post = to_string_pretty(&post).unwrap();
    Ok(post)
}

pub async fn update(id: i32, post: Post) -> Result<String, CustomError> {
    let post = Posts::update(id, post)?;
    let post = to_string_pretty(&post).unwrap();
    Ok(post)
}

pub async fn delete(id: i32) -> Result<String, CustomError> {
    let post = Posts::delete(id)?;
    let post = to_string_pretty(&post).unwrap();
    Ok(post)
}
