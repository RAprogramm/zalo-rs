// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use reqwest::Client;
use tracing::debug;

use crate::error::{HttpError, HttpResult};
use crate::oauth_types::{
    AuthorizationCodeRequest, OAuthTokenResponse, RefreshTokenRequest
};

const OAUTH_URL: &str = "https://oauth.zalo.me/v4/access_token";

/// OAuth 2.0 client for Zalo authentication.
///
/// This client handles the OAuth 2.0 flow for obtaining and refreshing
/// access tokens from the Zalo platform.
///
/// # Examples
///
/// ```rust,no_run
/// use zalo_http::oauth::OAuthClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = OAuthClient::new("APP_ID", "SECRET_KEY", "https://redirect.uri");
///     
///     // Exchange authorization code for access token
///     let tokens = client.get_access_token("AUTH_CODE").await?;
///     println!("Access token: {}", tokens.access_token);
///     
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct OAuthClient {
    inner: Client,
    app_id: String,
    secret_key: String,
    redirect_uri: String
}

impl OAuthClient {
    /// Creates a new OAuth client.
    ///
    /// # Arguments
    ///
    /// * `app_id` - Application ID from Zalo Developers
    /// * `secret_key` - Application secret key
    /// * `redirect_uri` - Redirect URI configured in Zalo app settings
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::oauth::OAuthClient;
    ///
    /// let client = OAuthClient::new("app_id", "secret", "https://example.com/callback");
    /// ```
    #[must_use]
    pub fn new(
        app_id: impl Into<String>,
        secret_key: impl Into<String>,
        redirect_uri: impl Into<String>
    ) -> Self {
        Self {
            inner: Client::new(),
            app_id: app_id.into(),
            secret_key: secret_key.into(),
            redirect_uri: redirect_uri.into()
        }
    }

    /// Exchanges an authorization code for an access token.
    ///
    /// This method should be called after the user authorizes your application
    /// and you receive an authorization code via the redirect URI.
    ///
    /// # Arguments
    ///
    /// * `code` - Authorization code received from Zalo OAuth
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Transport`] on network errors or
    /// [`HttpError::Deserialization`] on invalid response format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::oauth::OAuthClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OAuthClient::new("app", "secret", "https://example.com/cb");
    ///     let tokens = client.get_access_token("auth_code_here").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_access_token(&self, code: impl Into<String>) -> HttpResult<OAuthTokenResponse> {
        let request = AuthorizationCodeRequest::new(
            &self.app_id,
            &self.secret_key,
            code,
            &self.redirect_uri
        );

        debug!("Requesting access token from Zalo OAuth");

        let response = self
            .inner
            .post(OAUTH_URL)
            .json(&request)
            .send()
            .await
            .map_err(HttpError::from)?;

        let token_response: OAuthTokenResponse = response
            .json()
            .await
            .map_err(HttpError::from)?;

        Ok(token_response)
    }

    /// Refreshes an access token using a refresh token.
    ///
    /// Call this method when your access token is about to expire or has
    /// expired. The refresh token is typically obtained along with the
    /// initial access token.
    ///
    /// # Arguments
    ///
    /// * `refresh_token` - Refresh token from previous token response
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Transport`] on network errors or
    /// [`HttpError::Deserialization`] on invalid response format.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::oauth::OAuthClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OAuthClient::new("app", "secret", "https://example.com/cb");
    ///     let new_tokens = client.refresh_token("refresh_token_here").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn refresh_token(
        &self,
        refresh_token: impl Into<String>
    ) -> HttpResult<OAuthTokenResponse> {
        let request = RefreshTokenRequest::new(
            &self.app_id,
            &self.secret_key,
            refresh_token
        );

        debug!("Refreshing access token");

        let response = self
            .inner
            .post(OAUTH_URL)
            .json(&request)
            .send()
            .await
            .map_err(HttpError::from)?;

        let token_response: OAuthTokenResponse = response
            .json()
            .await
            .map_err(HttpError::from)?;

        Ok(token_response)
    }

    /// Returns the configured app ID.
    #[must_use]
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    /// Returns the configured redirect URI.
    #[must_use]
    pub fn redirect_uri(&self) -> &str {
        &self.redirect_uri
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_oauth_client() {
        let client = OAuthClient::new("app123", "secret456", "https://example.com/callback");
        
        assert_eq!(client.app_id(), "app123");
        assert_eq!(client.redirect_uri(), "https://example.com/callback");
    }

    #[test]
    fn authorization_code_request_serializes() {
        let request = AuthorizationCodeRequest::new("app", "secret", "code123", "https://cb");
        let json = serde_json::to_string(&request).expect("serialize");

        assert!(json.contains("\"app_id\":\"app\""));
        assert!(json.contains("\"grant_type\":\"authorization_code\""));
    }

    #[test]
    fn refresh_token_request_serializes() {
        let request = RefreshTokenRequest::new("app", "secret", "refresh123");
        let json = serde_json::to_string(&request).expect("serialize");

        assert!(json.contains("\"grant_type\":\"refresh_token\""));
    }
}
