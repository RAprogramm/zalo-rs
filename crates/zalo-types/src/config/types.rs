// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Configuration types.

use serde::{Deserialize, Serialize};

/// Deployment environment.
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    /// Local development.
    #[default]
    Development,
    /// Staging/testing.
    Staging,
    /// Production.
    Production,
}

impl Environment {
    /// Returns canonical string representation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Development => "development",
            Self::Staging => "staging",
            Self::Production => "production",
        }
    }
}

/// Logging configuration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoggingConfig {
    filter: String,
    format: LogFormat,
}

impl LoggingConfig {
    /// Creates new logging config.
    #[must_use]
    pub fn new(filter: impl Into<String>, format: LogFormat) -> Self {
        Self {
            filter: filter.into(),
            format,
        }
    }

    /// Returns filter expression.
    #[must_use]
    pub fn filter(&self) -> &str {
        &self.filter
    }

    /// Returns log format.
    #[must_use]
    pub fn format(&self) -> LogFormat {
        self.format
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            filter: "info".to_owned(),
            format: LogFormat::Text,
        }
    }
}

/// Log output format.
#[derive(Default, Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Plain text.
    #[default]
    Text,
    /// Structured JSON.
    Json,
}
