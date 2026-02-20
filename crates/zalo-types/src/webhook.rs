// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Webhook event types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Webhook event payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    /// Application ID.
    pub app_id: String,
    /// Sender information.
    pub sender: SenderInfo,
    /// Recipient information.
    pub recipient: RecipientInfo,
    /// Event type.
    pub event_name: WebhookEventType,
    /// Unix timestamp of the event.
    pub timestamp: u64,
    /// Optional message data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<WebhookMessage>,
    /// MAC signature for verification.
    pub mac: String,
}

/// Sender information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenderInfo {
    /// User ID.
    pub id: String,
    /// User display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Recipient information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientInfo {
    /// OA ID.
    pub id: String,
}

/// Webhook event type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WebhookEventType {
    /// User followed the OA.
    Follow,
    /// User unfollowed the OA.
    Unfollow,
    /// User sent a text message.
    UserSendText,
    /// User sent an image.
    UserSendImage,
    /// User sent a file.
    UserSendFile,
    /// User sent a sticker.
    UserSendSticker,
    /// User sent a GIF.
    UserSendGif,
    /// User sent a location.
    UserSendLocation,
    /// User clicked a link.
    UserClickLink,
    /// User clicked a button.
    UserClickButton,
    /// User received a message.
    UserReceivedMessage,
    /// User saw a message.
    UserSeenMessage,
}

/// Webhook message data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMessage {
    /// Message type.
    #[serde(rename = "type")]
    pub message_type: String,
    /// Text content (for text messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Attachment ID (for media messages).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
}
