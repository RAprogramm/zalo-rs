use tracing::dispatcher::{self, Dispatch};
use tracing_subscriber::{
    fmt,
    layer::{Layer, SubscriberExt},
    EnvFilter, Registry,
};

use crate::{
    config::{AppConfig, LogFormat},
    error::{CoreError, CoreResult, ObservabilityError},
};

/// Builds a tracing dispatcher based on the runtime configuration.
///
/// The caller can install the dispatcher manually or use [`init_tracing`].
///
/// # Examples
///
/// ```
/// use zalo_core::{build_tracing_dispatch, ConfigLoader};
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
            source,
        }
    })?;

    let fmt_layer = match config.logging().format() {
        LogFormat::Json => fmt::layer().json().boxed(),
        LogFormat::Text => fmt::layer().boxed(),
    };

    let subscriber = Registry::default().with(filter).with(fmt_layer);

    Ok(Dispatch::new(subscriber))
}

/// Installs the global tracing subscriber according to the configuration.
///
/// # Errors
///
/// Returns [`CoreError::Observability`] when the dispatcher cannot be built or
/// when the global subscriber has already been installed.
///
/// # Examples
///
/// ```
/// use zalo_core::{init_tracing, ConfigLoader};
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
pub fn init_tracing(config: &AppConfig) -> CoreResult<()> {
    let dispatch = build_tracing_dispatch(config).map_err(CoreError::from)?;
    dispatcher::set_global_default(dispatch)
        .map_err(|source| CoreError::from(ObservabilityError::from(source)))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::config::LoggingConfig;
    use masterror::{AppError, AppErrorKind};

    #[test]
    fn builds_dispatcher_for_text_logs() {
        let config = AppConfig::default().with_logging(LoggingConfig::new("info", LogFormat::Text));
        let dispatch = build_tracing_dispatch(&config).expect("dispatcher");

        tracing::dispatcher::with_default(&dispatch, || {
            tracing::info!("boot");
        });
    }

    #[test]
    fn rejects_invalid_filter_expression() {
        let config =
            AppConfig::default().with_logging(LoggingConfig::new("=info", LogFormat::Text));
        let error = build_tracing_dispatch(&config).expect_err("invalid filter");

        match error {
            ObservabilityError::InvalidFilter { filter, .. } => {
                assert_eq!(filter, "=info");
            }
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn init_tracing_sets_global_dispatcher() {
        if tracing::dispatcher::has_been_set() {
            return;
        }
        // Ensure we use a unique filter per test run to avoid collisions.
        let logging = LoggingConfig::new("warn", LogFormat::Text);
        let config = AppConfig::default().with_logging(logging);

        init_tracing(&config).expect("initialization should succeed");

        // Subsequent attempts should fail with an install error.
        let second = init_tracing(&config).expect_err("second init must fail");
        let app_error = AppError::from(second);
        assert!(matches!(app_error.kind, AppErrorKind::Internal));
    }
}
