use actix_web::{error, web, ResponseError};

use crate::api_error::ApiError;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        error::InternalError::from_response(
            err,
            ApiError::SerializeError("Post Request is Incorrect".to_string()).error_response()
        )
        .into()
    }));
}
