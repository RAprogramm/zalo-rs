// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! WASM-friendly SDK utilities for Zalo mini apps.
//!
//! The crate exposes a complete set of typed primitives covering the full
//! lifecycle of a Zalo Mini App: authentication, user data retrieval,
//! geolocation, key-value storage, payments, in-app navigation, social
//! sharing, and lifecycle event handling.
//!
//! # Quick overview
//!
//! ```
//! use zalo_sdk::{
//!     auth::{AuthCode, AuthorizeRequest},
//!     lifecycle::AppLifecycleEvent,
//!     navigation::NavigateRequest,
//!     payment::CheckoutRequest,
//!     share::ShareRequest,
//!     storage::{GetStorageRequest, SetStorageRequest},
//!     user::{GetUserInfoRequest, UserInfo}
//! };
//! ```

/// OAuth authorization flow types.
pub mod auth;

/// Mini app runtime context and handshake primitives.
pub mod context;

/// Error types exposed by the SDK.
pub mod error;

/// Geolocation access types.
pub mod location;

/// App lifecycle event model.
pub mod lifecycle;

/// In-app navigation and web view primitives.
pub mod navigation;

/// Zalo Pay checkout and payment types.
pub mod payment;

/// Social sharing request and response types.
pub mod share;

/// Key-value sandbox storage types.
pub mod storage;

/// User profile and phone number retrieval types.
pub mod user;

pub use auth::{AccessToken, AuthCode, AuthorizeRequest, AuthorizeResponse};
pub use context::{HandshakePayload, MiniAppContext};
pub use error::{SdkError, SdkResult};
pub use lifecycle::{AppLifecycleEvent, LifecyclePayload};
pub use location::{Coordinates, GetLocationRequest, LocationAccuracy, LocationResponse};
pub use navigation::{NavigateRequest, OpenWebviewRequest, RoutePath, SetTitleRequest};
pub use payment::{Amount, CheckoutRequest, CheckoutResponse, OrderId, PaymentStatus};
pub use share::{ShareRequest, ShareResponse, ShareResult};
pub use storage::{
    GetStorageRequest, GetStorageResponse, RemoveStorageRequest, SetStorageRequest, StorageKey,
    StorageValue
};
pub use user::{
    GetPhoneNumberRequest, GetUserInfoRequest, PhoneNumber, PhoneNumberResponse, UserInfo
};
