// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Configuration loader.

use std::{
    env,
    path::{Path, PathBuf},
};

use figment::{
    Figment,
    providers::{Env, Format, Serialized, Toml},
};
use serde::{Deserialize, Serialize};

use super::app::AppConfig;
use crate::error::{ConfigError, TypesError, TypesResult};

/// Loads configuration from environment and TOML files.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigLoader {
    env_prefix: String,
    file_path: Option<PathBuf>,
}

impl ConfigLoader {
    /// Creates new loader with prefix.
    #[must_use]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            env_prefix: prefix.into(),
            file_path: None,
        }
    }

    /// Sets config file path.
    #[must_use]
    pub fn with_file_path(mut self, path: impl AsRef<Path>) -> Self {
        self.file_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Returns file path.
    #[must_use]
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Loads configuration.
    pub fn load(&self) -> TypesResult<AppConfig> {
        let mut figment = Figment::from(Serialized::defaults(AppConfig::default()));

        let env_path = env_config_path(&self.env_prefix);
        let resolved_path = env_path.as_deref().or(self.file_path.as_deref());

        if let Some(path) = resolved_path {
            if !path.exists() {
                return Err(ConfigError::MissingFile {
                    path: path.to_path_buf(),
                }
                .into());
            }
            figment = figment.merge(Toml::file(path));
        }

        figment = figment.merge(Env::prefixed(&self.env_prefix).split("__"));

        figment
            .extract::<AppConfig>()
            .map_err(ConfigError::from)
            .map_err(TypesError::from)
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new("ZALO_BOT_")
    }
}

fn env_config_path(prefix: &str) -> Option<PathBuf> {
    let mut key = String::with_capacity(prefix.len() + "CONFIG_PATH".len());
    key.push_str(prefix);
    key.push_str("CONFIG_PATH");

    let value = env::var(&key).ok()?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(PathBuf::from(trimmed))
    }
}
