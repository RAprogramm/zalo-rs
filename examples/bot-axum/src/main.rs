use std::process::ExitCode;

use tracing::{dispatcher, info};
use zalo_bot::init_tracing;
use zalo_types::{AppError, ConfigLoader};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            log_failure(&error);
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), AppError> {
    let config = ConfigLoader::default().load()?;

    if !dispatcher::has_been_set() {
        init_tracing(&config)?;
    }

    info!(
        environment = config.environment().as_str(),
        "bot demo ready"
    );

    Ok(())
}

fn log_failure(error: &AppError) {
    if dispatcher::has_been_set() {
        error.log();
    } else {
        eprintln!("fatal error: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_returns_error_when_config_missing() {
        std::env::set_var("ZALO_BOT_CONFIG_PATH", "/missing.toml");
        let result = run();
        std::env::remove_var("ZALO_BOT_CONFIG_PATH");

        let error = result.expect_err("config path should be required");
        assert!(matches!(error.kind, zalo_types::AppErrorKind::Config));
    }
}
