use actix_web::web;

use crate::errors::CustomError;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::JsonConfig::default().error_handler(|err, _req| {
        CustomError::SerializeError(format!("Invalid JSON: {}", err)).into()
    }));
}
