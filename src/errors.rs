use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[derive(Debug, Display, Error)]
pub enum CustomError {
    #[display(fmt = "Not found")]
    NotFound,
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

impl ResponseError for CustomError {
    fn status_code(&self) -> StatusCode {
        match self {
            CustomError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let message = self.to_string();

        HttpResponse::build(status).body(ErrorResponse::new(status.as_u16(), &message))
    }
}
