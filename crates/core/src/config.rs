use std::path::{Path, PathBuf};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::error::{ConfigError, CoreError, CoreResult};

/// Application-level configuration contract.
///
/// The structure is intentionally small and focused on bootstrap concerns.
///
/// # Examples
///
/// ```
/// use zalo_core::{AppConfig, Environment};
///
/// let config = AppConfig::default();
/// assert_eq!(config.environment(), Environment::Development);
/// ```
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct AppConfig {
    environment: Environment,
    logging: LoggingConfig,
}

impl AppConfig {
    /// Returns the configured deployment environment.
    #[must_use]
    pub fn environment(&self) -> Environment {
        self.environment
    }

    /// Returns the logging configuration block.
    #[must_use]
    pub fn logging(&self) -> &LoggingConfig {
        &self.logging
    }

    /// Creates a copy of the configuration with the provided environment.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::{AppConfig, Environment};
    ///
    /// let production = AppConfig::default().with_environment(Environment::Production);
    /// assert_eq!(production.environment(), Environment::Production);
    /// ```
    #[must_use]
    pub fn with_environment(mut self, environment: Environment) -> Self {
        self.environment = environment;
        self
    }

    /// Creates a copy of the configuration with custom logging settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::{AppConfig, LogFormat, LoggingConfig};
    ///
    /// let logging = LoggingConfig::new("debug", LogFormat::Json);
    /// let config = AppConfig::default().with_logging(logging);
    /// assert_eq!(config.logging().filter(), "debug");
    /// ```
    #[must_use]
    pub fn with_logging(mut self, logging: LoggingConfig) -> Self {
        self.logging = logging;
        self
    }
}

/// Deployment environment the service operates in.
///
/// # Examples
///
/// ```
/// use zalo_core::Environment;
///
/// assert_eq!(Environment::Production.as_str(), "production");
/// ```
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Environment {
    /// Local development (default).
    #[default]
    Development,
    /// Internal staging/testing environment.
    Staging,
    /// Production deployment.
    Production,
}

impl Environment {
    /// Returns the canonical string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::Environment;
    ///
    /// assert_eq!(Environment::Staging.as_str(), "staging");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

/// Logging subsystem configuration.
///
/// # Examples
///
/// ```
/// use zalo_core::{LogFormat, LoggingConfig};
///
/// let logging = LoggingConfig::default();
/// assert_eq!(logging.format(), LogFormat::Text);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct LoggingConfig {
    filter: String,
    format: LogFormat,
}

impl LoggingConfig {
    fn default_filter() -> String {
        "info".to_owned()
    }

    /// Builds a logging configuration with custom settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::{LogFormat, LoggingConfig};
    ///
    /// let logging = LoggingConfig::new("trace", LogFormat::Json);
    /// assert_eq!(logging.filter(), "trace");
    /// ```
    #[must_use]
    pub fn new(filter: impl Into<String>, format: LogFormat) -> Self {
        Self {
            filter: filter.into(),
            format,
        }
    }

    /// Returns the tracing filter expression.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::LoggingConfig;
    ///
    /// let logging = LoggingConfig::default();
    /// assert_eq!(logging.filter(), "info");
    /// ```
    #[must_use]
    pub fn filter(&self) -> &str {
        &self.filter
    }

    /// Returns the log output format.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::{LogFormat, LoggingConfig};
    ///
    /// let logging = LoggingConfig::default();
    /// assert_eq!(logging.format(), LogFormat::Text);
    /// ```
    #[must_use]
    pub fn format(&self) -> LogFormat {
        self.format
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            filter: Self::default_filter(),
            format: LogFormat::default(),
        }
    }
}

/// Supported logging output formats.
///
/// # Examples
///
/// ```
/// use zalo_core::LogFormat;
///
/// assert_eq!(LogFormat::Json, LogFormat::Json);
/// ```
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Human-readable, plain-text logs.
    #[default]
    Text,
    /// Structured JSON logs.
    Json,
}

/// Loads configuration from the filesystem and environment variables.
///
/// # Examples
///
/// ```
/// use zalo_core::ConfigLoader;
///
/// let config = ConfigLoader::default().load().expect("config");
/// assert_eq!(config.logging().filter(), "info");
/// ```
#[derive(Clone, Debug)]
pub struct ConfigLoader {
    env_prefix: String,
    file_path: Option<PathBuf>,
}

