use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use sqlx::Error as SqlxError;

#[derive(Debug, Display)]
pub enum AppError {
    #[display(fmt = "Database error: {}", _0)]
    Database(SqlxError),

    #[display(fmt = "Not found: {}", _0)]
    NotFound(String),
}

impl std::error::Error for AppError {}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => HttpResponse::InternalServerError().body("Database error"),
            AppError::NotFound(msg) => HttpResponse::NotFound().body(msg.clone()),
        }
    }
}