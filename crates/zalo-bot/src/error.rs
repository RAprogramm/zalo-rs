use hmac::digest::InvalidLength;
use thiserror::Error;
use tracing::dispatcher::SetGlobalDefaultError;
use tracing_subscriber::filter::ParseError as FilterParseError;
use zalo_types::{AppError, AppErrorKind, AppResult, TypesError};

/// Convenient result alias for bot-specific operations.
pub type BotResult<T> = AppResult<T, BotError>;

/// Top-level error type surfaced by the bot utilities.
#[derive(Debug, Error)]
pub enum BotError {
    /// Shared type or configuration failure.
    #[error(transparent)]
    Types(#[from] TypesError),
    /// Observability initialisation failed.
    #[error(transparent)]
    Observability(#[from] ObservabilityError),
    /// Incoming webhook signature is not valid.
    #[error(transparent)]
    Signature(#[from] SignatureError),
}

impl From<BotError> for AppError {
    fn from(error: BotError) -> Self {
        match error {
            BotError::Types(inner) => inner.into(),
            BotError::Observability(inner) => inner.into(),
            BotError::Signature(inner) => inner.into(),
        }
    }
}

/// Errors produced by the observability subsystem.
#[derive(Debug, Error)]
pub enum ObservabilityError {
    /// Invalid tracing filter expression.
    #[error("invalid tracing filter `{filter}`: {source}")]
    InvalidFilter {
        /// Original filter expression.
        filter: String,
        /// Source parse error raised by `tracing-subscriber`.
        #[source]
        source: FilterParseError,
    },
    /// Failed to install the global tracing subscriber.
    #[error("failed to install tracing subscriber: {source}")]
    Install {
        /// Source error triggered when installing the global subscriber.
        #[from]
        source: SetGlobalDefaultError,
    },
}

impl From<ObservabilityError> for AppError {
    fn from(error: ObservabilityError) -> Self {
        match &error {
            ObservabilityError::InvalidFilter { .. } => {
                AppError::with(AppErrorKind::Config, error.to_string())
            }
            ObservabilityError::Install { .. } => {
                AppError::with(AppErrorKind::Internal, error.to_string())
            }
        }
    }
}

/// Errors emitted when verifying webhook signatures.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SignatureError {
    /// The signature header is missing.
    #[error("missing webhook signature header")]
    Missing,
    /// The signature does not match the expected value.
    #[error("webhook signature verification failed")]
    VerificationFailed,
    /// The configured secret has an invalid length for the HMAC algorithm.
    #[error("invalid secret length: {0}")]
    InvalidSecretLength(#[from] InvalidLength),
}

impl From<SignatureError> for AppError {
    fn from(error: SignatureError) -> Self {
        match &error {
            SignatureError::Missing | SignatureError::VerificationFailed => {
                AppError::with(AppErrorKind::Unauthorized, error.to_string())
            }
            SignatureError::InvalidSecretLength(_) => {
                AppError::with(AppErrorKind::Config, error.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::EnvFilter;

    #[test]
    fn observability_filter_maps_to_config_error() {
        let parse_error = "="
            .parse::<EnvFilter>()
            .expect_err("invalid filter should trigger an error");
        let error = ObservabilityError::InvalidFilter {
            filter: "=".to_owned(),
            source: parse_error,
        };
        let app_error = AppError::from(error);

        assert!(matches!(app_error.kind, AppErrorKind::Config));
    }

    #[test]
    fn signature_error_maps_to_unauthorized_kind() {
        let app_error = AppError::from(SignatureError::VerificationFailed);

        assert!(matches!(app_error.kind, AppErrorKind::Unauthorized));
    }

    #[test]
    fn bot_error_from_types_preserves_kind() {
        let types_error = TypesError::with_message("boom");
        let app_error = AppError::from(BotError::from(types_error));

        assert!(matches!(app_error.kind, AppErrorKind::Internal));
    }
}
