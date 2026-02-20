// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Message types.

use serde::{Deserialize, Serialize};

/// Recipient for outbound messages.
#[derive(Clone, Debug, Serialize)]
pub struct Recipient {
    /// User identifier.
    pub user_id: String,
}

impl Recipient {
    /// Creates recipient for user.
    #[must_use]
    pub fn for_user(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
        }
    }
}

/// Message type.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// Customer Service (24h window).
    Cs,
    /// Transaction.
    Transaction,
    /// Promotion.
    Promotion,
}

/// Text payload.
#[derive(Clone, Debug, Serialize)]
pub struct TextPayload {
    /// Text content.
    pub text: String,
}

/// Send text request.
#[derive(Debug, Serialize)]
pub struct SendTextRequest {
    /// Recipient.
    pub recipient: Recipient,
    /// Message payload.
    pub message: TextPayload,
    /// Message type.
    #[serde(rename = "type")]
    pub message_type: MessageType,
}

impl SendTextRequest {
    /// Creates new text request.
    #[must_use]
    pub fn new(
        user_id: impl Into<String>,
        text: impl Into<String>,
        message_type: MessageType,
    ) -> Self {
        Self {
            recipient: Recipient::for_user(user_id),
            message: TextPayload {
                text: text.into(),
            },
            message_type,
        }
    }
}

/// Send message response.
#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    /// Message ID.
    pub message_id: String,
}
