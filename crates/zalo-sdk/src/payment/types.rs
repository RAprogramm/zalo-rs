// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Payment types for Zalo Pay integration.

use serde::{Deserialize, Serialize};

use crate::error::SdkResult;

/// Order identifier wrapper.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrderId(String);

impl OrderId {
    /// Creates a new order ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Returns the order ID as string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Payment amount in VND.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Amount(u64);

impl Amount {
    /// Creates a new amount.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAmount`] if amount is zero.
    pub fn new(amount: u64) -> SdkResult<Self> {
        if amount == 0 {
            return Err(crate::SdkError::InvalidAmount);
        }
        Ok(Self(amount))
    }

    /// Returns the amount value.
    #[must_use]
    pub fn value(&self) -> u64 {
        self.0
    }
}

/// Payment status.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    /// Payment completed successfully.
    Success,
    /// Payment was cancelled by user.
    Cancelled,
    /// Payment failed.
    Failed,
    /// Payment is pending.
    Pending,
}

/// Checkout request.
#[derive(Clone, Debug, Serialize)]
pub struct CheckoutRequest {
    order_id: String,
    amount: u64,
    description: String,
}

impl CheckoutRequest {
    /// Creates a new checkout request.
    ///
    /// # Errors
    ///
    /// Returns error if order_id or description is empty, or amount is zero.
    pub fn new(
        order_id: impl Into<String>,
        amount: u64,
        description: impl Into<String>,
    ) -> SdkResult<Self> {
        let order_id = order_id.into();
        let description = description.into();

        if order_id.trim().is_empty() {
            return Err(crate::SdkError::InvalidOrderId(order_id));
        }
        if description.trim().is_empty() {
            return Err(crate::SdkError::EmptyDescription);
        }
        if amount == 0 {
            return Err(crate::SdkError::InvalidAmount);
        }

        Ok(Self {
            order_id,
            amount,
            description,
        })
    }
}

/// Checkout response.
#[derive(Clone, Debug, Deserialize)]
pub struct CheckoutResponse {
    /// Payment status.
    pub status: PaymentStatus,
    /// Optional transaction ID if payment was processed.
    pub transaction_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_rejects_zero() {
        assert!(Amount::new(0).is_err());
    }

    #[test]
    fn amount_accepts_positive() {
        let amount = Amount::new(1000).unwrap();
        assert_eq!(amount.value(), 1000);
    }

    #[test]
    fn order_id_accepts_valid() {
        let id = OrderId::new("ORDER_123");
        assert_eq!(id.as_str(), "ORDER_123");
    }
}
