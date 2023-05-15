use once_cell::sync::Lazy;
use serde_json::{json, Value};

pub static RESP_ROOT: Lazy<Value> = Lazy::new(|| {
    json!({
      "post_detail_url": "http://0.0.0.0:8080/posts/{id}",
      "post_list_url": "http://0.0.0.0:8080/posts"
    })
});

pub static RESP_NOTFOUND: Lazy<Value> =
    Lazy::new(|| json!({"status": 404, "message": "Not Found".to_string()}));

pub static RESP_INVALID_JSON: Lazy<Value> =
    Lazy::new(|| json!({"status": 400, "message": "Invalid JSON Data".to_string()}));
