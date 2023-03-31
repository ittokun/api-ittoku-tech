use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fmt::{Debug, Display, Formatter};
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub enum ApiError {
    NotFound,
    DatabaseError(String),
    SerializeError(String),
    ValidationError(String),
    InternalServerError(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse<'a> {
    pub code: u16,
    pub message: &'a str,
}

#[derive(Debug, Serialize)]
pub struct ApiValidationResponse<'a> {
    pub errors: Vec<ApiResponse<'a>>,
}

impl ApiResponse<'_> {
    pub fn new(code: u16, message: &str) -> String {
        let api_resp = &ApiResponse { code, message };
        to_string_pretty(api_resp).unwrap()
    }
}

impl ApiValidationResponse<'_> {
    pub fn new(errors: &str) -> String {
        let errors = serde_json::from_str::<Vec<ApiResponse>>(errors).unwrap();
        let api_validation_resp = &ApiValidationResponse { errors };
        to_string_pretty(api_validation_resp).unwrap()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(value: ValidationErrors) -> Self {
        let mut errors = Vec::new();

        for (_, v) in value.field_errors() {
            errors.push(ApiResponse::new(422, &v.first().unwrap().code));
        };
        errors.sort();
        let errors = errors.join(",");

        ApiError::ValidationError(format!("[{errors}]"))
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
        match self {
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::DatabaseError(_) => StatusCode::CONFLICT,
            ApiError::SerializeError(_) => StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
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
            ApiError::SerializeError(message) => {
                HttpResponse::build(status).body(ApiResponse::new(status.as_u16(), message))
            }
            ApiError::ValidationError(message) => {
                HttpResponse::build(status).body(ApiValidationResponse::new(message))
            }
            ApiError::InternalServerError(message) => {
                HttpResponse::build(status).body(ApiResponse::new(status.as_u16(), message))
            }
        }
    }
}
