// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! OaClient module.

mod client;
mod request;

pub use client::OaClient;
pub(crate) use request::AuthenticatedRequest;
