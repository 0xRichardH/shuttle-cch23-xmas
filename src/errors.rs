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
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    SqlxMigrationError(#[from] sqlx::migrate::MigrateError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    NumParseIntError(#[from] std::num::ParseIntError),

    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("{{\"result\":\"naughty\",\"reason\":\"{1}\"}}")]
    InvalidPasswordGameInput(StatusCode, String),
    #[error("An internal error occurred: {0}")]
    Internal(anyhow::Error),
}

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {}", self);

        match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            AppError::InvalidPasswordGameInput(status, _) => {
                (status, self.to_string()).into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self),
            )
                .into_response(),
        }
    }
}
