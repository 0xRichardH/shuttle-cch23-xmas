use std::fmt::Display;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
    #[error(transparent)]
    ImageError(#[from] image::ImageError),
    #[error(transparent)]
    MultipartError(#[from] axum_extra::extract::multipart::MultipartError),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("Bad request: {0}")]
    BadRequest(String),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {}", self);

        match self {
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, format!("Bad request: {}", msg)).into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self),
            )
                .into_response(),
        }
    }
}
