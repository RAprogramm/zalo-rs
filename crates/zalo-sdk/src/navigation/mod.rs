// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Navigation API implementation.

pub mod types;

pub use types::*;

use crate::SdkResult;

/// Navigates to a route within the Mini App.
pub fn navigate(_request: NavigateRequest) {
    // Placeholder - actual implementation depends on platform
}

/// Opens a webview with the specified URL.
pub fn open_webview(_request: OpenWebviewRequest) {
    // Placeholder - actual implementation depends on platform
}

/// Sets the navigation bar title.
///
/// # Errors
///
/// Returns error if title validation fails.
pub fn set_title(_request: SetTitleRequest) -> SdkResult<()> {
    // Placeholder - actual implementation depends on platform
    Ok(())
}

/// Closes the Mini App.
pub fn close_app() {
    // Placeholder - actual implementation depends on platform
}
