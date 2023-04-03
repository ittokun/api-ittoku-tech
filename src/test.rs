use crate::comments::*;
use crate::posts::*;
use once_cell::sync::Lazy;
use serde_json::{json, Value};

pub static RESP_NOTFOUND: Lazy<Value> =
    Lazy::new(|| json!({"code": 404, "message": "Not Found".to_string()}));

pub static RESP_INCORRECT: Lazy<Value> =
    Lazy::new(|| json!({ "code": 400, "message": "Post Request is Incorrect" }));

// Post test

pub static POST_VALID_PARAMS: Lazy<Value> =
    Lazy::new(|| json!({ "title": "Test Post", "body":  "This is a Test", }));

pub static POST_INVALID_PARAMS: Lazy<Value> = Lazy::new(|| json!({ "title": "", "body":  "", }));

pub static POST_INVALID_PARAMS_2: Lazy<Value> =
    Lazy::new(|| json!({ "title": "a".repeat(257), "body":  "a".repeat(65537), }));

pub static POST_RESP_REQUIRED: Lazy<Value> = Lazy::new(|| {
    json!({ "errors": [
        { "code": 422, "message": "body is required"},
        { "code": 422, "message": "title is required"},
    ]})
});

pub static POST_RESP_TOOLONG: Lazy<Value> = Lazy::new(|| {
    json!({ "errors": [
        { "code": 422, "message": "body is too long"},
        { "code": 422, "message": "title is too long"},
    ]})
});

// Comment test

pub static COMMENT_VALID_PARAMS: Lazy<Value> = Lazy::new(|| json!({ "body": "Test Comment", }));

pub static COMMENT_INVALID_PARAMS: Lazy<Value> = Lazy::new(|| json!({ "body": "", }));

pub static COMMENT_INVALID_PARAMS_2: Lazy<Value> =
    Lazy::new(|| json!({ "body": "a".repeat(65537) }));

pub static COMMENT_RESP_REQUIRED: Lazy<Value> =
    Lazy::new(|| json!({ "errors": [ { "code": 422, "message": "body is required"}, ]}));

pub static COMMENT_RESP_TOOLONG: Lazy<Value> =
    Lazy::new(|| json!({ "errors": [ { "code": 422, "message": "body is too long"}, ]}));

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

pub fn create_comment(post: Post) -> Comment {
    let comment = CommentParams {
        body: "This is a create_comment()".to_string(),
        updated_at: None,
    };
    Comment::create(post.id, comment).expect("Failed create_comment()")
}
