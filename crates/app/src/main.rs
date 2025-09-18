use std::process::ExitCode;

use tracing::{dispatcher, info};
use zalo_core::{init_tracing, AppError, ConfigLoader};

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
    let config = ConfigLoader::default().load().map_err(AppError::from)?;

    init_tracing(&config).map_err(AppError::from)?;

    info!(
        environment = config.environment().as_str(),
        filter = config.logging().filter(),
        format = ?config.logging().format(),
        "bootstrap completed"
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
