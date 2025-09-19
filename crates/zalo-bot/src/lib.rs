#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Zalo Official Account bot utilities.
//!
//! The crate bundles reusable observability helpers and webhook signature
//! verification suitable for building OA bots and webhooks.

/// Error definitions for the bot crate.
pub mod error;
/// Observability helpers wrapping `tracing` initialisation.
pub mod observability;
/// Webhook signature verification helpers.
pub mod webhook;

pub use error::{BotError, BotResult, ObservabilityError, SignatureError};
pub use observability::{build_tracing_dispatch, init_tracing};
pub use webhook::WebhookVerifier;
