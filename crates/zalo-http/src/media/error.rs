// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Media upload errors.

use masterror::Error;

use crate::error::HttpError;

/// Media upload errors.
#[derive(Debug, Error)]
pub enum MediaError {
    /// File not found.
    #[error("file not found: {0}")]
    NotFound(String),

    /// Invalid file format.
    #[error("invalid file format: {0}")]
    InvalidFormat(String),

    /// File too large.
    #[error("file size ({size} bytes) exceeds maximum ({max} bytes)")]
    TooLarge {
        /// Actual file size.
        size: usize,
        /// Maximum allowed size.
        max: usize,
    },

    /// Invalid URL.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// HTTP error.
    #[error("HTTP error: {0}")]
    Http(#[from] HttpError),
}

/// Result alias for media operations.
pub type MediaResult<T> = Result<T, MediaError>;
