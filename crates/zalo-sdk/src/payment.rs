// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// Amount of money expressed in the smallest currency unit (e.g. VND cents).
///
/// Must be a positive non-zero integer.
///
/// # Examples
///
/// ```
/// use zalo_sdk::payment::Amount;
///
/// let amount = Amount::new(50_000)?;
/// assert_eq!(amount.value(), 50_000);
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Amount(u64);

impl Amount {
    /// Creates a validated payment amount.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAmount`] when `value` is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::payment::Amount;
    ///
    /// let amount = Amount::new(10_000)?;
    /// assert_eq!(amount.value(), 10_000);
    ///
    /// assert!(Amount::new(0).is_err());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: u64) -> SdkResult<Self> {
        if value == 0 {
            return Err(SdkError::InvalidAmount);
        }
        Ok(Self(value))
    }

    /// Returns the raw amount value.
    #[must_use]
    pub fn value(self) -> u64 {
        self.0
    }
}

/// An order identifier unique within the merchant's system.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OrderId(String);

impl OrderId {
    /// Creates a validated order identifier.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidOrderId`] when the value is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::payment::OrderId;
    ///
    /// let id = OrderId::new("ORDER-001")?;
    /// assert_eq!(id.as_str(), "ORDER-001");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SdkError::InvalidOrderId(value));
        }
        Ok(Self(value))
    }

    /// Returns the raw identifier string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Checkout request submitted to the Zalo Pay payment gateway.
///
/// # Examples
///
/// ```
/// use zalo_sdk::payment::{Amount, CheckoutRequest, OrderId};
///
/// let req = CheckoutRequest::new("ORDER-1", 150_000, "Test payment")?;
/// assert_eq!(req.description, "Test payment");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct CheckoutRequest {
    /// Merchant-assigned order identifier.
    pub order_id:    String,
    /// Payment amount in smallest currency unit.
    pub amount:      u64,
    /// Human-readable description shown to the user.
    pub description: String
}

impl CheckoutRequest {
    /// Constructs a checkout request after validating all fields.
    ///
    /// # Errors
    ///
    /// Returns errors from [`OrderId::new`], [`Amount::new`], or
    /// [`SdkError::EmptyDescription`] when the description is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::payment::CheckoutRequest;
    ///
    /// let req = CheckoutRequest::new("ORD-42", 99_000, "Coffee order")?;
    /// assert_eq!(req.amount, 99_000);
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        order_id: impl Into<String>,
        amount: u64,
        description: impl Into<String>
    ) -> SdkResult<Self> {
        let order_id = OrderId::new(order_id)?;
        let amount = Amount::new(amount)?;
        let description = description.into();
        if description.trim().is_empty() {
            return Err(SdkError::EmptyDescription);
        }
        Ok(Self {
            order_id: order_id.0,
            amount: amount.0,
            description
        })
    }
}

/// Result of a completed payment transaction.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PaymentStatus {
    /// Payment was completed successfully.
    Success,
    /// Payment was cancelled by the user.
    Cancelled,
    /// Payment failed due to a platform or gateway error.
    Failed
}

/// Response returned after a checkout attempt.
///
/// # Examples
///
/// ```
/// use zalo_sdk::payment::{CheckoutResponse, PaymentStatus};
///
/// let json = r#"{"transaction_id":"txn-001","status":"success"}"#;
/// let resp: CheckoutResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.status, PaymentStatus::Success);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutResponse {
    /// Platform-assigned transaction identifier.
    pub transaction_id: String,
    /// Outcome of the payment attempt.
    pub status:         PaymentStatus
}

impl CheckoutResponse {
    /// Returns `true` when the transaction completed successfully.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::payment::{CheckoutResponse, PaymentStatus};
    ///
    /// let resp = CheckoutResponse {
    ///     transaction_id: "txn-1".to_owned(),
    ///     status:         PaymentStatus::Success
    /// };
    /// assert!(resp.is_success());
    /// ```
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.status == PaymentStatus::Success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amount_rejects_zero() {
        let err = Amount::new(0).expect_err("zero");
        assert!(matches!(err, SdkError::InvalidAmount));
    }

    #[test]
    fn amount_accepts_positive() {
        let a = Amount::new(1).expect("minimum positive");
        assert_eq!(a.value(), 1);
    }

    #[test]
    fn amount_accepts_large_value() {
        let a = Amount::new(u64::MAX).expect("max u64");
        assert_eq!(a.value(), u64::MAX);
    }

    #[test]
    fn order_id_rejects_empty() {
        let err = OrderId::new("").expect_err("empty");
        assert!(matches!(err, SdkError::InvalidOrderId(_)));
    }

    #[test]
    fn order_id_rejects_whitespace() {
        let err = OrderId::new("  ").expect_err("whitespace");
        assert!(matches!(err, SdkError::InvalidOrderId(_)));
    }

    #[test]
    fn order_id_accepts_valid() {
        let id = OrderId::new("ORD-001").expect("valid");
        assert_eq!(id.as_str(), "ORD-001");
    }

    #[test]
    fn checkout_request_rejects_zero_amount() {
        let err = CheckoutRequest::new("ORD", 0, "desc").expect_err("zero amount");
        assert!(matches!(err, SdkError::InvalidAmount));
    }

    #[test]
    fn checkout_request_rejects_empty_description() {
        let err = CheckoutRequest::new("ORD", 1000, "  ").expect_err("empty desc");
        assert!(matches!(err, SdkError::EmptyDescription));
    }

    #[test]
    fn checkout_request_rejects_empty_order_id() {
        let err = CheckoutRequest::new("", 1000, "desc").expect_err("empty order id");
        assert!(matches!(err, SdkError::InvalidOrderId(_)));
    }

    #[test]
    fn checkout_request_builds_valid() {
        let req = CheckoutRequest::new("ORD-42", 50_000, "Coffee").expect("valid");
        assert_eq!(req.order_id, "ORD-42");
        assert_eq!(req.amount, 50_000);
        assert_eq!(req.description, "Coffee");
    }

    #[test]
    fn checkout_response_success_status() {
        let resp = CheckoutResponse {
            transaction_id: "txn-1".to_owned(),
            status:         PaymentStatus::Success
        };
        assert!(resp.is_success());
    }

    #[test]
    fn checkout_response_cancelled_is_not_success() {
        let resp = CheckoutResponse {
            transaction_id: "txn-2".to_owned(),
            status:         PaymentStatus::Cancelled
        };
        assert!(!resp.is_success());
    }

    #[test]
    fn checkout_response_deserialises_from_json() {
        let json = r#"{"transaction_id":"txn-001","status":"success"}"#;
        let resp: CheckoutResponse = serde_json::from_str(json).expect("deserialize");
        assert_eq!(resp.status, PaymentStatus::Success);
    }
}
