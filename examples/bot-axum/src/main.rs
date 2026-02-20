// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Zalo OA Bot example on Axum.
//!
//! This example demonstrates how to build a webhook-based bot
//! using Zalo Official Account API and Axum web framework.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};
use bytes::Bytes;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::{error, info, warn};
use zalo_bot::{ValidatedWebhookEvent, WebhookHandler, WebhookVerifier};
use zalo_http::OaClient;

/// Application configuration.
#[derive(Debug, Clone, Deserialize)]
struct AppConfig {
    /// Zalo OA access token.
    access_token: String,
    /// Webhook secret for signature verification.
    webhook_secret: String,
    /// Server bind address.
    bind_address: String,
}

impl AppConfig {
    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            access_token: std::env::var("ZALO_ACCESS_TOKEN")
                .unwrap_or_else(|_| "YOUR_ACCESS_TOKEN".to_string()),
            webhook_secret: std::env::var("ZALO_WEBHOOK_SECRET")
                .unwrap_or_else(|_| "YOUR_WEBHOOK_SECRET".to_string()),
            bind_address: std::env::var("BIND_ADDRESS")
                .unwrap_or_else(|_| "0.0.0.0:3000".to_string()),
        })
    }
}

/// Application state shared across handlers.
#[derive(Clone)]
struct AppState {
    verifier: WebhookVerifier,
    client: OaClient,
}

/// Custom webhook handler implementing business logic.
struct BotHandler;

impl WebhookHandler for BotHandler {
    fn on_follow(&self, event: &ValidatedWebhookEvent) {
        info!("New follower: {}", event.sender_id());
        // Send welcome message (would need async, so this is simplified)
    }

    fn on_unfollow(&self, event: &ValidatedWebhookEvent) {
        info!("Follower unfollowed: {}", event.sender_id());
    }

    fn on_text_message(&self, event: &ValidatedWebhookEvent) {
        if let Some(text) = event.message_text() {
            info!("Text message from {}: {}", event.sender_id(), text);
        }
    }

    fn on_image_message(&self, event: &ValidatedWebhookEvent) {
        if let Some(attachment_id) = event.attachment_id() {
            info!("Image message from {}: {}", event.sender_id(), attachment_id);
        }
    }
}

/// Webhook endpoint handler.
async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<String, (StatusCode, String)> {
    // Extract signature from header
    let signature = headers
        .get("x-zalo-hmac-sha256")
        .and_then(|v| v.to_str().ok());

    // Parse and validate webhook event
    let event = match ValidatedWebhookEvent::parse(&body, signature, &state.verifier) {
        Ok(e) => e,
        Err(e) => {
            error!("Failed to parse webhook: {}", e);
            return Err((StatusCode::BAD_REQUEST, e.to_string()));
        }
    };

    // Process the event
    let handler = BotHandler;
    
    match event.event_type() {
        zalo_types::WebhookEventType::Follow => {
            info!("Processing follow event from {}", event.sender_id());
            handler.on_follow(&event);
            
            // Send welcome message
            if let Err(e) = state.client
                .send_text_message(event.sender_id(), "Xin chào! Cảm ơn bạn đã theo dõi.")
                .await
            {
                warn!("Failed to send welcome message: {}", e);
            }
        }
        zalo_types::WebhookEventType::Unfollow => {
            info!("Processing unfollow event from {}", event.sender_id());
            handler.on_unfollow(&event);
        }
        zalo_types::WebhookEventType::UserSendText => {
            info!("Processing text message from {}", event.sender_id());
            handler.on_text_message(&event);
            
            // Echo response
            if let Some(text) = event.message_text() {
                if let Err(e) = state.client
                    .send_text_message(event.sender_id(), format!("Bạn đã gửi: {}", text))
                    .await
                {
                    warn!("Failed to send echo: {}", e);
                }
            }
        }
        zalo_types::WebhookEventType::UserSendImage => {
            info!("Processing image message from {}", event.sender_id());
            handler.on_image_message(&event);
        }
        _ => {
            info!("Received event: {:?}", event.event_type());
            handler.on_any(&event);
        }
    }

    Ok("OK".to_string())
}

/// Health check endpoint.
async fn health_check() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("bot_axum=info".parse()?)
                .add_directive("zalo_http=info".parse()?)
                .add_directive("zalo_bot=info".parse()?),
        )
        .init();

    // Load configuration
    let config = AppConfig::from_env()?;

    info!("Starting Zalo OA Bot...");
    info!("Access token: {}", mask_token(&config.access_token));
    info!("Bind address: {}", config.bind_address);

    // Create Zalo clients
    let verifier = WebhookVerifier::new(&config.webhook_secret)?;
    let client = OaClient::new(&config.access_token)?;

    let state = AppState { verifier, client };

    // Build router
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .route("/health", post(health_check))
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr: SocketAddr = config.bind_address.parse()?;
    info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Mask token for logging.
fn mask_token(token: &str) -> String {
    if token.len() > 8 {
        format!("{}...{}", &token[..4], &token[token.len() - 4..])
    } else {
        "***".to_string()
    }
}
