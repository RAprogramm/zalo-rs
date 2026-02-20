// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Storage API implementation.

pub mod types;

pub use types::*;

/// Gets a value from storage.
#[must_use]
pub fn get_storage(request: GetStorageRequest) -> GetStorageResponse {
    // Placeholder - actual implementation depends on platform
    GetStorageResponse {
        key: request.key().to_owned(),
        value: None,
    }
}

/// Sets a value in storage.
///
/// # Errors
///
/// Returns error if key or value validation fails.
pub fn set_storage(_request: SetStorageRequest) -> crate::SdkResult<()> {
    // Placeholder - actual implementation depends on platform
    Ok(())
}

/// Removes a value from storage.
#[must_use]
pub fn remove_storage(_request: RemoveStorageRequest) {
    // Placeholder - actual implementation depends on platform
}
