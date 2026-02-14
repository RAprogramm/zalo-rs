// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User domain types and requests for the Zalo SDK.
//!
//! This module contains strongly-typed domain objects used by the SDK:
//! - AccessToken (zeroized on drop)
//! - UserId
//! - Gender (forward-compatible)
//! - Birthday (validated `dd/mm/yyyy`)
//! - PhoneNumber (normalized)
//! - UserInfo and request/response types

use std::fmt::{Formatter, Result as FmtResult};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

use crate::error::{SdkError, SdkResult};

/// Strongly-typed access token.
/// The inner `String` is zeroized on drop to reduce secret lifetime in memory.
#[repr(transparent)]
#[derive(Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct AccessToken(String);

impl Drop for AccessToken {
    fn drop(&mut self) {
        self.0.zeroize();
    }
}

impl AccessToken {
    /// Create a validated `AccessToken`.
    ///
    /// Returns `SdkError::InvalidAccessToken` if the provided value is empty
    /// or contains only whitespace.
    pub fn new(input: impl AsRef<str>) -> SdkResult<Self> {
        let raw = input.as_ref().trim();
        if raw.is_empty() {
            return Err(SdkError::InvalidAccessToken(raw.to_owned()));
        }
        Ok(Self(raw.to_owned()))
    }

    /// Return token as string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // redact token in debug output to avoid leaking secrets in logs
        f.write_str("AccessToken([REDACTED])")
    }
}

/// Strongly-typed user identifier.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(Box<str>);

impl UserId {
    /// Create validated `UserId`.
    ///
    /// Returns `SdkError::InvalidUserId` if the provided value is empty.
    pub fn new(input: impl AsRef<str>) -> SdkResult<Self> {
        let raw = input.as_ref().trim();
        if raw.is_empty() {
            return Err(SdkError::InvalidUserId(raw.to_owned()));
        }
        Ok(Self(raw.into()))
    }

    /// Return user id as string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Forward-compatible user gender value returned by the platform.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Gender {
    /// Male gender.
    Male,
    /// Female gender.
    Female,
    /// Explicitly reported as unknown.
    Unknown,
    /// Any non-standard or future value returned by the platform.
    /// Preserves the original string for forward compatibility.
    Other(Box<str>)
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let raw: Box<str> = Deserialize::deserialize(d)?;
        Ok(match raw.as_ref() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "unknown" => Gender::Unknown,
            _ => Gender::Other(raw)
        })
    }
}

impl Serialize for Gender {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        match self {
            Gender::Male => s.serialize_str("male"),
            Gender::Female => s.serialize_str("female"),
            Gender::Unknown => s.serialize_str("unknown"),
            Gender::Other(v) => s.serialize_str(v)
        }
    }
}

/// Birthday represented in `dd/mm/yyyy` format.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Birthday(NaiveDate);

impl Birthday {
    const FORMAT: &'static str = "%d/%m/%Y";

    /// Parse birthday from `dd/mm/yyyy`. Returns `InvalidBirthday` on failure.
    pub fn parse(raw: &str) -> SdkResult<Self> {
        let date = NaiveDate::parse_from_str(raw, Self::FORMAT)
            .map_err(|_| SdkError::InvalidBirthday(raw.to_owned()))?;
        Ok(Self(date))
    }

    /// Return underlying `NaiveDate`.
    #[must_use]
    pub fn as_date(&self) -> NaiveDate {
        self.0
    }
}

impl<'de> Deserialize<'de> for Birthday {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let raw: &str = Deserialize::deserialize(d)?;
        Birthday::parse(raw).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Birthday {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        s.serialize_str(&self.0.format(Self::FORMAT).to_string())
    }
}

/// Detailed profile of authenticated user.
///
/// Unknown JSON fields are rejected to avoid silently ignoring API changes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserInfo {
    /// Unique user identifier assigned by the platform.
    pub id:       UserId,
    /// Display name of the user.
    pub name:     Box<str>,
    /// URL pointing to the user's avatar image.
    pub avatar:   Box<str>,
    /// User birthday if provided and successfully parsed.
    #[serde(default)]
    pub birthday: Option<Birthday>,
    /// User gender if provided by the platform.
    #[serde(default)]
    pub gender:   Option<Gender>
}

/// Validated phone number.
///
/// Accepts digits with optional leading `+`. Normalized to canonical form.
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneNumber(Box<str>);

impl PhoneNumber {
    /// Create validated and normalized phone number.
    ///
    /// Returns `SdkError::InvalidPhoneNumber` on invalid input.
    pub fn new(input: impl AsRef<str>) -> SdkResult<Self> {
        let raw = input.as_ref().trim();

        if raw.is_empty() {
            return Err(SdkError::InvalidPhoneNumber(raw.to_owned()));
        }

        let (prefix, digits) = if let Some(rest) = raw.strip_prefix('+') {
            ("+", rest)
        } else {
            ("", raw)
        };

        if digits.is_empty() || !digits.as_bytes().iter().all(u8::is_ascii_digit) {
            return Err(SdkError::InvalidPhoneNumber(raw.to_owned()));
        }

        let mut normalized = String::with_capacity(prefix.len() + digits.len());
        normalized.push_str(prefix);
        normalized.push_str(digits);

        Ok(Self(normalized.into_boxed_str()))
    }

    /// Return phone number as string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Request to fetch authenticated user profile information.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct GetUserInfoRequest {
    /// Validated access token obtained during authorization.
    pub access_token: AccessToken
}

impl GetUserInfoRequest {
    /// Create a new profile request with a validated access token.
    #[must_use]
    pub fn new(access_token: AccessToken) -> Self {
        Self {
            access_token
        }
    }
}

/// Request to fetch the authenticated user's phone number.
#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct GetPhoneNumberRequest {
    /// Validated access token obtained during authorization.
    pub access_token: AccessToken
}

impl GetPhoneNumberRequest {
    /// Create a new phone-number request with a validated access token.
    #[must_use]
    pub fn new(access_token: AccessToken) -> Self {
        Self {
            access_token
        }
    }
}

/// Response returned by the `getPhoneNumber` API.
///
/// Unknown JSON fields are rejected.
#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PhoneNumberResponse {
    /// Raw phone number string returned by the platform.
    pub number: Box<str>
}

impl PhoneNumberResponse {
    /// Parse and validate the returned phone number.
    pub fn parse(&self) -> SdkResult<PhoneNumber> {
        PhoneNumber::new(&self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_validation() {
        assert!(PhoneNumber::new("").is_err());
        assert!(PhoneNumber::new("+").is_err());
        assert!(PhoneNumber::new("abc").is_err());
        assert!(PhoneNumber::new("+84 901").is_err());

        let ok = PhoneNumber::new("+84901234567");
        assert!(ok.is_ok());
    }

    #[test]
    fn access_token_validation() {
        assert!(AccessToken::new("").is_err());
        assert!(AccessToken::new("   ").is_err());
        assert!(AccessToken::new("tok").is_ok());
    }

    #[test]
    fn birthday_validation() {
        assert!(Birthday::parse("01/01/1990").is_ok());
        assert!(Birthday::parse("1990-01-01").is_err());
    }

    #[test]
    fn gender_forward_compat() {
        let g: Result<Gender, _> = serde_json::from_str("\"male\"");
        assert!(matches!(g, Ok(Gender::Male)));

        let g2: Result<Gender, _> = serde_json::from_str("\"nonbinary\"");
        assert!(matches!(g2, Ok(Gender::Other(_))));
    }
}
