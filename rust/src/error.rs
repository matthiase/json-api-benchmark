use actix_web::{error::ResponseError, HttpResponse, http::StatusCode };
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
    message: String,
}

#[derive(Error, Debug)]
pub enum ApplicationError {
    //#[error("Bad request: {}", _0)]
    //BadRequest(String),

    #[error("Not found")]
    NotFound,

    #[error("Internal server error")]
    InternalServerError,
}

impl ResponseError for ApplicationError {
    fn error_response(&self) -> HttpResponse {
        match self {
            //Self::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Self::NotFound => {
                let response = ErrorResponse{
                    code: StatusCode::NOT_FOUND.as_u16(),
                    error: "NotFound".to_string(),
                    message: "Not found".to_string()
                };
                HttpResponse::NotFound().json(response)
            },
            _ => {
                let response = ErrorResponse{
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    error: "InternalServerError".to_string(),
                    message: "Internal server error".to_string()
                };
                HttpResponse::InternalServerError().json(response)
            },
        }
    }
}

impl std::convert::From<sqlx::Error> for ApplicationError {
    fn from(err: sqlx::Error) -> ApplicationError {
        match err {
            sqlx::Error::RowNotFound => ApplicationError::NotFound,
            _ => ApplicationError::InternalServerError,
        }
    }
}

impl std::convert::From<std::io::Error> for ApplicationError {
    fn from(err: std::io::Error) -> ApplicationError {
        match err {
            _ => ApplicationError::InternalServerError
        }
    }
}

impl std::convert::From<anyhow::Error> for ApplicationError {
    fn from(err: anyhow::Error) -> ApplicationError {
        match err {
            _ => ApplicationError::InternalServerError,
        }
    }
}
