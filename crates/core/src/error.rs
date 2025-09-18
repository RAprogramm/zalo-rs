use std::path::PathBuf;

use figment::Error as FigmentError;
use masterror::{AppError, AppErrorKind, AppResult, Error};
use tracing::dispatcher::SetGlobalDefaultError;
use tracing_subscriber::filter::ParseError as FilterParseError;

/// Result alias for operations in the core crate.
///
/// This is a thin wrapper around [`AppResult`] that defaults to [`CoreError`]
/// as the error type.
pub type CoreResult<T> = AppResult<T, CoreError>;

/// Top-level error type used across the core crate.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Configuration subsystem failure.
    #[error(transparent)]
    Config(#[from] ConfigError),
    /// Observability/logging subsystem failure.
    #[error(transparent)]
    Observability(#[from] ObservabilityError),
}

/// Errors emitted when loading runtime configuration.
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Configuration file is not accessible.
    #[error("configuration file not found at {path}")]
    MissingFile {
        /// Path to the configuration file that could not be found.
        path: PathBuf,
    },
    /// Figment was unable to extract the configuration model.
    #[error("failed to extract configuration: {source}")]
    Extraction {
        /// Source extraction error produced by Figment.
        #[source]
        source: Box<FigmentError>,
    },
}

impl From<FigmentError> for ConfigError {
    fn from(error: FigmentError) -> Self {
        Self::Extraction {
            source: Box::new(error),
        }
    }
}

/// Errors emitted by the observability subsystem.
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
        #[from]
        /// Source error triggered when installing the global subscriber.
        source: SetGlobalDefaultError,
    },
}

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        AppError::with(AppErrorKind::Config, error.to_string())
    }
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

impl From<CoreError> for AppError {
    fn from(error: CoreError) -> Self {
        match error {
            CoreError::Config(inner) => inner.into(),
            CoreError::Observability(inner) => inner.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::EnvFilter;

    #[test]
    fn config_error_maps_to_app_error() {
        let error = ConfigError::MissingFile {
            path: PathBuf::from("/tmp/missing.toml"),
        };
        let app_error = AppError::from(error);

        assert!(matches!(app_error.kind, AppErrorKind::Config));
    }

    #[test]
    fn observability_filter_maps_to_config_kind() {
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
}
