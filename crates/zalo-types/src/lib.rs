#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Shared primitives for the Zalo bot platform.
//!
//! The crate contains reusable configuration models and error definitions that
//! can be leveraged by both the bot server and the mini app SDK.

/// Runtime configuration facilities.
pub mod config;
/// Core error types and aliases.
pub mod error;

pub use config::{AppConfig, ConfigLoader, Environment, LogFormat, LoggingConfig};
pub use error::{ConfigError, TypesError, TypesResult};
pub use masterror::{AppError, AppErrorKind, AppResult};
