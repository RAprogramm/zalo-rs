// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Conversation types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Summary of a recent chat conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationSummary {
    /// User ID of the conversation partner.
    pub user_id: String,
    /// User's display name.
    pub display_name: String,
    /// User's avatar URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Timestamp of the last message.
    pub last_message_time: u64,
    /// Preview of the last message.
    pub last_message_preview: String,
    /// Number of unread messages.
    pub unread_count: u64,
}

/// List of recent chats response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentChatList {
    /// List of conversation summaries.
    pub chats: Vec<ConversationSummary>,
    /// Total number of conversations.
    pub total: u64,
}

/// Query parameters for listing recent chats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentChatQuery {
    /// Offset for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}

/// Conversation message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    /// Message ID.
    pub message_id: String,
    /// Sender user ID.
    pub sender_id: String,
    /// Message type.
    pub message_type: String,
    /// Message content (text or preview).
    pub content: String,
    /// Unix timestamp of the message.
    pub timestamp: u64,
    /// Whether the message was seen.
    pub is_seen: bool,
}

/// Conversation history response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationHistory {
    /// List of messages.
    pub messages: Vec<ConversationMessage>,
    /// Total number of messages.
    pub total: u64,
}

/// Query parameters for conversation history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationQuery {
    /// User ID to get conversation with.
    pub user_id: String,
    /// Offset for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u64>,
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}
