// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! OaClient implementation.

use std::time::Duration;

use reqwest::{Client as ReqwestClient, Url};
use tracing::debug;
use zalo_types::message::MessageType;
use zalo_types::{SendMessageResponse, SendImageRequest, SendTextRequest};

use crate::error::{HttpError, HttpResult};
use zalo_types::user::{FollowerList, FollowerListQuery, UserProfile};

use super::AuthenticatedRequest;

const BASE_URL: &str = "https://openapi.zalo.me/v3.0/oa/";
const REQUEST_TIMEOUT_SECS: u64 = 15;
const ACCESS_TOKEN_HEADER: &str = "access_token";

/// Zalo OA HTTP client.
#[derive(Debug)]
pub struct OaClient {
    inner: ReqwestClient,
    token: String,
}

impl OaClient {
    /// Creates new client.
    pub fn new(access_token: impl Into<String>) -> HttpResult<Self> {
        let token = access_token.into();
        if token.trim().is_empty() {
            return Err(HttpError::configuration("access token must not be empty"));
        }

        let inner = ReqwestClient::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .map_err(HttpError::from)?;

        Ok(Self { inner, token })
    }

    /// Sends text message.
    pub async fn send_text_message(
        &self,
        user_id: impl Into<String>,
        text: impl Into<String>,
    ) -> HttpResult<String> {
        self.send_typed_text_message(user_id, text, MessageType::Cs)
            .await
    }

    /// Sends typed text message.
    pub async fn send_typed_text_message(
        &self,
        user_id: impl Into<String>,
        text: impl Into<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        let url = self.endpoint("message/cs")?;
        let body = SendTextRequest::new(user_id, text, message_type);

        debug!(endpoint = %url, "sending text message");

        let response: SendMessageResponse =
            self.post(url).json(&body).send_and_parse().await?;

        Ok(response.message_id)
    }

    /// Sends image message.
    pub async fn send_image_message(
        &self,
        user_id: impl Into<String>,
        image_url: impl Into<String>,
        caption: Option<String>,
    ) -> HttpResult<String> {
        self.send_typed_image_message(user_id, image_url, caption, MessageType::Cs)
            .await
    }

    /// Sends typed image message.
    pub async fn send_typed_image_message(
        &self,
        user_id: impl Into<String>,
        image_url: impl Into<String>,
        caption: Option<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        let url = self.endpoint("message/cs")?;
        let body = SendImageRequest::new(user_id, image_url, caption, message_type);

        debug!(endpoint = %url, "sending image message");

        let response: SendMessageResponse =
            self.post(url).json(&body).send_and_parse().await?;

        Ok(response.message_id)
    }

    /// Gets user profile.
    pub async fn get_user_profile(&self, user_id: impl AsRef<str>) -> HttpResult<UserProfile> {
        let url = self.endpoint("user/detail")?;

        debug!(endpoint = %url, user_id = user_id.as_ref(), "fetching user profile");

        let profile: UserProfile = self
            .get(url)
            .query(&[("user_id", user_id.as_ref())])
            .send_and_parse()
            .await?;

        Ok(profile)
    }

    /// Lists followers.
    pub async fn list_followers(&self, query: FollowerListQuery) -> HttpResult<FollowerList> {
        let url = self.endpoint("user/getlist")?;

        debug!(
            endpoint = %url,
            offset = query.offset,
            count = query.count,
            "listing followers"
        );

        let list: FollowerList = self.get(url).query(&query).send_and_parse().await?;

        Ok(list)
    }

    fn endpoint(&self, path: &str) -> HttpResult<Url> {
        Url::parse(&format!("{BASE_URL}{path}")).map_err(|err| {
            HttpError::configuration(format!("could not build endpoint URL: {err}"))
        })
    }

    fn get(&self, url: Url) -> AuthenticatedRequest {
        AuthenticatedRequest {
            inner: self.inner.get(url).header(ACCESS_TOKEN_HEADER, &self.token),
        }
    }

    fn post(&self, url: Url) -> AuthenticatedRequest {
        AuthenticatedRequest {
            inner: self
                .inner
                .post(url)
                .header(ACCESS_TOKEN_HEADER, &self.token),
        }
    }
}
