// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Media upload manager for Zalo OA API.

mod client;
mod error;
mod types;

pub use client::MediaManager;
pub use error::MediaError;
pub use types::MediaUploadResponse;
