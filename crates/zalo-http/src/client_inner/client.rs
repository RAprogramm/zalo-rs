// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! OaClient implementation.

use std::time::Duration;

use reqwest::Client as ReqwestClient;
use zalo_types::message::MessageType;
use zalo_types::user::{FollowerList, FollowerListQuery, OaInfo, UpdateFollowerRequest, UserProfile};
use zalo_types::tag::{TagFollowerRequest, TagList, TagListQuery, TagOperationResponse};
use zalo_types::conversation::{ConversationHistory, ConversationQuery, RecentChatList, RecentChatQuery};
use zalo_types::store::{CreateOrderRequest, CreateProductRequest, OrderList, OrderListQuery, StoreOrder, StoreProduct, OrderStatus};
use zalo_types::article::{ArticleVerification, ArticleVerificationQuery, CreateArticleRequest, VideoUploadPrepareRequest, VideoUploadPrepareResponse, VideoUploadVerifyRequest};

use crate::error::{HttpError, HttpResult};

use super::{send_text_message, send_typed_text_message, send_image_message, send_typed_image_message};
use super::{send_file_message, send_typed_file_message, send_template_message, send_template_message_with_elements};
use super::{get_user_profile, list_followers, get_oa_info, update_follower_info};
use super::{get_tags, tag_followers, untag_followers};
use super::{list_recent_chats, get_conversation};
use super::{create_product, update_product, get_product, list_products};
use super::{create_order, update_order, get_order, list_orders};
use super::{create_article, verify_article, upload_video_prepare, upload_video_verify};

const REQUEST_TIMEOUT_SECS: u64 = 15;

/// Zalo OA HTTP client.
#[derive(Debug)]
pub struct OaClient {
    #[allow(dead_code)]
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

    // ==================== Messaging ====================

    /// Sends text message.
    pub async fn send_text_message(
        &self,
        user_id: impl Into<String>,
        text: impl Into<String>,
    ) -> HttpResult<String> {
        send_text_message(&self.token, user_id, text).await
    }

    /// Sends typed text message.
    pub async fn send_typed_text_message(
        &self,
        user_id: impl Into<String>,
        text: impl Into<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        send_typed_text_message(&self.token, user_id, text, message_type).await
    }

    /// Sends image message.
    pub async fn send_image_message(
        &self,
        user_id: impl Into<String>,
        image_url: impl Into<String>,
        caption: Option<String>,
    ) -> HttpResult<String> {
        send_image_message(&self.token, user_id, image_url, caption).await
    }

    /// Sends typed image message.
    pub async fn send_typed_image_message(
        &self,
        user_id: impl Into<String>,
        image_url: impl Into<String>,
        caption: Option<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        send_typed_image_message(&self.token, user_id, image_url, caption, message_type).await
    }

    /// Sends file message.
    pub async fn send_file_message(
        &self,
        user_id: impl Into<String>,
        file_url: impl Into<String>,
        filename: impl Into<String>,
    ) -> HttpResult<String> {
        send_file_message(&self.token, user_id, file_url, filename).await
    }

    /// Sends typed file message.
    pub async fn send_typed_file_message(
        &self,
        user_id: impl Into<String>,
        file_url: impl Into<String>,
        filename: impl Into<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        send_typed_file_message(&self.token, user_id, file_url, filename, message_type).await
    }

    /// Sends template message.
    pub async fn send_template_message(
        &self,
        user_id: impl Into<String>,
        template_type: impl Into<String>,
        message_type: MessageType,
    ) -> HttpResult<String> {
        send_template_message(&self.token, user_id, template_type, message_type).await
    }

    /// Sends template message with elements.
    pub async fn send_template_message_with_elements(
        &self,
        user_id: impl Into<String>,
        template_type: impl Into<String>,
        message_type: MessageType,
        elements: Vec<zalo_types::TemplateElement>,
    ) -> HttpResult<String> {
        send_template_message_with_elements(&self.token, user_id, template_type, message_type, elements).await
    }

    // ==================== Users ====================

