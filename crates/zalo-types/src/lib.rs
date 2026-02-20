// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![deny(unsafe_code)]
#![deny(missing_docs)]
//! Shared primitives for the Zalo bot platform.
//!
//! The crate contains reusable configuration models and error definitions that
//! can be leveraged by both the bot server and the mini app SDK.

/// Runtime configuration facilities.
pub mod config;
/// Core error types and aliases.
pub mod error;
/// Image message types.
pub mod image;
/// Message types.
pub mod message;
/// Generic API response types.
pub mod response;
/// User-related types.
pub mod user;

pub use config::{AppConfig, ConfigLoader, Environment, LogFormat, LoggingConfig};
pub use error::{ConfigError, TypesError, TypesResult};
pub use image::{ImageAttachment, ImagePayload, SendImageRequest};
pub use masterror::{AppError, AppErrorKind, AppResult};
pub use message::{MessageType, Recipient, SendTextRequest, SendMessageResponse, TextPayload};
pub use response::ApiResponse;
pub use user::{FollowerList, FollowerListQuery, UserProfile};
