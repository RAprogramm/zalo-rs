// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Messaging API endpoints.

use crate::error::HttpResult;
use zalo_types::image::SendImageRequest;
use zalo_types::message::{MessageType, SendTextRequest, SendFileRequest, SendTemplateRequest, TemplateElement};
use zalo_types::SendMessageResponse;

/// Sends text message.
pub async fn send_text_message(
    token: &str,
    user_id: impl Into<String>,
    text: impl Into<String>,
) -> HttpResult<String> {
    send_typed_text_message(token, user_id, text, MessageType::Cs).await
}

/// Sends typed text message.
pub async fn send_typed_text_message(
    token: &str,
    user_id: impl Into<String>,
    text: impl Into<String>,
    message_type: MessageType,
) -> HttpResult<String> {
    let url = crate::client_inner::endpoint_url("message/cs")?;
    let body = SendTextRequest::new(user_id, text, message_type);

    tracing::debug!(endpoint = %url, "sending text message");

    let response: SendMessageResponse =
        crate::client_inner::post_json(token, url, &body).await?;

    Ok(response.message_id)
}

/// Sends image message.
pub async fn send_image_message(
    token: &str,
    user_id: impl Into<String>,
    image_url: impl Into<String>,
    caption: Option<String>,
) -> HttpResult<String> {
    send_typed_image_message(token, user_id, image_url, caption, MessageType::Cs).await
}

/// Sends typed image message.
pub async fn send_typed_image_message(
    token: &str,
    user_id: impl Into<String>,
    image_url: impl Into<String>,
    caption: Option<String>,
    message_type: MessageType,
) -> HttpResult<String> {
    let url = crate::client_inner::endpoint_url("message/cs")?;
    let body = SendImageRequest::new(user_id, image_url, caption, message_type);

    tracing::debug!(endpoint = %url, "sending image message");

    let response: SendMessageResponse =
        crate::client_inner::post_json(token, url, &body).await?;

    Ok(response.message_id)
}

/// Sends file message.
pub async fn send_file_message(
    token: &str,
    user_id: impl Into<String>,
    file_url: impl Into<String>,
    filename: impl Into<String>,
) -> HttpResult<String> {
    send_typed_file_message(token, user_id, file_url, filename, MessageType::Cs).await
}

/// Sends typed file message.
pub async fn send_typed_file_message(
    token: &str,
    user_id: impl Into<String>,
    file_url: impl Into<String>,
    filename: impl Into<String>,
    message_type: MessageType,
) -> HttpResult<String> {
    let url = crate::client_inner::endpoint_url("message/cs")?;
    let body = SendFileRequest::new(user_id, file_url, filename, message_type);

    tracing::debug!(endpoint = %url, "sending file message");

    let response: SendMessageResponse =
        crate::client_inner::post_json(token, url, &body).await?;

    Ok(response.message_id)
}

/// Sends template message.
pub async fn send_template_message(
    token: &str,
    user_id: impl Into<String>,
    template_type: impl Into<String>,
    message_type: MessageType,
) -> HttpResult<String> {
    let url = crate::client_inner::endpoint_url("message/cs")?;
    let body = SendTemplateRequest::new(user_id, template_type, message_type);

    tracing::debug!(endpoint = %url, "sending template message");

    let response: SendMessageResponse =
        crate::client_inner::post_json(token, url, &body).await?;

    Ok(response.message_id)
}

/// Sends template message with elements.
pub async fn send_template_message_with_elements(
    token: &str,
    user_id: impl Into<String>,
    template_type: impl Into<String>,
    message_type: MessageType,
    elements: Vec<TemplateElement>,
) -> HttpResult<String> {
    let url = crate::client_inner::endpoint_url("message/cs")?;
    let body = SendTemplateRequest::new(user_id, template_type, message_type)
        .with_elements(elements);

    tracing::debug!(endpoint = %url, "sending template message with elements");

    let response: SendMessageResponse =
        crate::client_inner::post_json(token, url, &body).await?;

    Ok(response.message_id)
}
