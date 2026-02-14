// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// Detailed profile of the authenticated Zalo user.
///
/// Returned by the platform after a successful `getUserInfo` call. All fields
/// are populated when the user has granted the `scope.userInfo` permission.
///
/// # Examples
///
/// ```
/// use zalo_sdk::user::UserInfo;
///
/// let json = r#"{"id":"u1","name":"Alice","avatar":"https://a.example.com/a.jpg","birthday":"01/01/1990","gender":"male"}"#;
/// let info: UserInfo = serde_json::from_str(json).unwrap();
/// assert_eq!(info.name, "Alice");
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UserInfo {
    /// Unique Zalo user identifier.
    pub id:       String,
    /// Display name of the user.
    pub name:     String,
    /// URL to the user's avatar image.
    pub avatar:   String,
    /// Date of birth in `dd/mm/yyyy` format, if available.
    #[serde(default)]
    pub birthday: Option<String>,
    /// Self-reported gender: `"male"`, `"female"`, or `"unknown"`.
    #[serde(default)]
    pub gender:   Option<String>
}

/// Validated phone number retrieved from the platform.
///
/// Obtaining a phone number requires the `scope.userPhonenumber` permission and
/// an explicit user approval prompt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Creates a validated phone number.
    ///
    /// The value must be non-empty and consist only of digits and an optional
    /// leading `+`.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidPhoneNumber`] when the value fails
    /// validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::user::PhoneNumber;
    ///
    /// let phone = PhoneNumber::new("+84901234567")?;
    /// assert_eq!(phone.as_str(), "+84901234567");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(SdkError::InvalidPhoneNumber(value));
        }

        let digit_chars = if let Some(stripped) = trimmed.strip_prefix('+') {
            stripped
        } else {
            trimmed
        };

        if digit_chars.is_empty() || !digit_chars.chars().all(|c| c.is_ascii_digit()) {
            return Err(SdkError::InvalidPhoneNumber(value));
        }

        Ok(Self(trimmed.to_owned()))
    }

    /// Returns the phone number string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Request to fetch the current user's profile.
///
/// Requires that `scope.userInfo` was granted during authorization.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GetUserInfoRequest {
    /// Access token obtained after authorization.
    pub access_token: String
}

impl GetUserInfoRequest {
    /// Constructs a new request with the provided access token.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAccessToken`] when the token is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::user::GetUserInfoRequest;
    ///
    /// let req = GetUserInfoRequest::new("token-abc")?;
    /// assert_eq!(req.access_token, "token-abc");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(access_token: impl Into<String>) -> SdkResult<Self> {
        let token = access_token.into();
        if token.trim().is_empty() {
            return Err(SdkError::InvalidAccessToken(token));
        }
        Ok(Self {
            access_token: token
        })
    }
}

/// Request to retrieve the phone number of the current user.
///
/// Requires that `scope.userPhonenumber` was granted.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GetPhoneNumberRequest {
    /// Access token obtained after authorization.
    pub access_token: String
}

impl GetPhoneNumberRequest {
    /// Constructs a new phone number request.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAccessToken`] when the token is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::user::GetPhoneNumberRequest;
    ///
    /// let req = GetPhoneNumberRequest::new("tok")?;
    /// assert_eq!(req.access_token, "tok");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(access_token: impl Into<String>) -> SdkResult<Self> {
        let token = access_token.into();
        if token.trim().is_empty() {
            return Err(SdkError::InvalidAccessToken(token));
        }
        Ok(Self {
            access_token: token
        })
    }
}

/// Response returned by the `getPhoneNumber` API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct PhoneNumberResponse {
    /// Raw phone number string from the platform.
    pub number: String
}

impl PhoneNumberResponse {
    /// Parses the raw number into a validated [`PhoneNumber`].
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidPhoneNumber`] if the value does not pass
    /// validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::user::PhoneNumberResponse;
    ///
    /// let resp = PhoneNumberResponse {
    ///     number: "+84901234567".to_owned()
    /// };
    /// let phone = resp.parse()?;
    /// assert_eq!(phone.as_str(), "+84901234567");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse(&self) -> SdkResult<PhoneNumber> {
        PhoneNumber::new(&self.number)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_rejects_empty() {
        let err = PhoneNumber::new("").expect_err("empty");
        assert!(matches!(err, SdkError::InvalidPhoneNumber(_)));
    }

    #[test]
    fn phone_rejects_letters() {
        let err = PhoneNumber::new("abc").expect_err("letters");
        assert!(matches!(err, SdkError::InvalidPhoneNumber(_)));
    }

    #[test]
    fn phone_rejects_plus_only() {
        let err = PhoneNumber::new("+").expect_err("plus only");
        assert!(matches!(err, SdkError::InvalidPhoneNumber(_)));
    }

    #[test]
    fn phone_accepts_digits() {
        let phone = PhoneNumber::new("0901234567").expect("digits only");
        assert_eq!(phone.as_str(), "0901234567");
    }

    #[test]
    fn phone_accepts_international_format() {
        let phone = PhoneNumber::new("+84901234567").expect("international");
        assert_eq!(phone.as_str(), "+84901234567");
    }

    #[test]
    fn phone_response_parses_valid_number() {
        let resp = PhoneNumberResponse {
            number: "+84901234567".to_owned()
        };
        let phone = resp.parse().expect("parse");
        assert_eq!(phone.as_str(), "+84901234567");
    }

    #[test]
    fn user_info_deserialises_all_fields() {
        let json = r#"{"id":"u1","name":"Alice","avatar":"https://a.example.com/a.jpg","birthday":"01/01/1990","gender":"female"}"#;
        let info: UserInfo = serde_json::from_str(json).expect("deserialize");
        assert_eq!(info.id, "u1");
        assert_eq!(info.name, "Alice");
        assert_eq!(info.birthday.as_deref(), Some("01/01/1990"));
        assert_eq!(info.gender.as_deref(), Some("female"));
    }

    #[test]
    fn user_info_optional_fields_default_to_none() {
        let json = r#"{"id":"u2","name":"Bob","avatar":"url"}"#;
        let info: UserInfo = serde_json::from_str(json).expect("deserialize");
        assert!(info.birthday.is_none());
        assert!(info.gender.is_none());
    }

    #[test]
    fn get_user_info_request_rejects_empty_token() {
        let err = GetUserInfoRequest::new("").expect_err("empty token");
        assert!(matches!(err, SdkError::InvalidAccessToken(_)));
    }

    #[test]
    fn get_user_info_request_accepts_valid_token() {
        let req = GetUserInfoRequest::new("tok-abc").expect("valid");
        assert_eq!(req.access_token, "tok-abc");
    }

    #[test]
    fn get_phone_number_request_rejects_empty_token() {
        let err = GetPhoneNumberRequest::new("   ").expect_err("whitespace");
        assert!(matches!(err, SdkError::InvalidAccessToken(_)));
    }
}
