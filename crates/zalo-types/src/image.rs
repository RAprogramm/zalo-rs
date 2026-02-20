// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Image message types for Zalo OA API.

use serde::{Deserialize, Serialize};

use crate::message::MessageType;

/// Image attachment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAttachment {
    /// Image URL.
    pub url: String,
    /// Image caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

/// Image message payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePayload {
    /// Image attachment.
    pub attachment: ImageAttachment,
}

/// Request to send an image message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendImageRequest {
    /// Message recipient.
    pub recipient: crate::message::Recipient,
    /// Message type.
    #[serde(rename = "msg_type")]
    pub message_type: MessageType,
    /// Message payload.
    pub payload: ImagePayload,
}

impl SendImageRequest {
    /// Creates a new image message request.
    pub fn new(
        user_id: impl Into<String>,
        image_url: impl Into<String>,
        caption: Option<String>,
        message_type: MessageType,
    ) -> Self {
        Self {
            recipient: crate::message::Recipient::Individual {
                user_id: user_id.into(),
            },
            message_type,
            payload: ImagePayload {
                attachment: ImageAttachment {
                    url: image_url.into(),
                    caption,
                },
            },
        }
    }
}
