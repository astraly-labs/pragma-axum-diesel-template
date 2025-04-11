use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DbError(String),
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
        };
        (status, Json(self)).into_response()
    }
}
