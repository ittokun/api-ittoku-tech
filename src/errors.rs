use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::Display;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Debug, Display)]
pub enum CustomError {
    NotFound,
    InternalError(String),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse<'a> {
    pub status: u16,
    pub message: &'a str,
}

impl ErrorResponse<'_> {
    pub fn new(status: u16, message: &str) -> String {
        let error_response = ErrorResponse { status, message };
        to_string_pretty(&error_response).unwrap()
    }
}

impl From<DbErr> for CustomError {
    fn from(err: DbErr) -> Self {
        match err {
            DbErr::RecordNotFound(_) => CustomError::NotFound,
            _ => CustomError::InternalError(err.to_string()),
        }
    }
}

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::NotFound => StatusCode::NOT_FOUND,
            CustomError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        match self {
            CustomError::NotFound => {
                let message = "Not Found";
                HttpResponse::build(status).body(ErrorResponse::new(status.as_u16(), &message))
            }
            CustomError::InternalError(message) => {
                HttpResponse::build(status).body(ErrorResponse::new(status.as_u16(), &message))
            }
        }
    }
}
