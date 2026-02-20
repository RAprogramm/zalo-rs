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
use zalo_types::store::{CreateOrderRequest, CreateProductRequest, OrderList, OrderListQuery, StoreOrder, StoreProduct};
use zalo_types::article::{ArticleVerification, ArticleVerificationQuery, CreateArticleRequest, VideoUploadPrepareRequest, VideoUploadPrepareResponse, VideoUploadVerifyRequest};

use crate::error::{HttpError, HttpResult};
use zalo_types::user::{FollowerList, FollowerListQuery, UpdateFollowerRequest, UserProfile};

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

    /// Updates follower information.
    pub async fn update_follower_info(
        &self,
        request: UpdateFollowerRequest,
    ) -> HttpResult<UserProfile> {
        let url = self.endpoint("user/update")?;

        debug!(
            endpoint = %url,
            user_id = %request.user_id,
            "updating follower info"
        );

        let response: UserProfile =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
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

    /// Lists recent chats.
    pub async fn list_recent_chats(&self, query: RecentChatQuery) -> HttpResult<RecentChatList> {
        let url = self.endpoint("user/listrecentchat")?;

        debug!(
            endpoint = %url,
            offset = ?query.offset,
            count = ?query.count,
            "listing recent chats"
        );

        let list: RecentChatList = self.get(url).query(&query).send_and_parse().await?;

        Ok(list)
    }

    /// Gets conversation history.
    pub async fn get_conversation(
        &self,
        query: ConversationQuery,
    ) -> HttpResult<ConversationHistory> {
        let url = self.endpoint("user/conversation")?;

        debug!(
            endpoint = %url,
            user_id = %query.user_id,
            offset = ?query.offset,
            count = ?query.count,
            "fetching conversation"
        );

        let history: ConversationHistory =
            self.get(url).query(&query).send_and_parse().await?;

        Ok(history)
    }

    // ==================== Store API ====================

    /// Creates a product.
    pub async fn create_product(
        &self,
        request: CreateProductRequest,
    ) -> HttpResult<StoreProduct> {
        let url = self.endpoint("store/product/create")?;

        debug!(
            endpoint = %url,
            name = %request.name,
            code = %request.code,
            "creating product"
        );

        let response: StoreProduct =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
    }

    /// Updates a product.
    pub async fn update_product(
        &self,
        product_id: impl Into<String>,
        request: CreateProductRequest,
    ) -> HttpResult<StoreProduct> {
        let url = self.endpoint("store/product/update")?;

        debug!(
            endpoint = %url,
            product_id = %product_id.into(),
            "updating product"
        );

        let response: StoreProduct =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
    }

    /// Gets a product by ID.
    pub async fn get_product(
        &self,
        product_id: impl AsRef<str>,
    ) -> HttpResult<StoreProduct> {
        let url = self.endpoint("store/product/detail")?;

        debug!(
            endpoint = %url,
            product_id = product_id.as_ref(),
            "fetching product"
        );

        let response: StoreProduct = self
            .get(url)
            .query(&[("product_id", product_id.as_ref())])
            .send_and_parse()
            .await?;

        Ok(response)
    }

    /// Lists products.
    pub async fn list_products(
        &self,
        offset: Option<u64>,
        count: Option<u64>,
    ) -> HttpResult<Vec<StoreProduct>> {
        let url = self.endpoint("store/product/list")?;

        debug!(
            endpoint = %url,
            offset = ?offset,
            count = ?count,
            "listing products"
        );

        let mut query = Vec::new();
        if let Some(o) = offset {
            query.push(("offset", o.to_string()));
        }
        if let Some(c) = count {
            query.push(("count", c.to_string()));
        }

        let response: Vec<StoreProduct> =
            self.get(url).query(&query).send_and_parse().await?;

        Ok(response)
    }

    /// Creates an order.
    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> HttpResult<StoreOrder> {
        let url = self.endpoint("store/order/create")?;

        debug!(
            endpoint = %url,
            user_id = %request.user_id,
            "creating order"
        );

        let response: StoreOrder =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
    }

    /// Updates an order.
    pub async fn update_order(
        &self,
        order_id: impl Into<String>,
        status: zalo_types::store::OrderStatus,
        reason: Option<String>,
    ) -> HttpResult<StoreOrder> {
        let url = self.endpoint("store/order/update")?;

        let order_id_str = order_id.into();

        debug!(
            endpoint = %url,
            order_id = %order_id_str,
            status = ?status,
            "updating order"
        );

        #[derive(serde::Serialize)]
        struct UpdateOrderBody {
            order_id: String,
            status: zalo_types::store::OrderStatus,
            #[serde(skip_serializing_if = "Option::is_none")]
            reason: Option<String>,
        }

        let body = UpdateOrderBody {
            order_id: order_id_str,
            status,
            reason,
        };

        let response: StoreOrder =
            self.post(url).json(&body).send_and_parse().await?;

        Ok(response)
    }

    /// Gets an order by ID.
    pub async fn get_order(
        &self,
        order_id: impl AsRef<str>,
    ) -> HttpResult<StoreOrder> {
        let url = self.endpoint("store/order/detail")?;

        debug!(
            endpoint = %url,
            order_id = order_id.as_ref(),
            "fetching order"
        );

        let response: StoreOrder = self
            .get(url)
            .query(&[("order_id", order_id.as_ref())])
            .send_and_parse()
            .await?;

        Ok(response)
    }

    /// Lists orders.
    pub async fn list_orders(
        &self,
        query: OrderListQuery,
    ) -> HttpResult<OrderList> {
        let url = self.endpoint("store/order/list")?;

        debug!(
            endpoint = %url,
            offset = ?query.offset,
            count = ?query.count,
            status = ?query.status,
            "listing orders"
        );

        let response: OrderList =
            self.get(url).query(&query).send_and_parse().await?;

        Ok(response)
    }

    // ==================== Article API ====================

    /// Creates an article.
    pub async fn create_article(
        &self,
        request: CreateArticleRequest,
    ) -> HttpResult<ArticleVerification> {
        let url = self.endpoint("article/create")?;

        debug!(
            endpoint = %url,
            title = %request.title,
            "creating article"
        );

        let response: ArticleVerification =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
    }

    /// Verifies an article.
    pub async fn verify_article(
        &self,
        query: ArticleVerificationQuery,
    ) -> HttpResult<ArticleVerification> {
        let url = self.endpoint("article/verify")?;

        debug!(
            endpoint = %url,
            token = %query.token,
            "verifying article"
        );

        let response: ArticleVerification =
            self.get(url).query(&query).send_and_parse().await?;

        Ok(response)
    }

    /// Prepares video upload.
    pub async fn upload_video_prepare(
        &self,
        request: VideoUploadPrepareRequest,
    ) -> HttpResult<VideoUploadPrepareResponse> {
        let url = self.endpoint("article/upload_video/preparevideo")?;

        debug!(
            endpoint = %url,
            video_name = %request.video_name,
            video_size = request.video_size,
            "preparing video upload"
        );

        let response: VideoUploadPrepareResponse =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(response)
    }

    /// Verifies video upload.
    pub async fn upload_video_verify(
        &self,
        request: VideoUploadVerifyRequest,
    ) -> HttpResult<()> {
        let url = self.endpoint("article/upload_video/verify")?;

        debug!(
            endpoint = %url,
            upload_id = %request.upload_id,
            "verifying video upload"
        );

        let _: serde_json::Value =
            self.post(url).json(&request).send_and_parse().await?;

        Ok(())
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
