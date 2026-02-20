// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

#![deny(unsafe_code)]
#![deny(missing_docs)]
//! Shared primitives for the Zalo bot platform.
//!
//! The crate contains reusable configuration models and error definitions that
//! can be leveraged by both the bot server and the mini app SDK.

/// Runtime configuration facilities.
pub mod config;
/// Core error types and aliases.
pub mod error;
/// Article API types.
pub mod article;
/// Conversation API types.
pub mod conversation;
/// Image message types.
pub mod image;
/// Media upload types.
pub mod media;
/// Message types.
pub mod message;
/// Generic API response types.
pub mod response;
/// Store and Order API types.
pub mod store;
/// Tag management types.
pub mod tag;
/// User-related types.
pub mod user;
/// Webhook event types.
pub mod webhook;

pub use article::{ArticleDraft, ArticleStatus, ArticleVerification, CreateArticleRequest, VideoUploadPrepareRequest, VideoUploadPrepareResponse, VideoUploadVerifyRequest};
pub use config::{AppConfig, ConfigLoader, Environment, LogFormat, LoggingConfig};
pub use conversation::{ConversationHistory, ConversationMessage, ConversationQuery, ConversationSummary, RecentChatList, RecentChatQuery};
pub use error::{ConfigError, TypesError, TypesResult};
pub use image::{ImageAttachment, ImagePayload, SendImageRequest};
pub use masterror::{AppError, AppErrorKind, AppResult};
pub use media::{MediaUploadResponse, MediaType};
pub use message::{MessageType, Recipient, SendFileRequest, SendTemplateRequest, SendTextRequest, SendMessageResponse, TemplateButton, TemplateElement, TemplatePayload, TextPayload};
pub use response::ApiResponse;
pub use store::{CreateOrderRequest, CreateProductRequest, OrderItem, OrderList, OrderListQuery, OrderStatus, ProductStatus, ShippingInfo, StoreOrder, StoreProduct};
pub use tag::{TagFailure, TagFollowerRequest, TagInfo, TagList, TagListQuery, TagOperationResponse};
pub use user::{FollowerList, FollowerListQuery, OaInfo, UpdateFollowerRequest, UserProfile};
pub use webhook::{RecipientInfo, SenderInfo, WebhookEvent, WebhookEventType, WebhookMessage};
