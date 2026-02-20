// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Configuration module.

pub mod app;
pub mod loader;
pub mod types;

pub use app::AppConfig;
pub use loader::ConfigLoader;
pub use types::{Environment, LogFormat, LoggingConfig};
