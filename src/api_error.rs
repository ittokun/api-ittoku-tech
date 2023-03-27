use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Serialize)]
pub enum ApiError {
    NotFound,
    DatabaseError(String),
    InternalServerError(String),
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<'a> {
    pub code: u16,
    pub message: &'a str,
}

impl ApiResponse<'_> {
    pub fn new(code: u16, message: &str) -> String {
        let api_resp = &ApiResponse { code, message };
        to_string_pretty(api_resp).unwrap()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> Self {
        match error {
            DieselError::DatabaseError(_, err) => {
                let message = err.message().to_string();
                ApiError::DatabaseError(message)
            }
            DieselError::NotFound => ApiError::NotFound,
            err => ApiError::InternalServerError(format!("Diesel Error: {}", err)),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::DatabaseError(_) => StatusCode::CONFLICT,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let status = self.status_code();

        match self {
            ApiError::NotFound => {
                let message = "Not Found";
                HttpResponse::build(status).body(ApiResponse::new(status.as_u16(), message))
            }
            ApiError::DatabaseError(message) => {
                HttpResponse::build(status).body(ApiResponse::new(status.as_u16(), message))
            }
            ApiError::InternalServerError(message) => {
                HttpResponse::build(status).body(ApiResponse::new(status.as_u16(), message))
            }
        }
    }
}
