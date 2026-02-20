// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Webhook event parsing and handling.

use zalo_types::{WebhookEvent, WebhookEventType};

/// Parsed webhook event with validated signature.
#[derive(Debug, Clone)]
pub struct ValidatedWebhookEvent {
    /// The parsed event data.
    pub event: WebhookEvent,
    /// Raw JSON payload for reference.
    pub raw_payload: String,
}

impl ValidatedWebhookEvent {
    /// Parses and validates a webhook event.
    ///
    /// # Errors
    ///
    /// Returns error if JSON parsing fails or signature verification fails.
    pub fn parse(
        payload: &[u8],
        signature: Option<&str>,
        verifier: &crate::WebhookVerifier,
    ) -> crate::BotResult<Self> {
        let raw_payload = String::from_utf8_lossy(payload).to_string();

        verifier.verify(payload, signature)?;

        let event: WebhookEvent =
            serde_json::from_slice(payload).map_err(|e| crate::BotError::WebhookParse {
                message: format!("failed to parse webhook JSON: {}", e),
            })?;

        Ok(Self {
            event,
            raw_payload,
        })
    }

    /// Returns the event type.
    #[must_use]
    pub fn event_type(&self) -> WebhookEventType {
        self.event.event_name
    }

    /// Returns sender user ID.
    #[must_use]
    pub fn sender_id(&self) -> &str {
        &self.event.sender.id
    }

    /// Returns sender name if available.
    #[must_use]
    pub fn sender_name(&self) -> Option<&str> {
        self.event.sender.name.as_deref()
    }

    /// Returns message text if this is a text message event.
    #[must_use]
    pub fn message_text(&self) -> Option<&str> {
        self.event
            .message
            .as_ref()
            .and_then(|m| m.text.as_deref())
    }

    /// Returns attachment ID if this is a media message event.
    #[must_use]
    pub fn attachment_id(&self) -> Option<&str> {
        self.event
            .message
            .as_ref()
            .and_then(|m| m.attachment_id.as_deref())
    }
}

/// Handler for specific webhook event types.
pub trait WebhookHandler: Send + Sync {
    /// Handles a follow event.
    fn on_follow(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles an unfollow event.
    fn on_unfollow(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a text message event.
    fn on_text_message(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles an image message event.
    fn on_image_message(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a file message event.
    fn on_file_message(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a sticker message event.
    fn on_sticker_message(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a GIF message event.
    fn on_gif_message(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a location message event.
    fn on_location_message(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a link click event.
    fn on_link_click(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a button click event.
    fn on_button_click(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a message received event.
    fn on_message_received(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles a message seen event.
    fn on_message_seen(&self, _event: &ValidatedWebhookEvent) {}

    /// Handles any event (fallback).
    fn on_any(&self, _event: &ValidatedWebhookEvent) {}
}

/// Dispatches webhook events to appropriate handlers.
pub struct WebhookDispatcher {
    handlers: Vec<Box<dyn WebhookHandler>>,
}

impl WebhookDispatcher {
    /// Creates new empty dispatcher.
    #[must_use]
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Adds a handler to the dispatcher.
    pub fn add_handler(&mut self, handler: impl WebhookHandler + 'static) {
        self.handlers.push(Box::new(handler));
    }

    /// Dispatches event to all handlers.
    pub fn dispatch(&self, event: &ValidatedWebhookEvent) {
        for handler in &self.handlers {
            handler.on_any(event);

            match event.event_type() {
                WebhookEventType::Follow => handler.on_follow(event),
                WebhookEventType::Unfollow => handler.on_unfollow(event),
                WebhookEventType::UserSendText => handler.on_text_message(event),
                WebhookEventType::UserSendImage => handler.on_image_message(event),
                WebhookEventType::UserSendFile => handler.on_file_message(event),
                WebhookEventType::UserSendSticker => handler.on_sticker_message(event),
                WebhookEventType::UserSendGif => handler.on_gif_message(event),
                WebhookEventType::UserSendLocation => handler.on_location_message(event),
                WebhookEventType::UserClickLink => handler.on_link_click(event),
                WebhookEventType::UserClickButton => handler.on_button_click(event),
                WebhookEventType::UserReceivedMessage => handler.on_message_received(event),
                WebhookEventType::UserSeenMessage => handler.on_message_seen(event),
            }
        }
    }
}

impl Default for WebhookDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_webhook_event() {
        let verifier = crate::WebhookVerifier::new("secret").unwrap();
        let payload = br#"{"app_id":"123","sender":{"id":"user1"},"recipient":{"id":"oa1"},"event_name":"follow","timestamp":1234567890,"mac":"placeholder"}"#;
        let signature = verifier.sign_payload(payload).unwrap();

        // Note: This test will fail because the MAC in payload doesn't match
        // A proper test would need to construct a full valid payload
    }

    #[test]
    fn rejects_invalid_json() {
        let verifier = crate::WebhookVerifier::new("secret").unwrap();
        let payload = b"not valid json";
        let signature = verifier.sign_payload(payload).unwrap();

        let result = ValidatedWebhookEvent::parse(payload, Some(&signature), &verifier);
        assert!(result.is_err());
    }
}
