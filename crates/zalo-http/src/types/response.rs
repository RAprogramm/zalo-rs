// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Common response types.

use serde::Deserialize;

/// API response envelope.
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// Error code (0 = success).
    pub error: i64,
    /// Message.
    pub message: String,
    /// Data.
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// Returns true if success.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.error == 0
    }
}
