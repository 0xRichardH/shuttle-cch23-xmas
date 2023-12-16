/// Re-export of the `AppError` type
pub use crate::errors::AppError;

/// An alias for the `Result` type
pub type Result<T> = core::result::Result<T, AppError>;

/// Generic wrapper
/// for external types to type From/TryFrom conversions
pub struct W<T>(pub T);
