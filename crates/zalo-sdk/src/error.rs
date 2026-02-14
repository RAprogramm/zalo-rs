// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use masterror::Error;
use zalo_types::{AppError, AppErrorKind, AppResult};

/// Result alias for operations in the SDK crate.
pub type SdkResult<T> = AppResult<T, SdkError>;

/// Errors returned by the Zalo Mini App SDK.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum SdkError {
    /// Provided app identifier is empty or malformed.
    #[error("invalid app identifier: {0:?}")]
    InvalidAppId(String),

    /// Provided OA identifier is empty or malformed.
    #[error("invalid oa identifier: {0:?}")]
    InvalidOaId(String),

    /// Authorization code is empty or blank.
    #[error("invalid authorization code: {0:?}")]
    InvalidAuthCode(String),

    /// Access token is empty or blank.
    #[error("invalid access token: {0:?}")]
    InvalidAccessToken(String),

    /// No OAuth scopes were supplied for the authorization request.
    #[error("authorization request must include at least one scope")]
    EmptyScopes,

    /// Phone number failed format validation.
    #[error("invalid phone number: {0:?}")]
    InvalidPhoneNumber(String),

    /// Geographic coordinates are outside the valid WGS-84 range or are NaN.
    #[error("invalid coordinates: latitude={latitude}, longitude={longitude}")]
    InvalidCoordinates {
        /// Latitude value that failed validation.
        latitude:  f64,
        /// Longitude value that failed validation.
        longitude: f64
    },

    /// Storage key is empty or contains only whitespace.
    #[error("storage key must not be empty")]
    StorageKeyEmpty,

    /// Storage key exceeds the platform-imposed byte limit.
    #[error("storage key length {length} exceeds maximum {maximum}")]
    StorageKeyTooLong {
        /// Actual byte length of the supplied key.
        length:  usize,
        /// Maximum allowed byte length.
        maximum: usize
    },

    /// Storage value exceeds the platform-imposed byte limit.
    #[error("storage value length {length} exceeds maximum {maximum}")]
    StorageValueTooLong {
        /// Actual byte length of the supplied value.
        length:  usize,
        /// Maximum allowed byte length.
        maximum: usize
    },

    /// Payment amount must be a positive non-zero value.
    #[error("payment amount must be greater than zero")]
    InvalidAmount,

    /// Order identifier is empty or blank.
    #[error("invalid order identifier: {0:?}")]
    InvalidOrderId(String),

    /// Payment description is empty or blank.
    #[error("payment description must not be empty")]
    EmptyDescription,

    /// Route path is missing a leading slash or is otherwise malformed.
    #[error("invalid route path: {0:?}")]
    InvalidRoutePath(String),

    /// Navigation title is empty or exceeds the character limit.
    #[error("invalid navigation title: {0:?}")]
    InvalidNavTitle(String),

    /// URL is empty or blank.
    #[error("invalid url: {0:?}")]
    InvalidUrl(String),

    /// Share card title is empty or blank.
    #[error("invalid share title: {0:?}")]
    InvalidShareTitle(String),

    /// User identifier is empty, blank, or fails validation rules.
    ///
    /// This error is returned when constructing a strongly-typed `UserId`
    /// from an invalid input string.
    ///
    /// The contained value is the original user-supplied identifier.
    #[error("invalid user identifier: {0:?}")]
    InvalidUserId(String),

    /// Birthday string does not match the expected `dd/mm/yyyy` format
    /// or represents an invalid calendar date.
    ///
    /// This error is returned when parsing a `Birthday` from external input.
    ///
    /// The contained value is the original unparsed string.
    #[error("invalid birthday format: {0:?}")]
    InvalidBirthday(String)
}

impl From<SdkError> for AppError {
    fn from(error: SdkError) -> Self {
        match &error {
            SdkError::InvalidAppId(_)
            | SdkError::InvalidOaId(_)
            | SdkError::InvalidAuthCode(_)
            | SdkError::InvalidAccessToken(_)
            | SdkError::EmptyScopes
            | SdkError::InvalidPhoneNumber(_)
            | SdkError::InvalidCoordinates {
                ..
            }
            | SdkError::StorageKeyEmpty
            | SdkError::StorageKeyTooLong {
                ..
            }
            | SdkError::StorageValueTooLong {
                ..
            }
            | SdkError::InvalidAmount
            | SdkError::InvalidOrderId(_)
            | SdkError::InvalidUserId(_)
            | SdkError::InvalidBirthday(_)
            | SdkError::EmptyDescription
            | SdkError::InvalidRoutePath(_)
            | SdkError::InvalidNavTitle(_)
            | SdkError::InvalidUrl(_)
            | SdkError::InvalidShareTitle(_) => {
                AppError::with(AppErrorKind::Validation, error.to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_variants_map_to_validation_kind() {
        let cases: &[SdkError] = &[
            SdkError::InvalidAppId("x".to_owned()),
            SdkError::InvalidOaId("x".to_owned()),
            SdkError::InvalidAuthCode("x".to_owned()),
            SdkError::InvalidAccessToken("x".to_owned()),
            SdkError::EmptyScopes,
            SdkError::InvalidPhoneNumber("x".to_owned()),
            SdkError::InvalidCoordinates {
                latitude:  999.0,
                longitude: 0.0
            },
            SdkError::StorageKeyEmpty,
            SdkError::StorageKeyTooLong {
                length:  200,
                maximum: 128
            },
            SdkError::StorageValueTooLong {
                length:  5000,
                maximum: 4096
            },
            SdkError::InvalidAmount,
            SdkError::InvalidOrderId("".to_owned()),
            SdkError::InvalidUserId("u1".to_owned()),
            SdkError::InvalidBirthday("bad".to_owned()),
            SdkError::EmptyDescription,
            SdkError::InvalidRoutePath("home".to_owned()),
            SdkError::InvalidNavTitle("".to_owned()),
            SdkError::InvalidUrl("".to_owned()),
            SdkError::InvalidShareTitle("".to_owned())
        ];

        for case in cases {
            let app_error = AppError::from(case.clone());
            assert!(
                matches!(app_error.kind, AppErrorKind::Validation),
                "expected Validation for {case:?}"
            );
        }
    }

    #[test]
    fn invalid_coordinates_display_contains_values() {
        let err = SdkError::InvalidCoordinates {
            latitude:  999.0,
            longitude: -999.0
        };
        let msg = err.to_string();
        assert!(msg.contains("999"), "message: {msg}");
    }

    #[test]
    fn storage_key_too_long_display_contains_sizes() {
        let err = SdkError::StorageKeyTooLong {
            length:  200,
            maximum: 128
        };
        let msg = err.to_string();
        assert!(msg.contains("200"));
        assert!(msg.contains("128"));
    }

    #[test]
    fn storage_value_too_long_display_contains_sizes() {
        let err = SdkError::StorageValueTooLong {
            length:  5000,
            maximum: 4096
        };
        let msg = err.to_string();
        assert!(msg.contains("5000"));
        assert!(msg.contains("4096"));
    }
}