    /// Gets user profile.
    pub async fn get_user_profile(&self, user_id: impl AsRef<str>) -> HttpResult<UserProfile> {
        get_user_profile(&self.token, user_id).await
    }

    /// Lists followers.
    pub async fn list_followers(&self, query: FollowerListQuery) -> HttpResult<FollowerList> {
        list_followers(&self.token, query).await
    }

    /// Gets OA information.
    pub async fn get_oa_info(&self) -> HttpResult<OaInfo> {
        get_oa_info(&self.token).await
    }

    /// Updates follower information.
    pub async fn update_follower_info(&self, request: UpdateFollowerRequest) -> HttpResult<UserProfile> {
        update_follower_info(&self.token, request).await
    }

    // ==================== Tags ====================

    /// Lists tags.
    pub async fn get_tags(&self, query: TagListQuery) -> HttpResult<TagList> {
        get_tags(&self.token, query).await
    }

    /// Tags followers.
    pub async fn tag_followers(&self, request: TagFollowerRequest) -> HttpResult<TagOperationResponse> {
        tag_followers(&self.token, request).await
    }

    /// Removes tag from followers.
    pub async fn untag_followers(&self, request: TagFollowerRequest) -> HttpResult<TagOperationResponse> {
        untag_followers(&self.token, request).await
    }

    // ==================== Conversations ====================

    /// Lists recent chats.
    pub async fn list_recent_chats(&self, query: RecentChatQuery) -> HttpResult<RecentChatList> {
        list_recent_chats(&self.token, query).await
    }

    /// Gets conversation history.
    pub async fn get_conversation(&self, query: ConversationQuery) -> HttpResult<ConversationHistory> {
        get_conversation(&self.token, query).await
    }

    // ==================== Store ====================

    /// Creates a product.
    pub async fn create_product(&self, request: CreateProductRequest) -> HttpResult<StoreProduct> {
        create_product(&self.token, request).await
    }

    /// Updates a product.
    pub async fn update_product(&self, product_id: impl Into<String>, request: CreateProductRequest) -> HttpResult<StoreProduct> {
        update_product(&self.token, product_id, request).await
    }

    /// Gets a product by ID.
    pub async fn get_product(&self, product_id: impl AsRef<str>) -> HttpResult<StoreProduct> {
        get_product(&self.token, product_id).await
    }

    /// Lists products.
    pub async fn list_products(&self, offset: Option<u64>, count: Option<u64>) -> HttpResult<Vec<StoreProduct>> {
        list_products(&self.token, offset, count).await
    }

    /// Creates an order.
    pub async fn create_order(&self, request: CreateOrderRequest) -> HttpResult<StoreOrder> {
        create_order(&self.token, request).await
    }

    /// Updates an order.
    pub async fn update_order(&self, order_id: impl Into<String>, status: OrderStatus, reason: Option<String>) -> HttpResult<StoreOrder> {
        update_order(&self.token, order_id, status, reason).await
    }

    /// Gets an order by ID.
    pub async fn get_order(&self, order_id: impl AsRef<str>) -> HttpResult<StoreOrder> {
        get_order(&self.token, order_id).await
    }

    /// Lists orders.
    pub async fn list_orders(&self, query: OrderListQuery) -> HttpResult<OrderList> {
        list_orders(&self.token, query).await
    }

    // ==================== Articles ====================

    /// Creates an article.
    pub async fn create_article(&self, request: CreateArticleRequest) -> HttpResult<ArticleVerification> {
        create_article(&self.token, request).await
    }

    /// Verifies an article.
    pub async fn verify_article(&self, query: ArticleVerificationQuery) -> HttpResult<ArticleVerification> {
        verify_article(&self.token, query).await
    }

    /// Prepares video upload.
    pub async fn upload_video_prepare(&self, request: VideoUploadPrepareRequest) -> HttpResult<VideoUploadPrepareResponse> {
        upload_video_prepare(&self.token, request).await
    }

    /// Verifies video upload.
    pub async fn upload_video_verify(&self, request: VideoUploadVerifyRequest) -> HttpResult<()> {
        upload_video_verify(&self.token, request).await
    }
}
