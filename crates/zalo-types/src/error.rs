use std::error::Error as StdError;
use std::path::PathBuf;

use figment::Error as FigmentError;
use masterror::{AppError, AppErrorKind, AppResult};
use thiserror::Error;

/// Result alias for operations in the `zalo-types` crate.
///
/// The alias guarantees that all fallible operations map to [`TypesError`],
/// which in turn can always be converted into [`AppError`].
pub type TypesResult<T> = AppResult<T, TypesError>;

/// Top-level error type emitted by the shared primitives.
#[derive(Debug, Error)]
pub enum TypesError {
    /// Configuration subsystem failure.
    #[error(transparent)]
    Config(#[from] ConfigError),
    /// Wrapper for other error sources that should be surfaced to callers.
    #[error("{message}")]
    Other {
        /// Human-readable error message.
        message: String,
        /// Optional source error for richer diagnostics.
        #[source]
        source: Option<Box<dyn StdError + Send + Sync>>,
    },
}

impl TypesError {
    /// Creates a new [`TypesError::Other`] value with the provided message.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_types::TypesError;
    ///
    /// let error = TypesError::with_message("something went wrong");
    /// assert_eq!(format!("{error}"), "something went wrong");
    /// ```
    #[must_use]
    pub fn with_message(message: impl Into<String>) -> Self {
        Self::Other {
            message: message.into(),
            source: None,
        }
    }

    /// Attaches a source error to an [`TypesError::Other`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_types::TypesError;
    ///
    /// let source = std::io::Error::new(std::io::ErrorKind::Other, "io");
    /// let error = TypesError::with_message("failed").with_source(source);
    /// assert!(matches!(error, TypesError::Other { source: Some(_), .. }));
    /// ```
    #[must_use]
    pub fn with_source(self, source: impl StdError + Send + Sync + 'static) -> Self {
        match self {
            Self::Other { message, .. } => Self::Other {
                message,
                source: Some(Box::new(source)),
            },
            other => other,
        }
    }
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

impl From<ConfigError> for AppError {
    fn from(error: ConfigError) -> Self {
        AppError::with(AppErrorKind::Config, error.to_string())
    }
}

impl From<TypesError> for AppError {
    fn from(error: TypesError) -> Self {
        match error {
            TypesError::Config(inner) => inner.into(),
            TypesError::Other { message, .. } => AppError::with(AppErrorKind::Internal, message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_error_maps_to_app_error() {
        let error = ConfigError::MissingFile {
            path: PathBuf::from("/tmp/missing.toml"),
        };
        let app_error = AppError::from(error);

        assert!(matches!(app_error.kind, AppErrorKind::Config));
    }

    #[test]
    fn other_error_maps_to_internal_kind() {
        let error = TypesError::with_message("boom");
        let app_error = AppError::from(error);

        assert!(matches!(app_error.kind, AppErrorKind::Internal));
    }

    #[test]
    fn with_source_attaches_context() {
        let source = std::io::Error::new(std::io::ErrorKind::Other, "io");
        let error = TypesError::with_message("failure").with_source(source);

        match error {
            TypesError::Other { source, .. } => {
                assert!(source.is_some());
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
