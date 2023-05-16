use once_cell::sync::Lazy;
use serde_json::{json, Value};

#[allow(dead_code)]
pub static RESP_ROOT: Lazy<Value> = Lazy::new(|| {
    json!({
      "post_detail_url": "http://0.0.0.0:8080/posts/{id}",
      "post_list_url": "http://0.0.0.0:8080/posts"
    })
});

#[allow(dead_code)]
pub static RESP_NOTFOUND: Lazy<Value> =
    Lazy::new(|| json!({"status": 404, "message": "Not Found".to_string()}));

#[allow(dead_code)]
pub static RESP_INVALID_JSON: Lazy<Value> =
    Lazy::new(|| json!({"status": 400, "message": "Invalid JSON Data".to_string()}));

#[allow(dead_code)]
pub static NEW_POST: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "test title",
        "text": "test text",
    })
});

#[allow(dead_code)]
pub static NEW_POST_UPDATED: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "test title updated",
        "text": "test text updated",
    })
});

#[allow(dead_code)]
pub static NEW_POST_EMPTY: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "",
        "text": "",
    })
});

#[allow(dead_code)]
pub static NEW_POST_TOO_LONG: Lazy<Value> = Lazy::new(|| {
    json!({
        "title": "x".repeat(257),
        "text": "x".repeat(65537),
    })
});

#[allow(dead_code)]
pub static RESP_POST_EMPTY: Lazy<Value> = Lazy::new(|| {
    json!({
        "errors": [
            {
                "status": 422,
                "message": "title cannot be empty",
            },
            {
                "status": 422,
                "message": "text cannot be empty",
            },
        ]
    })
});

#[allow(dead_code)]
pub static RESP_POST_TOO_LONG: Lazy<Value> = Lazy::new(|| {
    json!({
        "errors": [
            {
                "status": 422,
                "message": "title cannot be longer than 256 characters",
            },
            {
                "status": 422,
                "message": "text cannot be longer than 65536 characters",
            },
        ]
    })
});
