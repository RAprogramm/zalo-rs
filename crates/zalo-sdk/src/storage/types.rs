// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Storage types for Mini App key-value storage.

use serde::{Deserialize, Serialize};

use crate::error::SdkResult;

const MAX_KEY_LEN: usize = 128;
const MAX_VALUE_LEN: usize = 4096;

/// A validated storage key.
///
/// Keys must be non-empty and not exceed [`MAX_KEY_LEN`] bytes.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StorageKey(String);

impl StorageKey {
    /// Creates a validated storage key.
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(crate::SdkError::StorageKeyEmpty);
        }
        if value.len() > MAX_KEY_LEN {
            return Err(crate::SdkError::StorageKeyTooLong {
                length: value.len(),
                maximum: MAX_KEY_LEN,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageValue(String);

impl StorageValue {
    /// Creates a validated storage value.
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.len() > MAX_VALUE_LEN {
            return Err(crate::SdkError::StorageValueTooLong {
                length: value.len(),
                maximum: MAX_VALUE_LEN,
            });
        }
        Ok(Self(value))
    }

    /// Returns the raw value string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Request to get a value from storage.
#[derive(Clone, Debug, Serialize)]
pub struct GetStorageRequest {
    key: String,
}

impl GetStorageRequest {
    /// Creates a new get storage request.
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }

    /// Returns the storage key.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }
}

/// Response from get storage operation.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct GetStorageResponse {
    /// Storage key.
    pub key: String,
    /// Optional value if found.
    pub value: Option<String>,
}

/// Request to set a value in storage.
#[derive(Clone, Debug, Serialize)]
pub struct SetStorageRequest {
    key: String,
    value: String,
}

impl SetStorageRequest {
    /// Creates a new set storage request.
    #[must_use]
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

/// Request to remove a value from storage.
#[derive(Clone, Debug, Serialize)]
pub struct RemoveStorageRequest {
    key: String,
}

impl RemoveStorageRequest {
    /// Creates a new remove storage request.
    #[must_use]
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_accepts_valid() {
        let key = StorageKey::new("valid_key").unwrap();
        assert_eq!(key.as_str(), "valid_key");
    }

    #[test]
    fn key_rejects_empty() {
        assert!(StorageKey::new("").is_err());
    }

    #[test]
    fn value_accepts_valid() {
        let val = StorageValue::new("value").unwrap();
        assert_eq!(val.as_str(), "value");
    }
}
