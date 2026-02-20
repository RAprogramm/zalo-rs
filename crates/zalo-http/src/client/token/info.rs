// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Token information and lifecycle.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::oauth_types::OAuthTokenResponse;

use super::secure::SecureToken;

/// Token info with expiration.
#[derive(Clone, Debug)]
pub struct AccessTokenInfo {
    /// Access token.
    pub access_token: SecureToken,
    /// Refresh token.
    pub refresh_token: SecureToken,
    /// Lifetime in seconds.
    pub expires_in: u64,
    /// Expiration timestamp.
    pub expires_at: Option<u64>,
}

impl AccessTokenInfo {
    /// Creates from OAuth response.
    #[must_use]
    pub fn from_response(response: &OAuthTokenResponse) -> Self {
        let expires_at = Some(calculate_expires_at(response.expires_in));
        Self {
            access_token: SecureToken::new(&response.access_token),
            refresh_token: SecureToken::new(&response.refresh_token),
            expires_in: response.expires_in,
            expires_at,
        }
    }

    /// Creates empty info.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            access_token: SecureToken::new(String::new()),
            refresh_token: SecureToken::new(String::new()),
            expires_in: 0,
            expires_at: None,
        }
    }

    /// Returns true if expired or expires within buffer.
    pub async fn is_expired(&self, buffer_seconds: u64) -> bool {
        match self.expires_at {
            Some(expires_at) => {
                let now = current_timestamp();
                now + buffer_seconds >= expires_at
            }
            None => true,
        }
    }

    /// Returns access token value.
    pub async fn access_token(&self) -> String {
        self.access_token.get().await
    }

    /// Returns refresh token value.
    pub async fn refresh_token(&self) -> String {
        self.refresh_token.get().await
    }
}

fn calculate_expires_at(expires_in: u64) -> u64 {
    current_timestamp() + expires_in
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
