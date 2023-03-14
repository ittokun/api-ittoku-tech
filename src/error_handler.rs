use actix_web::http::StatusCode;
use actix_web::body::BoxBody;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{Serialize, Deserialize};
use serde_json::json;
use core::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomError {
    pub status_code: u16,
    pub message: String,
}

impl CustomError {
    pub fn new(status_code: u16, message: String) -> CustomError {
        CustomError {
            status_code,
            message,
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                CustomError::new(404, "Not Found".to_string())
            },
            err => CustomError::new(500, format!("Unknown Diesel error: {}", err)),
        }
    }
}

impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let message = match status_code.as_u16() < 500 {
            true => self.message.clone(),
            false => "Internal server error".to_string(),
        };

        HttpResponse::build(status_code).json(json!({ "message": message }))
    }
}
