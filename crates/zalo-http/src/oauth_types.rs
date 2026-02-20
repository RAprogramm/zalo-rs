// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// OAuth token response from Zalo API.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OAuthTokenResponse {
    /// Access token for API requests.
    pub access_token: String,
    /// Refresh token for obtaining new access tokens.
    pub refresh_token: String,
    /// Token lifetime in seconds.
    pub expires_in: u64,
}

/// OAuth error response.
#[derive(Clone, Debug, Deserialize)]
pub struct OAuthErrorResponse {
    /// Error code.
    pub error: String,
    /// Human-readable error description.
    pub error_description: Option<String>,
}

/// OAuth authorization code request.
#[derive(Clone, Debug, Serialize)]
pub struct AuthorizationCodeRequest {
    /// Application ID.
    pub app_id: String,
    /// Application secret key.
    pub secret_key: String,
    /// Authorization code from OAuth flow.
    pub code: String,
    /// Redirect URI used in authorization request.
    pub redirect_uri: String,
    /// Grant type (always "authorization_code").
    pub grant_type: String,
}

impl AuthorizationCodeRequest {
    /// Creates a new authorization code request.
    #[must_use]
    pub fn new(
        app_id: impl Into<String>,
        secret_key: impl Into<String>,
        code: impl Into<String>,
        redirect_uri: impl Into<String>
    ) -> Self {
        Self {
            app_id: app_id.into(),
            secret_key: secret_key.into(),
            code: code.into(),
            redirect_uri: redirect_uri.into(),
            grant_type: "authorization_code".to_owned()
        }
    }
}

/// OAuth refresh token request.
#[derive(Clone, Debug, Serialize)]
pub struct RefreshTokenRequest {
    /// Application ID.
    pub app_id: String,
    /// Application secret key.
    pub secret_key: String,
    /// Refresh token to use.
    pub refresh_token: String,
    /// Grant type (always "refresh_token").
    pub grant_type: String,
}

impl RefreshTokenRequest {
    /// Creates a new refresh token request.
    #[must_use]
    pub fn new(
        app_id: impl Into<String>,
        secret_key: impl Into<String>,
        refresh_token: impl Into<String>
    ) -> Self {
        Self {
            app_id: app_id.into(),
            secret_key: secret_key.into(),
            refresh_token: refresh_token.into(),
            grant_type: "refresh_token".to_owned()
        }
    }
}
