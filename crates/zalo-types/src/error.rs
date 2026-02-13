// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
    path::PathBuf
};

use figment::Error as FigmentError;
use masterror::{AppError, AppErrorKind, AppResult, Error};

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
        message: String
    }
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
            message: message.into()
        }
    }
}

/// Errors emitted when loading runtime configuration.
#[derive(Debug)]
pub enum ConfigError {
    /// Configuration file is not accessible.
    MissingFile {
        /// Path to the configuration file that could not be found.
        path: PathBuf
    },
    /// Figment was unable to extract the configuration model.
    Extraction {
        /// Source extraction error produced by Figment.
        source: Box<FigmentError>
    }
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::MissingFile {
                path
            } => {
                write!(f, "configuration file not found at {}", path.display())
            }
            Self::Extraction {
                source
            } => {
                write!(f, "failed to extract configuration: {source}")
            }
        }
    }
}

impl StdError for ConfigError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Extraction {
                source
            } => Some(source.as_ref()),
            Self::MissingFile {
                ..
            } => None
        }
    }
}

impl From<FigmentError> for ConfigError {
    fn from(error: FigmentError) -> Self {
        Self::Extraction {
            source: Box::new(error)
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
            TypesError::Other {
                message
            } => AppError::with(AppErrorKind::Internal, message)
        }
    }
}

#[cfg(test)]
mod tests {
    use figment::Figment;

    use super::*;

    #[test]
    fn config_error_maps_to_app_error() {
        let error = ConfigError::MissingFile {
            path: PathBuf::from("/tmp/missing.toml")
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
    fn missing_file_display_contains_path() {
        let path = PathBuf::from("/etc/zalo/config.toml");
        let error = ConfigError::MissingFile {
            path: path.clone()
        };

        assert!(error.to_string().contains("/etc/zalo/config.toml"));
    }

    #[test]
    fn config_error_source_is_some_for_extraction() {
        let figment_error = Figment::new().extract::<String>().unwrap_err();
        let error = ConfigError::from(figment_error);

        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn config_error_source_is_none_for_missing_file() {
        let error = ConfigError::MissingFile {
            path: PathBuf::from("/tmp/x")
        };

        assert!(StdError::source(&error).is_none());
    }

    #[test]
    fn types_error_with_message_displays_correctly() {
        let err = TypesError::with_message("test error message");
        assert_eq!(err.to_string(), "test error message");
    }
}
