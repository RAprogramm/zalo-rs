#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Core primitives for the Zalo bot platform.
//!
//! This crate provides configuration loading, error handling and observability
//! helpers shared across binaries and supporting libraries.

/// Runtime configuration facilities.
pub mod config;
/// Core error types and aliases.
pub mod error;
/// Observability/tracing utilities.
pub mod observability;

pub use config::{AppConfig, ConfigLoader, Environment, LogFormat, LoggingConfig};
pub use error::{ConfigError, CoreError, CoreResult, ObservabilityError};
pub use masterror::{AppError, AppErrorKind, AppResult};
pub use observability::{build_tracing_dispatch, init_tracing};
