// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Async HTTP client for the Zalo Official Account API.
//!
//! Provides a typed, async wrapper around the Zalo OA v3 REST API, including
//! message delivery, subscriber management, and media uploads.
//!
//! # Quick start
//!
//! ```rust,no_run
//! use zalo_http::client::OaClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = OaClient::new("YOUR_ACCESS_TOKEN")?;
//!     let msg_id = client
//!         .send_text_message("USER_ID", "Hello from Rust!")
//!         .await?;
//!     println!("sent: {msg_id}");
//!     Ok(())
//! }
//! ```

/// Typed HTTP client for the Zalo OA API.
mod client_inner;
pub mod client;
/// Error types for the HTTP layer.
pub mod error;
/// OAuth 2.0 client for Zalo authentication.
pub mod oauth;
/// OAuth request and response types.
pub mod oauth_types;
/// API request and response models.
pub mod types;

/// Rate limiter for API requests.
pub mod rate_limiter;

pub use client::OaClient;
pub use client::token::{AccessTokenInfo, SecureToken, TokenManager};
pub use error::{HttpError, HttpResult};
pub use oauth::OAuthClient;
pub use oauth_types::OAuthTokenResponse;
pub use rate_limiter::RateLimiter;
