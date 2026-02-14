// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

const MAX_KEY_LEN: usize = 128;
const MAX_VALUE_LEN: usize = 4096;

/// A validated storage key.
///
/// Keys must be non-empty and not exceed [`MAX_KEY_LEN`] bytes.
///
/// # Examples
///
/// ```
/// use zalo_sdk::storage::StorageKey;
///
/// let key = StorageKey::new("user_prefs")?;
/// assert_eq!(key.as_str(), "user_prefs");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StorageKey(String);

impl StorageKey {
    /// Creates a validated storage key.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::StorageKeyEmpty`] when the key is blank, or
    /// [`SdkError::StorageKeyTooLong`] when it exceeds the limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::storage::StorageKey;
    ///
    /// let key = StorageKey::new("cart_items")?;
    /// assert_eq!(key.as_str(), "cart_items");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SdkError::StorageKeyEmpty);
        }
        if value.len() > MAX_KEY_LEN {
            return Err(SdkError::StorageKeyTooLong {
                length:  value.len(),
                maximum: MAX_KEY_LEN
            });
        }
        Ok(Self(value))
    }

    /// Returns the raw key string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A validated storage value.
///
/// Values must not exceed [`MAX_VALUE_LEN`] bytes.
///
/// # Examples
///
/// ```
/// use zalo_sdk::storage::StorageValue;
///
/// let val = StorageValue::new("hello")?;
/// assert_eq!(val.as_str(), "hello");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageValue(String);

impl StorageValue {
    /// Creates a validated storage value.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::StorageValueTooLong`] when the value exceeds the
    /// byte limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::storage::StorageValue;
    ///
    /// let val = StorageValue::new("my-value")?;
    /// assert_eq!(val.as_str(), "my-value");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.len() > MAX_VALUE_LEN {
            return Err(SdkError::StorageValueTooLong {
                length:  value.len(),
                maximum: MAX_VALUE_LEN
            });
        }
        Ok(Self(value))
    }

    /// Creates an empty storage value.
    #[must_use]
    pub fn empty() -> Self {
        Self(String::new())
    }

    /// Returns the raw value string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether the value is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Request to store a key–value pair in the mini app sandbox.
#[derive(Clone, Debug, Serialize)]
pub struct SetStorageRequest {
    /// Storage key.
    pub key:   String,
    /// Value to store.
    pub value: String
}

impl SetStorageRequest {
    /// Constructs a write request after validating the key and value.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`StorageKey::new`] and [`StorageValue::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::storage::SetStorageRequest;
    ///
    /// let req = SetStorageRequest::new("theme", "dark")?;
    /// assert_eq!(req.key, "theme");
    /// assert_eq!(req.value, "dark");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> SdkResult<Self> {
        let key = StorageKey::new(key)?;
        let value = StorageValue::new(value)?;
        Ok(Self {
            key:   key.0,
            value: value.0
        })
    }
}

/// Request to retrieve a value from the mini app sandbox.
#[derive(Clone, Debug, Serialize)]
pub struct GetStorageRequest {
    /// Key to look up.
    pub key: String
}

impl GetStorageRequest {
    /// Constructs a read request after validating the key.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`StorageKey::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::storage::GetStorageRequest;
    ///
    /// let req = GetStorageRequest::new("theme")?;
    /// assert_eq!(req.key, "theme");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(key: impl Into<String>) -> SdkResult<Self> {
        let key = StorageKey::new(key)?;
        Ok(Self {
            key: key.0
        })
    }
}

/// Response from a storage read operation.
///
/// # Examples
///
/// ```
/// use zalo_sdk::storage::GetStorageResponse;
///
/// let resp = GetStorageResponse {
///     value: "dark".to_owned()
/// };
/// assert_eq!(resp.value, "dark");
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetStorageResponse {
    /// Value found for the requested key, or an empty string if absent.
    pub value: String
}

/// Request to remove a key from storage.
#[derive(Clone, Debug, Serialize)]
pub struct RemoveStorageRequest {
    /// Key to delete.
    pub key: String
}

impl RemoveStorageRequest {
    /// Constructs a delete request after validating the key.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`StorageKey::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::storage::RemoveStorageRequest;
    ///
    /// let req = RemoveStorageRequest::new("theme")?;
    /// assert_eq!(req.key, "theme");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(key: impl Into<String>) -> SdkResult<Self> {
        let key = StorageKey::new(key)?;
        Ok(Self {
            key: key.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_rejects_empty() {
        let err = StorageKey::new("").expect_err("empty");
        assert!(matches!(err, SdkError::StorageKeyEmpty));
    }

    #[test]
    fn key_rejects_whitespace() {
        let err = StorageKey::new("   ").expect_err("whitespace");
        assert!(matches!(err, SdkError::StorageKeyEmpty));
    }

    #[test]
    fn key_rejects_too_long() {
        let long = "x".repeat(MAX_KEY_LEN + 1);
        let err = StorageKey::new(long).expect_err("too long");
        assert!(matches!(
            err,
            SdkError::StorageKeyTooLong {
                maximum: 128,
                ..
            }
        ));
    }

    #[test]
    fn key_accepts_boundary_length() {
        let exact = "x".repeat(MAX_KEY_LEN);
        StorageKey::new(exact).expect("exact limit");
    }

    #[test]
    fn value_rejects_too_long() {
        let long = "v".repeat(MAX_VALUE_LEN + 1);
        let err = StorageValue::new(long).expect_err("too long");
        assert!(matches!(
            err,
            SdkError::StorageValueTooLong {
                maximum: 4096,
                ..
            }
        ));
    }

    #[test]
    fn value_accepts_empty_string() {
        let val = StorageValue::new("").expect("empty value ok");
        assert!(val.is_empty());
    }

    #[test]
    fn value_empty_constructor() {
        let val = StorageValue::empty();
        assert!(val.is_empty());
    }

    #[test]
    fn set_request_validates_both_fields() {
        let req = SetStorageRequest::new("k", "v").expect("valid");
        assert_eq!(req.key, "k");
        assert_eq!(req.value, "v");
    }

    #[test]
    fn set_request_rejects_empty_key() {
        let err = SetStorageRequest::new("", "v").expect_err("empty key");
        assert!(matches!(err, SdkError::StorageKeyEmpty));
    }

    #[test]
    fn get_request_validates_key() {
        let req = GetStorageRequest::new("my-key").expect("valid");
        assert_eq!(req.key, "my-key");
    }

    #[test]
    fn remove_request_validates_key() {
        let req = RemoveStorageRequest::new("my-key").expect("valid");
        assert_eq!(req.key, "my-key");
    }

    #[test]
    fn get_storage_response_deserialises() {
        let json = r#"{"value":"dark"}"#;
        let resp: GetStorageResponse = serde_json::from_str(json).expect("deserialize");
        assert_eq!(resp.value, "dark");
    }
}
