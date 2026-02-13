// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::Error;
use zalo_types::{AppError, AppErrorKind, AppResult};

/// Result alias for HTTP client operations.
pub type HttpResult<T> = AppResult<T, HttpError>;

/// Errors produced by the Zalo OA HTTP client.
#[derive(Debug, Error)]
pub enum HttpError {
    /// The underlying HTTP transport failed.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),
    /// The Zalo OA API returned a non-zero error code.
    #[error("Zalo API error {code}: {message}")]
    Api {
        /// Numeric error code from the `error` field of the API response.
        code:    i64,
        /// Human-readable message from the `message` field of the API response.
        message: String
    },
    /// The server responded with an unexpected HTTP status code.
    #[error("unexpected HTTP {status}: {body}")]
    UnexpectedStatus {
        /// HTTP status code returned by the server.
        status: u16,
        /// Raw response body for diagnostic purposes.
        body:   String
    },
    /// JSON deserialization of the response body failed.
    #[error("response deserialization failed: {0}")]
    Deserialization(#[from] serde_json::Error),
    /// Client configuration is invalid.
    #[error("invalid client configuration: {0}")]
    Configuration(String),
    /// The server indicated that the rate limit has been exceeded (error -210).
    #[error("rate limit exceeded")]
    RateLimited,
    /// The access token is invalid or expired (error -204).
    #[error("access token invalid or expired")]
    Unauthorized
}

impl HttpError {
    /// Creates an [`HttpError::Api`] value from code and message.
    ///
    /// When the code maps to a well-known sentinel (-204, -210) the returned
    /// variant is the corresponding typed error rather than the generic `Api`
    /// variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::error::HttpError;
    ///
    /// let err = HttpError::from_api_response(-210, "rate limit");
    /// assert!(matches!(err, HttpError::RateLimited));
    ///
    /// let err = HttpError::from_api_response(-204, "expired");
    /// assert!(matches!(err, HttpError::Unauthorized));
    ///
    /// let err = HttpError::from_api_response(-202, "bad param");
    /// assert!(matches!(
    ///     err,
    ///     HttpError::Api {
    ///         code: -202,
    ///         ..
    ///     }
    /// ));
    /// ```
    #[must_use]
    pub fn from_api_response(code: i64, message: impl Into<String>) -> Self {
        match code {
            -204 | -240 => Self::Unauthorized,
            -210 => Self::RateLimited,
            _ => Self::Api {
                code,
                message: message.into()
            }
        }
    }

    /// Creates an [`HttpError::Configuration`] value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::error::HttpError;
    ///
    /// let err = HttpError::configuration("access token must not be empty");
    /// assert!(matches!(err, HttpError::Configuration(_)));
    /// ```
    #[must_use]
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::Configuration(msg.into())
    }
}

impl From<HttpError> for AppError {
    fn from(error: HttpError) -> Self {
        match &error {
            HttpError::Transport(_)
            | HttpError::UnexpectedStatus {
                ..
            } => AppError::with(AppErrorKind::Network, error.to_string()),
            HttpError::Api {
                ..
            }
            | HttpError::Deserialization(_) => {
                AppError::with(AppErrorKind::Internal, error.to_string())
            }
            HttpError::Configuration(_) => AppError::with(AppErrorKind::Config, error.to_string()),
            HttpError::RateLimited => AppError::with(AppErrorKind::RateLimited, error.to_string()),
            HttpError::Unauthorized => {
                AppError::with(AppErrorKind::Unauthorized, error.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use zalo_types::AppErrorKind;

    use super::*;

    #[test]
    fn rate_limit_code_maps_to_rate_limited_variant() {
        let err = HttpError::from_api_response(-210, "quota exceeded");
        assert!(matches!(err, HttpError::RateLimited));
    }

    #[test]
    fn expired_token_code_maps_to_unauthorized_variant() {
        let err = HttpError::from_api_response(-204, "expired");
        assert!(matches!(err, HttpError::Unauthorized));
    }

    #[test]
    fn legacy_api_code_also_maps_to_unauthorized() {
        let err = HttpError::from_api_response(-240, "v2 deprecated");
        assert!(matches!(err, HttpError::Unauthorized));
    }

    #[test]
    fn unknown_negative_code_becomes_api_variant() {
        let err = HttpError::from_api_response(-202, "bad param");
        assert!(matches!(
            err,
            HttpError::Api {
                code: -202,
                ..
            }
        ));
    }

    #[test]
    fn configuration_error_maps_to_config_kind() {
        let err = HttpError::configuration("empty token");
        let app_err = AppError::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Config));
    }

    #[test]
    fn unauthorized_maps_to_unauthorized_kind() {
        let err = HttpError::Unauthorized;
        let app_err = AppError::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::Unauthorized));
    }

    #[test]
    fn rate_limited_maps_to_rate_limited_kind() {
        let err = HttpError::RateLimited;
        let app_err = AppError::from(err);
        assert!(matches!(app_err.kind, AppErrorKind::RateLimited));
    }

    #[test]
    fn configuration_variant_display() {
        let err = HttpError::configuration("missing token");
        assert_eq!(
            err.to_string(),
            "invalid client configuration: missing token"
        );
    }

    #[test]
    fn api_variant_display_contains_code_and_message() {
        let err = HttpError::Api {
            code:    -213,
            message: "user not subscribed".to_owned()
        };
        let text = err.to_string();
        assert!(text.contains("-213"));
        assert!(text.contains("user not subscribed"));
    }

    #[test]
    fn unexpected_status_display_contains_status_and_body() {
        let err = HttpError::UnexpectedStatus {
            status: 503,
            body:   "service unavailable".to_owned()
        };
        let text = err.to_string();
        assert!(text.contains("503"));
        assert!(text.contains("service unavailable"));
    }
}
