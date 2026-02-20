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
    /// Transaction message.
    Transaction,
    /// Promotion message.
    Promotion,
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

/// File attachment payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilePayload {
    /// File URL or file_id.
    pub url: String,
    /// File name.
    pub filename: String,
}

/// Request to send a file message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendFileRequest {
    /// Message recipient.
    pub recipient: Recipient,
    /// Message type.
    #[serde(rename = "msg_type")]
    pub message_type: MessageType,
    /// Message payload.
    pub payload: FilePayload,
}

impl SendFileRequest {
    /// Creates a new file message request.
    pub fn new(
        user_id: impl Into<String>,
        url: impl Into<String>,
        filename: impl Into<String>,
        message_type: MessageType,
    ) -> Self {
        Self {
            recipient: Recipient::Individual {
                user_id: user_id.into(),
            },
            message_type,
            payload: FilePayload {
                url: url.into(),
                filename: filename.into(),
            },
        }
    }
}

/// Template button.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateButton {
    /// Button type (url, phone, etc).
    #[serde(rename = "type")]
    pub button_type: String,
    /// Button title.
    pub title: String,
    /// Button URL or phone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Template element for list messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElement {
    /// Element title.
    pub title: String,
    /// Element description.
    pub description: String,
    /// Element image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Element buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<TemplateButton>>,
}

/// Template payload for list/button messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplatePayload {
    /// Template type (list, button).
    pub template_type: String,
    /// Header title.
    pub header: Option<String>,
    /// Header subtitle.
    pub subtitle: Option<String>,
    /// Template elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elements: Option<Vec<TemplateElement>>,
    /// Global buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buttons: Option<Vec<TemplateButton>>,
}

/// Request to send a template message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTemplateRequest {
    /// Message recipient.
    pub recipient: Recipient,
    /// Message type.
    #[serde(rename = "msg_type")]
    pub message_type: MessageType,
    /// Message payload.
    pub payload: TemplatePayload,
}

impl SendTemplateRequest {
    /// Creates a new template message request.
    pub fn new(
        user_id: impl Into<String>,
        template_type: impl Into<String>,
        message_type: MessageType,
    ) -> Self {
        Self {
            recipient: Recipient::Individual {
                user_id: user_id.into(),
            },
            message_type,
            payload: TemplatePayload {
                template_type: template_type.into(),
                header: None,
                subtitle: None,
                elements: None,
                buttons: None,
            },
        }
    }

    /// Sets header and subtitle.
    pub fn with_header(mut self, header: impl Into<String>, subtitle: impl Into<String>) -> Self {
        self.payload.header = Some(header.into());
        self.payload.subtitle = Some(subtitle.into());
        self
    }

    /// Sets elements.
    pub fn with_elements(mut self, elements: Vec<TemplateElement>) -> Self {
        self.payload.elements = Some(elements);
        self
    }

    /// Sets buttons.
    pub fn with_buttons(mut self, buttons: Vec<TemplateButton>) -> Self {
        self.payload.buttons = Some(buttons);
        self
    }
}

/// Response from send message API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageResponse {
    /// Message ID.
    #[serde(rename = "message_id")]
    pub message_id: String,
}
