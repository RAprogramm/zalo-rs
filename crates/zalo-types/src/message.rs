// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Message types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Message recipient type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Recipient {
    /// Individual user.
    #[serde(rename = "individual")]
    Individual {
        /// User ID.
        user_id: String,
    },
    /// Group chat.
    #[serde(rename = "group")]
    Group {
        /// Group ID.
        group_id: String,
    },
}

/// Message type for Zalo OA.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// Customer service message.
    Cs,
    /// Brand message.
    Brand,
}

/// Text message payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPayload {
    /// Message text.
    pub text: String,
}

/// Request to send a text message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTextRequest {
    /// Message recipient.
    pub recipient: Recipient,
    /// Message type.
    #[serde(rename = "msg_type")]
    pub message_type: MessageType,
    /// Message payload.
    pub payload: TextPayload,
}

impl SendTextRequest {
    /// Creates a new text message request.
    pub fn new(
        user_id: impl Into<String>,
        text: impl Into<String>,
        message_type: MessageType,
    ) -> Self {
        Self {
            recipient: Recipient::Individual {
                user_id: user_id.into(),
            },
            message_type,
            payload: TextPayload { text: text.into() },
        }
    }
}

/// Response from send message API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// Message ID.
    #[serde(rename = "message_id")]
    pub message_id: String,
}
