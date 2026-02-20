// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User API implementation.

pub mod types;

pub use types::*;

/// Gets user information.
#[must_use]
pub fn get_user_info(_request: GetUserInfoRequest) -> Option<UserInfo> {
    // Placeholder - actual implementation depends on platform
    None
}

/// Gets user phone number.
#[must_use]
pub fn get_phone_number(_request: GetPhoneNumberRequest) -> Option<PhoneNumberResponse> {
    // Placeholder - actual implementation depends on platform
    None
}