impl ConfigLoader {
    /// Creates a loader that reads variables with the provided prefix.
    ///
    /// The prefix is used as-is without additional transformation. By
    /// convention we stick to uppercase prefixes ending with an underscore.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::ConfigLoader;
    ///
    /// let loader = ConfigLoader::new("MY_APP_");
    /// let config = loader.load().expect("config");
    /// assert_eq!(config.logging().filter(), "info");
    /// ```
    #[must_use]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            env_prefix: prefix.into(),
            file_path: None,
        }
    }

    /// Provides a configuration file path merged on top of defaults.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::fs::write;
    /// # use tempfile::NamedTempFile;
    /// use zalo_core::ConfigLoader;
    ///
    /// # fn run() -> Result<(), Box<dyn std::error::Error>> {
    /// # let file = NamedTempFile::new()?;
    /// # write(
    /// #     file.path(),
    /// #     "environment = \"development\"\n[logging]\nfilter = \"trace\"\n"
    /// # )?;
    /// let loader = ConfigLoader::default().with_file_path(file.path());
    /// let config = loader.load()?;
    /// assert_eq!(config.logging().filter(), "trace");
    /// # Ok(())
    /// # }
    /// # run().expect("example executed");
    /// ```
    #[must_use]
    pub fn with_file_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.file_path = Some(path.into());
        self
    }

    /// Loads the configuration using defaults, file (optional) and environment.
    ///
    /// # Errors
    ///
    /// Returns [`CoreError::Config`] if the file is missing or Figment fails to
    /// deserialize the structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_core::ConfigLoader;
    ///
    /// let result = ConfigLoader::default().load();
    /// assert!(result.is_ok());
    /// ```
    pub fn load(&self) -> CoreResult<AppConfig> {
        let mut figment = Figment::from(Serialized::defaults(AppConfig::default()));

        if let Some(path) = &self.file_path {
            if !path_exists(path) {
                return Err(ConfigError::MissingFile { path: path.clone() }.into());
            }
            figment = figment.merge(Toml::file(path));
        }

        figment = figment.merge(Env::prefixed(&self.env_prefix).split("__"));

        figment
            .extract::<AppConfig>()
            .map_err(ConfigError::from)
            .map_err(CoreError::from)
    }
}

impl Default for ConfigLoader {
    fn default() -> Self {
        Self::new("ZALO_BOT_")
    }
}

fn path_exists(path: &Path) -> bool {
    path.exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use std::sync::Mutex;

    use tempfile::NamedTempFile;

    static ENV_GUARD: Mutex<()> = Mutex::new(());

    #[test]
    fn loads_default_configuration() {
        let _guard = ENV_GUARD.lock().expect("lock poisoned");
        std::env::remove_var("ZALO_BOT_ENVIRONMENT");
        std::env::remove_var("ZALO_BOT_LOGGING__FILTER");
        std::env::remove_var("ZALO_BOT_LOGGING__FORMAT");

        let config = ConfigLoader::default()
            .load()
            .expect("default configuration should load");

        assert_eq!(config.environment(), Environment::Development);
        assert_eq!(config.logging().filter(), "info");
        assert_eq!(config.logging().format(), LogFormat::Text);
    }

    #[test]
    fn merges_environment_variables() {
        let _guard = ENV_GUARD.lock().expect("lock poisoned");
        std::env::set_var("ZALO_BOT_ENVIRONMENT", "production");
        std::env::set_var("ZALO_BOT_LOGGING__FILTER", "debug");
        std::env::set_var("ZALO_BOT_LOGGING__FORMAT", "json");

        let config = ConfigLoader::default()
            .load()
            .expect("config should respect env overrides");

        std::env::remove_var("ZALO_BOT_ENVIRONMENT");
        std::env::remove_var("ZALO_BOT_LOGGING__FILTER");
        std::env::remove_var("ZALO_BOT_LOGGING__FORMAT");

        assert_eq!(config.environment(), Environment::Production);
        assert_eq!(config.logging().filter(), "debug");
        assert_eq!(config.logging().format(), LogFormat::Json);
    }

    #[test]
    fn fails_on_missing_file() {
        let error = ConfigLoader::default()
            .with_file_path("/definitely/missing.toml")
            .load()
            .expect_err("missing file should error");

        assert!(matches!(
            error,
            CoreError::Config(ConfigError::MissingFile { .. })
        ));
    }

    #[test]
    fn loads_from_toml_file() {
        let _guard = ENV_GUARD.lock().expect("lock poisoned");
        let file = NamedTempFile::new().expect("temp file");
        write(
            file.path(),
            r#"
                environment = "staging"

                [logging]
                filter = "warn"
                format = "text"
            "#,
        )
        .expect("write config");

        let config = ConfigLoader::default()
            .with_file_path(file.path())
            .load()
            .expect("file config should load");

        assert_eq!(config.environment(), Environment::Staging);
        assert_eq!(config.logging().filter(), "warn");
        assert_eq!(config.logging().format(), LogFormat::Text);
    }
}
