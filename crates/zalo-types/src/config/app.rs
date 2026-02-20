// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Application configuration.

use serde::{Deserialize, Serialize};

use super::types::{Environment, LoggingConfig};

/// Application configuration.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct AppConfig {
    environment: Environment,
    logging: LoggingConfig,
}

impl AppConfig {
    /// Returns deployment environment.
    #[must_use]
    pub fn environment(&self) -> Environment {
        self.environment
    }

    /// Returns logging config.
    #[must_use]
    pub fn logging(&self) -> &LoggingConfig {
        &self.logging
    }

    /// Creates copy with new environment.
    #[must_use]
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Creates copy with new logging.
    #[must_use]
    pub fn with_logging(mut self, logging: LoggingConfig) -> Self {
        self.logging = logging;
        self
    }
}
