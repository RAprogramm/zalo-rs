// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User types.

use serde::{Deserialize, Serialize};

use crate::error::SdkResult;

/// Gender enumeration.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum Gender {
    /// Male.
    #[serde(rename = "1")]
    Male,
    /// Female.
    #[serde(rename = "2")]
    Female,
    /// Other.
    #[serde(rename = "3")]
    Other,
}

/// Birthday representation.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Birthday {
    /// Day of month (1-31).
    pub day: u8,
    /// Month (1-12).
    pub month: u8,
    /// Year.
    pub year: u16,
}

impl Birthday {
    /// Parses a birthday string in dd/mm/yyyy format.
    ///
    /// # Errors
    ///
    /// Returns error if format is invalid.
    pub fn parse(s: &str) -> SdkResult<Self> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 3 {
            return Err(crate::SdkError::InvalidBirthday(s.to_owned()));
        }

        let day = parts[0]
            .parse::<u8>()
            .map_err(|_| crate::SdkError::InvalidBirthday(s.to_owned()))?;
        let month = parts[1]
            .parse::<u8>()
            .map_err(|_| crate::SdkError::InvalidBirthday(s.to_owned()))?;
        let year = parts[2]
            .parse::<u16>()
            .map_err(|_| crate::SdkError::InvalidBirthday(s.to_owned()))?;

        Ok(Self { day, month, year })
    }
}

/// User information.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct UserInfo {
    /// Unique user identifier.
    pub user_id: String,
    /// Display name.
    pub display_name: String,
    /// Avatar URL.
    pub avatar: String,
    /// Gender.
    pub gender: Gender,
    /// Optional birthday.
    pub birthday: Option<Birthday>,
}

/// Get user info request.
#[derive(Clone, Debug, Serialize)]
pub struct GetUserInfoRequest {
    app_id: String,
}

impl GetUserInfoRequest {
    /// Creates a new get user info request.
    #[must_use]
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

/// Phone number wrapper.
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PhoneNumber {
    /// Phone number string.
    pub number: String,
    /// Verification status.
    pub is_verified: bool,
}

/// Get phone number request.
#[derive(Clone, Debug, Serialize)]
pub struct GetPhoneNumberRequest {
    app_id: String,
}

impl GetPhoneNumberRequest {
    /// Creates a new get phone number request.
    #[must_use]
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

/// Phone number response.
#[derive(Clone, Debug, Deserialize)]
pub struct PhoneNumberResponse {
    /// Phone number data.
    pub phone: PhoneNumber,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn birthday_parse_valid() {
        let bday = Birthday::parse("01/02/1990").unwrap();
        assert_eq!(bday.day, 1);
        assert_eq!(bday.month, 2);
        assert_eq!(bday.year, 1990);
    }

    #[test]
    fn birthday_parse_invalid() {
        assert!(Birthday::parse("invalid").is_err());
    }
}
