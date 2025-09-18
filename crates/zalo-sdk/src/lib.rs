#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! WASM-friendly SDK utilities for Zalo mini apps.
//!
//! The crate exposes lightweight helpers for managing runtime context and
//! preparing handshake payloads.

/// Context management primitives for the mini app runtime.
pub mod context;
/// Error types exposed by the SDK.
pub mod error;

pub use context::{HandshakePayload, MiniAppContext};
pub use error::{SdkError, SdkResult};
