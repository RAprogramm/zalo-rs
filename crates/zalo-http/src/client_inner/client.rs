// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! OaClient implementation.

use std::time::Duration;

use reqwest::{Client as ReqwestClient, Url};
use tracing::debug;
use zalo_types::message::MessageType;
use zalo_types::{SendMessageResponse, SendFileRequest, SendImageRequest, SendTemplateRequest, SendTextRequest};
use zalo_types::tag::{TagFollowerRequest, TagList, TagListQuery, TagOperationResponse};
use zalo_types::conversation::{ConversationHistory, ConversationQuery, RecentChatList, RecentChatQuery};

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

    /// Sends file message.
    pub async fn send_file_message(
        &self,
        user_id: impl Into<String>,
        file_url: impl Into<String>,
        filename: impl Into<String>,
    ) -> HttpResult<String> {
        self.send_typed_file_message(user_id, file_url, filename, MessageType::Cs)
            .await
    }

    /// Sends typed file message.
    pub async fn send_typed_file_message(
        &self,
        user_id: impl Into<String>,
        file_url: impl Into<String>,
        filename: impl Into<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        let url = self.endpoint("message/cs")?;
        let body = SendFileRequest::new(user_id, file_url, filename, message_type);

        debug!(endpoint = %url, "sending file message");

        let response: SendMessageResponse =
            self.post(url).json(&body).send_and_parse().await?;

        Ok(response.message_id)
    }

    /// Sends template message.
    pub async fn send_template_message(
        &self,
        user_id: impl Into<String>,
        template_type: impl Into<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        let url = self.endpoint("message/cs")?;
        let body = SendTemplateRequest::new(user_id, template_type, message_type);

        debug!(endpoint = %url, "sending template message");

        let response: SendMessageResponse =
            self.post(url).json(&body).send_and_parse().await?;

        Ok(response.message_id)
    }

    /// Sends template message with elements.
    pub async fn send_template_message_with_elements(
        &self,
        user_id: impl Into<String>,
        template_type: impl Into<String>,
        message_type: MessageType,
        elements: Vec<zalo_types::TemplateElement>,
    ) -> HttpResult<String> {
        let url = self.endpoint("message/cs")?;
        let body = SendTemplateRequest::new(user_id, template_type, message_type)
            .with_elements(elements);

        debug!(endpoint = %url, "sending template message with elements");

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

    /// Lists tags.
    pub async fn get_tags(&self, query: TagListQuery) -> HttpResult<TagList> {
        let url = self.endpoint("tag/gettagsofoa")?;

        debug!(
            endpoint = %url,
            page = ?query.page,
            page_size = ?query.page_size,
            "listing tags"
        );

        let list: TagList = self.get(url).query(&query).send_and_parse().await?;

        Ok(list)
    }

    /// Tags followers.
    pub async fn tag_followers(&self, request: TagFollowerRequest) -> HttpResult<TagOperationResponse> {
        let url = self.endpoint("tag/tagfollower")?;

        debug!(
            endpoint = %url,
            tag_id = %request.tag_id,
            count = request.uids.len(),
            "tagging followers"
        );

        let response: TagOperationResponse =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
    }

    /// Removes tag from followers.
    pub async fn untag_followers(&self, request: TagFollowerRequest) -> HttpResult<TagOperationResponse> {
        let url = self.endpoint("tag/rmfollowerfromtag")?;

        debug!(
            endpoint = %url,
            tag_id = %request.tag_id,
            count = request.uids.len(),
            "removing tag from followers"
        );

        let response: TagOperationResponse =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
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
