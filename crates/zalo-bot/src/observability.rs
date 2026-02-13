// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use tracing::dispatcher::{self, Dispatch};
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};
use zalo_types::{AppConfig, LogFormat};

use crate::error::{BotError, BotResult, ObservabilityError};

/// Builds a tracing dispatcher based on the runtime configuration.
///
/// The caller can install the dispatcher manually or use [`init_tracing`].
///
/// # Errors
///
/// Returns [`ObservabilityError::InvalidFilter`] when the filter expression
/// in the configuration cannot be parsed.
///
/// # Examples
///
/// ```
/// use zalo_bot::build_tracing_dispatch;
/// use zalo_types::ConfigLoader;
///
/// # fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigLoader::default().load()?;
/// let dispatch = build_tracing_dispatch(&config)?;
/// tracing::dispatcher::with_default(&dispatch, || {
///     tracing::info!("observability ready");
/// });
/// # Ok(())
/// # }
/// # demo().expect("example executed");
/// ```
pub fn build_tracing_dispatch(config: &AppConfig) -> Result<Dispatch, ObservabilityError> {
    let filter_expression = config.logging().filter().to_owned();
    let filter = EnvFilter::try_new(filter_expression.clone()).map_err(|source| {
        ObservabilityError::InvalidFilter {
            filter: filter_expression,
            source
        }
    })?;

    let dispatch = match config.logging().format() {
        LogFormat::Json => {
            let subscriber = Registry::default().with(filter).with(fmt::layer().json());
            Dispatch::new(subscriber)
        }
        LogFormat::Text => {
            let subscriber = Registry::default().with(filter).with(fmt::layer());
            Dispatch::new(subscriber)
        }
    };

    Ok(dispatch)
}

/// Installs the global tracing subscriber according to the configuration.
///
/// # Errors
///
/// Returns [`BotError::Observability`] when the dispatcher cannot be built or
/// when the global subscriber has already been installed.
///
/// # Examples
///
/// ```
/// use zalo_bot::init_tracing;
/// use zalo_types::ConfigLoader;
///
/// # fn demo() -> Result<(), Box<dyn std::error::Error>> {
/// let config = ConfigLoader::default().load()?;
/// if tracing::dispatcher::has_been_set() {
///     return Ok(());
/// }
/// init_tracing(&config)?;
/// tracing::info!("subscriber installed");
/// # Ok(())
/// # }
/// # demo().expect("example executed");
/// ```
pub fn init_tracing(config: &AppConfig) -> BotResult<()> {
    let dispatch = build_tracing_dispatch(config)?;
    dispatcher::set_global_default(dispatch)
        .map_err(ObservabilityError::from)
        .map_err(BotError::from)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use zalo_types::{AppError, AppErrorKind, LoggingConfig};

    use super::*;

    #[test]
    fn builds_dispatcher_for_text_logs() {
        let config =
            AppConfig::default().with_logging(LoggingConfig::new("info", LogFormat::Text));
        let dispatch = build_tracing_dispatch(&config).expect("dispatcher");

        tracing::dispatcher::with_default(&dispatch, || {
            tracing::info!("boot");
        });
    }

    #[test]
    fn builds_dispatcher_for_json_logs() {
        let config =
            AppConfig::default().with_logging(LoggingConfig::new("warn", LogFormat::Json));
        let dispatch = build_tracing_dispatch(&config).expect("dispatcher");

        tracing::dispatcher::with_default(&dispatch, || {
            tracing::warn!("json log test");
        });
    }

    #[test]
    fn rejects_invalid_filter_expression() {
        let config =
            AppConfig::default().with_logging(LoggingConfig::new("=info", LogFormat::Text));
        let error = build_tracing_dispatch(&config).expect_err("invalid filter");

        match error {
            ObservabilityError::InvalidFilter {
                filter, ..
            } => {
                assert_eq!(filter, "=info");
            }
            other => panic!("unexpected error: {other:?}")
        }
    }

    #[test]
    fn init_tracing_sets_global_dispatcher() {
        if tracing::dispatcher::has_been_set() {
            return;
        }
        let logging = LoggingConfig::new("warn", LogFormat::Text);
        let config = AppConfig::default().with_logging(logging);

        init_tracing(&config).expect("initialization should succeed");

        let second = init_tracing(&config).expect_err("second init must fail");
        let app_error = AppError::from(second);
        assert!(matches!(app_error.kind, AppErrorKind::Internal));
    }
}
