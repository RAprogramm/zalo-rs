// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Secure token storage.

use std::sync::Arc;

use tokio::sync::RwLock;
use zeroize::Zeroize;

/// Securely stored token that zeroes memory on drop.
#[derive(Clone)]
pub struct SecureToken {
    inner: Arc<RwLock<String>>,
}

impl SecureToken {
    /// Creates new secure token.
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            inner: Arc::new(RwLock::new(token.into())),
        }
    }

    /// Returns token value.
    pub async fn get(&self) -> String {
        self.inner.read().await.clone()
    }

    /// Updates token value.
    pub async fn set(&self, new_token: impl Into<String>) {
        let mut inner = self.inner.write().await;
        inner.zeroize();
        *inner = new_token.into();
    }

    /// Returns true if empty.
    pub async fn is_empty(&self) -> bool {
        self.inner.read().await.is_empty()
    }
}

impl std::fmt::Debug for SecureToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SecureToken").finish_non_exhaustive()
    }
}
