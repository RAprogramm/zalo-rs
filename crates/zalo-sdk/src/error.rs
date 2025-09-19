use thiserror::Error;
use zalo_types::{AppError, AppErrorKind, AppResult};

/// Result alias for operations in the SDK crate.
pub type SdkResult<T> = AppResult<T, SdkError>;

/// Errors returned by the mini app SDK utilities.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SdkError {
    /// Provided app identifier is empty or malformed.
    #[error("invalid app identifier: {0}")]
    InvalidAppId(String),
    /// Provided OA identifier is empty or malformed.
    #[error("invalid oa identifier: {0}")]
    InvalidOaId(String),
}

impl From<SdkError> for AppError {
    fn from(error: SdkError) -> Self {
        AppError::with(AppErrorKind::Validation, error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdk_error_maps_to_validation_kind() {
        let app_error = AppError::from(SdkError::InvalidAppId("".to_owned()));

        assert!(matches!(app_error.kind, AppErrorKind::Validation));
    }
}
