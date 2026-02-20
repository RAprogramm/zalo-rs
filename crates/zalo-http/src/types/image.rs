// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Image message types.

use serde::Serialize;

use super::{MessageType, Recipient};

/// Image attachment payload.
#[derive(Clone, Debug, Serialize)]
pub struct ImagePayload {
    /// Image URL or token.
    pub url: String,
    /// Optional caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
}

/// Send image request.
#[derive(Debug, Serialize)]
pub struct SendImageRequest {
    /// Recipient.
    pub recipient: Recipient,
    /// Image attachment.
    pub attachment: ImageAttachment,
    /// Message type.
    #[serde(rename = "type")]
    pub message_type: MessageType,
}

/// Image attachment wrapper.
#[derive(Debug, Serialize)]
pub struct ImageAttachment {
    /// Attachment type.
    #[serde(rename = "type")]
    pub attachment_type: String,
    /// Payload.
    pub payload: ImagePayload,
}

impl SendImageRequest {
    /// Creates new image request.
    #[must_use]
    pub fn new(
        user_id: impl Into<String>,
        image_url: impl Into<String>,
        caption: Option<String>,
        message_type: MessageType,
    ) -> Self {
        Self {
            recipient: Recipient::for_user(user_id),
            attachment: ImageAttachment {
                attachment_type: "image".to_owned(),
                payload: ImagePayload {
                    url: image_url.into(),
                    caption,
                },
            },
            message_type,
        }
    }
}
