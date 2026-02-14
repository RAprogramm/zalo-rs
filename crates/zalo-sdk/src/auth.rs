// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// OAuth authorization code received after the user grants permission.
///
/// Wraps the raw string code returned by the Zalo platform after a successful
/// authorization flow. The code is single-use and expires quickly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthCode(String);

impl AuthCode {
    /// Creates a new authorization code after validating it is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAuthCode`] when the value is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::auth::AuthCode;
    ///
    /// let code = AuthCode::new("abc123")?;
    /// assert_eq!(code.as_str(), "abc123");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SdkError::InvalidAuthCode(value));
        }
        Ok(Self(value))
    }

    /// Returns the raw authorization code string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Access token granting the mini app permission to call protected APIs.
///
/// Tokens are short-lived. Always obtain a fresh one via [`AuthorizeRequest`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessToken(String);

impl AccessToken {
    /// Creates a new access token after validating it is non-empty.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAccessToken`] when the value is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::auth::AccessToken;
    ///
    /// let token = AccessToken::new("tok-xyz")?;
    /// assert_eq!(token.as_str(), "tok-xyz");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SdkError::InvalidAccessToken(value));
        }
        Ok(Self(value))
    }

    /// Returns the raw token string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Parameters required to initiate the Zalo OAuth authorization flow.
///
/// After the user approves the request, the platform returns an [`AuthCode`]
/// that can be exchanged for an [`AccessToken`] on the server side.
///
/// # Examples
///
/// ```
/// use zalo_sdk::auth::AuthorizeRequest;
///
/// let request = AuthorizeRequest::new("app-id", ["scope.userInfo", "scope.userPhonenumber"])?;
/// assert_eq!(request.app_id(), "app-id");
/// assert_eq!(
///     request.scopes(),
///     &["scope.userInfo", "scope.userPhonenumber"]
/// );
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AuthorizeRequest {
    app_id: String,
    scopes: Vec<String>
}

impl AuthorizeRequest {
    /// Constructs a new authorization request.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidAppId`] when `app_id` is blank, or
    /// [`SdkError::EmptyScopes`] when no scopes are provided.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::auth::AuthorizeRequest;
    ///
    /// let req = AuthorizeRequest::new("my-app", ["scope.userInfo"])?;
    /// assert_eq!(req.app_id(), "my-app");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new<S, I>(app_id: impl Into<String>, scopes: I) -> SdkResult<Self>
    where
        S: Into<String>,
        I: IntoIterator<Item = S>
    {
        let app_id = app_id.into();
        if app_id.trim().is_empty() {
            return Err(SdkError::InvalidAppId(app_id));
        }

        let scopes: Vec<String> = scopes.into_iter().map(Into::into).collect();
        if scopes.is_empty() {
            return Err(SdkError::EmptyScopes);
        }

        Ok(Self {
            app_id,
            scopes
        })
    }

    /// Returns the application identifier.
    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Returns the list of OAuth scopes requested.
    #[must_use]
    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }
}

/// Response returned by the Zalo authorization endpoint.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AuthorizeResponse {
    /// Authorization code to be exchanged for an access token.
    pub code:    String,
    /// Unique Zalo user identifier for the authorizing user.
    pub user_id: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_code_rejects_empty() {
        let err = AuthCode::new("").expect_err("empty");
        assert!(matches!(err, SdkError::InvalidAuthCode(_)));
    }

    #[test]
    fn auth_code_rejects_whitespace() {
        let err = AuthCode::new("  ").expect_err("whitespace");
        assert!(matches!(err, SdkError::InvalidAuthCode(_)));
    }

    #[test]
    fn auth_code_accepts_valid() {
        let code = AuthCode::new("code-abc").expect("valid");
        assert_eq!(code.as_str(), "code-abc");
    }

    #[test]
    fn access_token_rejects_empty() {
        let err = AccessToken::new("").expect_err("empty");
        assert!(matches!(err, SdkError::InvalidAccessToken(_)));
    }

    #[test]
    fn access_token_rejects_whitespace() {
        let err = AccessToken::new("   ").expect_err("whitespace");
        assert!(matches!(err, SdkError::InvalidAccessToken(_)));
    }

    #[test]
    fn access_token_accepts_valid() {
        let tok = AccessToken::new("tok-xyz").expect("valid");
        assert_eq!(tok.as_str(), "tok-xyz");
    }

    #[test]
    fn authorize_request_rejects_empty_app_id() {
        let err = AuthorizeRequest::new("", ["scope.userInfo"]).expect_err("empty app id");
        assert!(matches!(err, SdkError::InvalidAppId(_)));
    }

    #[test]
    fn authorize_request_rejects_empty_scopes() {
        let err = AuthorizeRequest::new("app", Vec::<&str>::new()).expect_err("no scopes");
        assert!(matches!(err, SdkError::EmptyScopes));
    }

    #[test]
    fn authorize_request_stores_all_scopes() {
        let req = AuthorizeRequest::new("app", ["scope.userInfo", "scope.userPhonenumber"])
            .expect("valid");
        assert_eq!(req.scopes().len(), 2);
        assert_eq!(req.scopes()[0], "scope.userInfo");
    }

    #[test]
    fn authorize_request_serialises_correctly() {
        let req = AuthorizeRequest::new("my-app", ["scope.userInfo"]).expect("valid");
        let json = serde_json::to_value(&req).expect("serialize");
        assert_eq!(json["app_id"], "my-app");
        assert_eq!(json["scopes"][0], "scope.userInfo");
    }
}
