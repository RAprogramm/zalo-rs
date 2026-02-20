// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Internal request helper.

use reqwest::{RequestBuilder, Url};
use serde::de::DeserializeOwned;
use tracing::warn;

use crate::error::{HttpError, HttpResult};
use crate::types::ApiResponse;

/// Authenticated request builder.
pub struct AuthenticatedRequest {
    pub inner: RequestBuilder,
}

impl AuthenticatedRequest {
    pub fn json<S: serde::Serialize>(self, body: &S) -> Self {
        Self {
            inner: self.inner.json(body),
        }
    }

    pub fn query<S: serde::Serialize>(self, params: &S) -> Self {
        Self {
            inner: self.inner.query(params),
        }
    }

    pub async fn send_and_parse<T: DeserializeOwned>(self) -> HttpResult<T> {
        let response = self.inner.send().await.map_err(HttpError::from)?;

        let status = response.status();
        let body = response.text().await.map_err(HttpError::from)?;

        if !status.is_success() {
            warn!(status = %status, "unexpected HTTP status from Zalo API");
            return Err(HttpError::UnexpectedStatus { status: status.as_u16(), body });
        }

        let envelope: ApiResponse<T> = serde_json::from_str(&body).map_err(HttpError::from)?;

        if !envelope.is_ok() {
            warn!(
                code = envelope.error,
                message = %envelope.message,
                "Zalo API returned non-zero error code"
            );
            return Err(HttpError::from_api_response(envelope.error, envelope.message));
        }

        envelope.data.ok_or_else(|| HttpError::Api {
            code: 0,
            message: "API returned success but data field was absent".to_owned(),
        })
    }
}
