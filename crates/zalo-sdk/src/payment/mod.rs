// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Payment API implementation.

pub mod types;

pub use types::*;

/// Initiates a checkout payment.
///
/// # Errors
///
/// Returns error if request validation fails.
pub fn checkout(_request: CheckoutRequest) -> crate::SdkResult<CheckoutResponse> {
    // Placeholder - actual implementation depends on platform
    Ok(CheckoutResponse {
        status: PaymentStatus::Pending,
        transaction_id: None,
    })
}
