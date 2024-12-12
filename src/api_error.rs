use actix_web::{
    error,
    http::StatusCode,
    HttpResponse,
};
use derive_more::derive::Display;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum APIError {
    #[display("Database error: {}", _0)]
    DatabaseError(DbErr),

    #[display("Authentication error: {}", _0)]
    AuthenticationError(String),

    #[display("Validation error: {}", _0)]
    ValidationError(String),

    #[display("internal server error")]
    InternalServerError,

    #[display("Not found error")]
    NotFoundError(String),
}

impl From<DbErr> for APIError {
    fn from(value: DbErr) -> Self {
        APIError::DatabaseError(value)
    }
}

impl APIError {
    pub fn from_option<T>(option: Option<T>, msg: &str) -> Result<T, APIError> {
        option.ok_or_else(|| APIError::NotFoundError(msg.to_string()))
    }
}

impl error::ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        match self {
            APIError::DatabaseError(message) => HttpResponse::InternalServerError().json(ErrorResponse{
                error: "DatabaseError".to_string(),
                message: message.to_string(),
            }),
            APIError::AuthenticationError(message) => HttpResponse::Unauthorized().json(ErrorResponse{
                error: "AuthenticationError".to_string(),
                message: message.clone()
            }),
            APIError::ValidationError(message) => HttpResponse::BadRequest().json(ErrorResponse{
                error: "ValidationError".to_string(),
                message: message.clone()
            }),
            APIError::InternalServerError => HttpResponse::InternalServerError().json(ErrorResponse{
                error: "InternalServerError".to_string(),
                message: "An unexpected error occurred".to_string(),
            }),
            APIError::NotFoundError(message) => HttpResponse::NotFound().json(ErrorResponse{
                error: "NotFoundError".to_string(),
                message: message.clone()
            }),
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            APIError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            APIError::ValidationError(_) => StatusCode::BAD_REQUEST,
            APIError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            APIError::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}