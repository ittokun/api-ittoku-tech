// use crate::comments::*;
use crate::posts::*;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

pub static VALID_JSON: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "Test Post",
        "body":  "This is a Test",
    })
});

pub static INVALID_JSON: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "",
        "body":  "",
    })
});

pub static INVALID_JSON_2: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "a".repeat(257),
        "body":  "a".repeat(65537),
    })
});

pub static RESP_NOTFOUND: Lazy<Value> =
    Lazy::new(|| json!({"code": 404, "message": "Not Found".to_string()}));

pub static RESP_REQUIRED: Lazy<Value> = Lazy::new(|| {
    json!({ "errors": [
        { "code": 422, "message": "body is required"},
        { "code": 422, "message": "title is required"},
    ]})
});

pub static RESP_TOOLONG: Lazy<Value> = Lazy::new(|| {
    json!({ "errors": [
        { "code": 422, "message": "body is too long"},
        { "code": 422, "message": "title is too long"},
    ]})
});

pub static RESP_INCORRECT: Lazy<Value> =
    Lazy::new(|| json!({ "code": 400, "message": "Post Request is Incorrect" }));

pub fn create_post() -> Post {
    let post = PostParams {
        title: "create_post()".to_string(),
        body: "This is a create_post()".to_string(),
        updated_at: None,
    };
    Post::create(post).expect("Failed create_post()")
}

pub fn delete_post(post: Post) {
    Post::delete(post.id).expect("Failed delete_post(id)");
}
