use std::env;
use std::path::{Path, PathBuf};

use figment::{
    providers::{Env, Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

use crate::error::{ConfigError, TypesError, TypesResult};

/// Application-level configuration contract.
///
/// The structure is intentionally small and focused on bootstrap concerns.
///
/// # Examples
///
/// ```
/// use zalo_types::{AppConfig, Environment};
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
    /// use zalo_types::{AppConfig, Environment};
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
    /// use zalo_types::{AppConfig, LogFormat, LoggingConfig};
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
/// use zalo_types::Environment;
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
    /// use zalo_types::Environment;
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
/// use zalo_types::{LogFormat, LoggingConfig};
///
/// let logging = LoggingConfig::new("info", LogFormat::Text);
/// assert_eq!(logging.filter(), "info");
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LoggingConfig {
    filter: String,
    format: LogFormat,
}

impl LoggingConfig {
    /// Creates a new logging configuration block.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_types::{LogFormat, LoggingConfig};
    ///
    /// let logging = LoggingConfig::new("warn", LogFormat::Json);
    /// assert_eq!(logging.format(), LogFormat::Json);
    /// ```
    #[must_use]
    pub fn new(filter: impl Into<String>, format: LogFormat) -> Self {
        Self {
            filter: filter.into(),
            format,
        }
    }

    /// Returns the configured filter expression.
    #[must_use]
    pub fn filter(&self) -> &str {
        &self.filter
    }

    /// Returns the configured logging format.
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

/// Supported output formats for logs.
///
/// # Examples
///
/// ```
/// use zalo_types::{LogFormat, LoggingConfig};
///
/// let logging = LoggingConfig::new("info", LogFormat::Json);
/// assert_eq!(matches!(logging.format(), LogFormat::Json), true);
/// ```
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat {
    /// Plain-text logs optimised for human consumption.
    Text,
    /// Structured JSON logs suitable for ingestion by log processors.
    Json,
}

impl Default for LogFormat {
    fn default() -> Self {
        Self::Text
    }
}

/// Loads configuration from environment variables and optional TOML files.
///
/// The loader honours an environment variable named `{prefix}CONFIG_PATH`
/// (for example `ZALO_BOT_CONFIG_PATH`) which, when set, overrides any file
/// path configured via [`with_file_path`](Self::with_file_path).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ConfigLoader {
    env_prefix: String,
    file_path: Option<PathBuf>,
}

impl ConfigLoader {
    /// Creates a new loader configured with the provided prefix.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_types::ConfigLoader;
    ///
    /// let loader = ConfigLoader::new("ZALO_BOT_");
    /// assert!(loader.load().is_ok());
    /// ```
    #[must_use]
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            env_prefix: prefix.into(),
            file_path: None,
        }
    }

    /// Overrides the configuration file path.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::path::Path;
    /// use zalo_types::ConfigLoader;
    ///
    /// let loader = ConfigLoader::new("ZALO_").with_file_path(Path::new("config.toml"));
    /// assert_eq!(loader.file_path().unwrap(), Path::new("config.toml"));
    /// ```
    #[must_use]
    pub fn with_file_path(mut self, path: impl AsRef<Path>) -> Self {
        self.file_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Returns the configured file path, if any.
    #[must_use]
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Loads the configuration from the configured sources.
    ///
    /// Environment variables take precedence over file values and defaults.
    ///
    /// # Errors
    ///
    /// Returns [`TypesError::Config`] when the configuration file is missing or
    /// the model fails validation.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_types::ConfigLoader;
    ///
    /// let result = ConfigLoader::default().load();
    /// assert!(result.is_ok());
    /// ```
    pub fn load(&self) -> TypesResult<AppConfig> {
        let mut figment = Figment::from(Serialized::defaults(AppConfig::default()));

        let env_path = env_config_path(&self.env_prefix);
        let resolved_path = env_path.as_deref().or(self.file_path.as_deref());

        if let Some(path) = resolved_path {
            if !path_exists(path) {
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

fn path_exists(path: &Path) -> bool {
    path.exists()
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
        std::env::remove_var("ZALO_BOT_CONFIG_PATH");

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
        std::env::remove_var("ZALO_BOT_CONFIG_PATH");

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
            TypesError::Config(ConfigError::MissingFile { .. })
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

    #[test]
    fn env_config_path_missing_file_errors() {
        let _guard = ENV_GUARD.lock().expect("lock poisoned");
        std::env::set_var("ZALO_BOT_CONFIG_PATH", "/definitely/missing.toml");

        let error = ConfigLoader::default()
            .load()
            .expect_err("missing env file should error");

        std::env::remove_var("ZALO_BOT_CONFIG_PATH");

        assert!(matches!(
            error,
            TypesError::Config(ConfigError::MissingFile { .. })
        ));
    }

    #[test]
    fn env_config_path_overrides_loader_setting() {
        let _guard = ENV_GUARD.lock().expect("lock poisoned");
        let env_file = NamedTempFile::new().expect("temp file");
        write(
            env_file.path(),
            r#"
                environment = "staging"

                [logging]
                filter = "trace"
                format = "json"
            "#,
        )
        .expect("write env config");

        let fallback = NamedTempFile::new().expect("fallback file");
        write(
            fallback.path(),
            r#"
                environment = "production"

                [logging]
                filter = "info"
                format = "text"
            "#,
        )
        .expect("write fallback config");

        std::env::set_var("ZALO_BOT_CONFIG_PATH", env_file.path());

        let config = ConfigLoader::default()
            .with_file_path(fallback.path())
            .load()
            .expect("env config should load");

        std::env::remove_var("ZALO_BOT_CONFIG_PATH");

        assert_eq!(config.environment(), Environment::Staging);
        assert_eq!(config.logging().filter(), "trace");
        assert_eq!(config.logging().format(), LogFormat::Json);
    }
}
