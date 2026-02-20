// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Internal request helper.

use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use tracing::warn;

use crate::error::{HttpError, HttpResult};
use zalo_types::ApiResponse;

const BASE_URL: &str = "https://openapi.zalo.me/v3.0/oa/";
const ACCESS_TOKEN_HEADER: &str = "access_token";

/// Builds endpoint URL.
pub(crate) fn endpoint_url(path: &str) -> HttpResult<Url> {
    Url::parse(&format!("{BASE_URL}{path}")).map_err(|err| {
        HttpError::configuration(format!("could not build endpoint URL: {err}"))
    })
}

/// Creates GET request builder.
fn get_request(url: Url, token: &str) -> RequestBuilder {
    Client::new().get(url).header(ACCESS_TOKEN_HEADER, token)
}

/// Creates POST request builder.
fn post_request(url: Url, token: &str) -> RequestBuilder {
    Client::new().post(url).header(ACCESS_TOKEN_HEADER, token)
}

/// Sends POST request with JSON body.
pub(crate) async fn post_json<T: serde::Serialize, R: DeserializeOwned>(
    token: &str,
    url: Url,
    body: &T,
) -> HttpResult<R> {
    let response = post_request(url, token)
        .json(body)
        .send()
        .await
        .map_err(HttpError::from)?;

    parse_response(response).await
}

/// Sends GET request with query params.
pub(crate) async fn get_with_query<R: DeserializeOwned, Q: serde::Serialize>(
    token: &str,
    url: Url,
    query: &Q,
) -> HttpResult<R> {
    let response = get_request(url, token)
        .query(query)
        .send()
        .await
        .map_err(HttpError::from)?;

    parse_response(response).await
}

/// Sends GET request without params.
pub(crate) async fn get_simple<R: DeserializeOwned>(
    token: &str,
    url: Url,
) -> HttpResult<R> {
    let response = get_request(url, token)
        .send()
        .await
        .map_err(HttpError::from)?;

    parse_response(response).await
}

/// Parses API response.
async fn parse_response<R: DeserializeOwned>(
    response: reqwest::Response,
) -> HttpResult<R> {
    let status = response.status();
    let body = response.text().await.map_err(HttpError::from)?;

    if !status.is_success() {
        warn!(status = %status, "unexpected HTTP status from Zalo API");
        return Err(HttpError::UnexpectedStatus {
            status: status.as_u16(),
            body,
        });
    }

    let envelope: ApiResponse<R> = serde_json::from_str(&body).map_err(HttpError::from)?;

    if !envelope.is_ok() {
        let message = envelope.message.unwrap_or_else(|| "unknown error".to_string());
        warn!(
            code = envelope.error,
            %message,
            "Zalo API returned non-zero error code"
        );
        return Err(HttpError::from_api_response(envelope.error, message));
    }

    envelope.data.ok_or_else(|| HttpError::Api {
        code: 0,
        message: "API returned success but data field was absent".to_owned(),
    })
}
