// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Token manager module.

mod info;
mod secure;

pub use info::AccessTokenInfo;
pub use secure::SecureToken;

use std::sync::Arc;

use tokio::sync::RwLock;

use crate::error::HttpResult;
use crate::oauth::OAuthClient;
use crate::oauth_types::OAuthTokenResponse;

/// Manages OAuth tokens with auto-refresh.
#[derive(Clone)]
pub struct TokenManager {
    tokens: Arc<RwLock<AccessTokenInfo>>,
    oauth_client: OAuthClient,
    refresh_buffer: u64,
}

impl TokenManager {
    /// Creates new manager.
    #[must_use]
    pub fn new(oauth_client: OAuthClient) -> Self {
        Self {
            tokens: Arc::new(RwLock::new(AccessTokenInfo::empty())),
            oauth_client,
            refresh_buffer: 300,
        }
    }

    /// Creates with custom buffer.
    #[must_use]
    pub fn with_buffer(oauth_client: OAuthClient, refresh_buffer: u64) -> Self {
        Self {
            tokens: Arc::new(RwLock::new(AccessTokenInfo::empty())),
            oauth_client,
            refresh_buffer,
        }
    }

    /// Initializes with auth code.
    pub async fn initialize_with_code(&self, code: impl Into<String>) -> HttpResult<()> {
        let response = self.oauth_client.get_access_token(code).await?;
        let token_info = AccessTokenInfo::from_response(&response);
        *self.tokens.write().await = token_info;
        Ok(())
    }

    /// Initializes with existing tokens.
    pub async fn initialize_with_tokens(
        &self,
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
        expires_in: u64,
    ) {
        let response = OAuthTokenResponse {
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            expires_in,
        };
        let token_info = AccessTokenInfo::from_response(&response);
        *self.tokens.write().await = token_info;
    }

    /// Gets valid token, refreshing if needed.
    pub async fn get_valid_token(&self) -> HttpResult<String> {
        let needs_refresh = {
            let tokens = self.tokens.read().await;
            tokens.is_expired(self.refresh_buffer).await
        };

        if needs_refresh {
            self.refresh_tokens().await?;
        }

        let tokens = self.tokens.read().await;
        Ok(tokens.access_token().await)
    }

    /// Gets current token without refresh check.
    pub async fn get_current_token(&self) -> String {
        let tokens = self.tokens.read().await;
        tokens.access_token().await
    }

    /// Gets refresh token.
    #[must_use]
    pub async fn get_refresh_token(&self) -> String {
        let tokens = self.tokens.read().await;
        tokens.refresh_token().await
    }

    /// Returns true if valid token available.
    pub async fn has_valid_token(&self) -> bool {
        let tokens = self.tokens.read().await;
        !tokens.is_expired(self.refresh_buffer).await
    }

    /// Manually refreshes tokens.
    pub async fn refresh_tokens(&self) -> HttpResult<()> {
        let refresh_token = {
            let tokens = self.tokens.read().await;
            tokens.refresh_token().await
        };

        if refresh_token.is_empty() {
            return Err(crate::error::HttpError::configuration(
                "no refresh token available",
            ));
        }

        let response = self.oauth_client.refresh_token(&refresh_token).await?;
        let token_info = AccessTokenInfo::from_response(&response);
        *self.tokens.write().await = token_info;
        Ok(())
    }

    /// Returns OAuth client.
    #[must_use]
    pub fn oauth_client(&self) -> &OAuthClient {
        &self.oauth_client
    }
}
