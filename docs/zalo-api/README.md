# Zalo Bot Platform

> Production-ready Rust integration for Zalo Official Account API

**Status:** Core implementation complete  
**Rust:** 1.93+ (Edition 2024)  
**Tests:** 234 passing

---

## Quick Start

```toml
[dependencies]
zalo-http = { path = "crates/zalo-http" }
zalo-bot = { path = "crates/zalo-bot" }
zalo-types = { path = "crates/zalo-types" }
```

```rust
use zalo_http::{OaClient, TokenManager, OAuthClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // OAuth & Token Management
    let oauth = OAuthClient::new("app_id", "secret", "redirect_uri");
    let manager = TokenManager::new(oauth);
    manager.initialize_with_code("auth_code").await?;
    
    // API Client with auto-refresh
    let client = OaClient::with_manager(manager).await?;
    let msg_id = client.send_text_message("user_id", "Hello!").await?;
    
    Ok(())
}
```

---

## Workspace Structure

```
zalo-rs/
├── crates/
│   ├── zalo-types/     # Config, errors, shared types
│   ├── zalo-bot/       # Webhooks, observability
│   ├── zalo-sdk/       # Mini App SDK (WASM)
│   └── zalo-http/      # OA API HTTP client
├── examples/
│   ├── bot-axum/       # Webhook server example
│   └── miniapp-leptos/ # Mini App example
└── docs/
    ├── zalo-api/       # API reference (by topic)
    └── IMPLEMENTATION_PLAN.md
```

---

## Features

### ✅ Implemented

| Component | Module | Reference |
|-----------|--------|-----------|
| OAuth 2.0 | [`zalo_http::oauth`](zalo-api/01-auth/README.md) | [`OAuthClient`](../crates/zalo-http/src/oauth.rs) |
| Token Manager | [`zalo_http::token_manager`](zalo-api/01-auth/README.md) | [`TokenManager`](../crates/zalo-http/src/token_manager.rs) |
| Secure Token | [`zalo_http::SecureToken`](zalo-api/01-auth/README.md) | [`SecureToken`](../crates/zalo-http/src/token_manager.rs#L12-L47) |
| Message API | [`zalo_http::OaClient`](zalo-api/02-messaging/README.md) | [`send_text_message`](../crates/zalo-http/src/client.rs#L79-L101) |
| User API | [`zalo_http::OaClient`](zalo-api/03-users/README.md) | [`get_user_profile`](../crates/zalo-http/src/client.rs#L134-L156) |
| Webhooks | [`zalo_bot::WebhookVerifier`](zalo-api/06-webhooks/README.md) | [`WebhookVerifier`](../crates/zalo-bot/src/webhook.rs) |
| Config | [`zalo_types::ConfigLoader`](zalo-api/01-auth/README.md) | [`ConfigLoader`](../crates/zalo-types/src/config.rs) |
| Tracing | [`zalo_bot::init_tracing`](zalo-api/08-errors/README.md) | [`init_tracing`](../crates/zalo-bot/src/observability.rs) |

### 🚧 In Progress

- [ ] Rate Limiter (10 req/s)
- [ ] Media Upload (multipart)
- [ ] Image/File Messages
- [ ] Tag Management
- [ ] Template Messages

---

## Core Components

### OAuth Client

Handles OAuth 2.0 flow for token acquisition.

**Location:** [`crates/zalo-http/src/oauth.rs`](../crates/zalo-http/src/oauth.rs)

```rust
pub struct OAuthClient {
    app_id: String,
    secret_key: String,
    redirect_uri: String,
}

impl OAuthClient {
    pub async fn get_access_token(&self, code: &str) -> HttpResult<OAuthTokenResponse>;
    pub async fn refresh_token(&self, refresh_token: &str) -> HttpResult<OAuthTokenResponse>;
}
```

**Docs:** [OAuth 2.0 Guide](zalo-api/01-auth/README.md#oauth-20-flow)

---

### Token Manager

Automatic token refresh with secure storage.

**Location:** [`crates/zalo-http/src/token_manager.rs`](../crates/zalo-http/src/token_manager.rs)

```rust
pub struct TokenManager {
    tokens: Arc<RwLock<AccessTokenInfo>>,
    oauth_client: OAuthClient,
    refresh_buffer: u64,  // 300s default
}

impl TokenManager {
    pub async fn get_valid_token(&self) -> HttpResult<String>;  // Auto-refresh
    pub async fn refresh_tokens(&self) -> HttpResult<()>;
}
```

**Features:**
- SecureToken with zeroize on drop
- Auto-refresh 5min before expiration
- Thread-safe (Arc<RwLock<>>)

**Docs:** [Token Management](zalo-api/01-auth/README.md#token-manager)

---

### HTTP Client

Type-safe OA API client with automatic token management.

**Location:** [`crates/zalo-http/src/client.rs`](../crates/zalo-http/src/client.rs)

```rust
pub struct OaClient {
    inner: Client,
    token_manager: Option<TokenManager>,
    token: String,
}

impl OaClient {
    pub async fn send_text_message(&self, user_id: &str, text: &str) -> HttpResult<String>;
    pub async fn get_user_profile(&self, user_id: &str) -> HttpResult<UserProfile>;
    pub async fn list_followers(&self, query: FollowerListQuery) -> HttpResult<FollowerList>;
}
```

**Docs:** [Messaging API](zalo-api/02-messaging/README.md)

---

### Webhook Verifier

HMAC-SHA256 signature verification.

**Location:** [`crates/zalo-bot/src/webhook.rs`](../crates/zalo-bot/src/webhook.rs)

```rust
pub struct WebhookVerifier {
    secret: Vec<u8>,
}

impl WebhookVerifier {
    pub fn verify(&self, payload: &[u8], signature: Option<&str>) -> BotResult<()>;
}
```

**Docs:** [Webhook Guide](zalo-api/06-webhooks/README.md#mac-signature)

---

## Error Handling

Unified error types with [`masterror`](https://crates.io/crates/masterror).

**Location:** [`crates/zalo-http/src/error.rs`](../crates/zalo-http/src/error.rs)

```rust
pub enum HttpError {
    Transport(#[from] reqwest::Error),
    Api { code: i64, message: String },
    RateLimited,      // -210
    Unauthorized,     // -204, -240
    Configuration(String),
}
```

**Error Codes:** [Full Reference](zalo-api/08-errors/README.md)

| Code | Type | Meaning |
|------|------|---------|
| `-204` | `Unauthorized` | Token expired |
| `-210` | `RateLimited` | 10 req/s exceeded |
| `-213` | `Api` | User not subscribed |
| `-214` | `Api` | Outside 24h window |

---

## Configuration

Environment-based config with TOML support.

**Location:** [`crates/zalo-types/src/config.rs`](../crates/zalo-types/src/config.rs)

```toml
# config.toml
environment = "production"

[logging]
filter = "info,zalo_http=debug"
format = "json"
```

```bash
# Environment variables
export ZALO_BOT__ENVIRONMENT=production
export ZALO_BOT__LOGGING__FILTER=debug
export ZALO_BOT__LOGGING__FORMAT=json
```

```rust
use zalo_types::ConfigLoader;

let config = ConfigLoader::default()
    .with_file_path("config.toml")
    .load()?;
```

**Docs:** [Configuration Guide](zalo-api/01-auth/README.md#configuration)

---

## Observability

Structured logging with `tracing`.

**Location:** [`crates/zalo-bot/src/observability.rs`](../crates/zalo-bot/src/observability.rs)

```rust
use zalo_bot::init_tracing;

let config = ConfigLoader::default().load()?;
init_tracing(&config)?;

tracing::info!("Bot started");
tracing::error!(error = %e, "Request failed");
```

**Formats:** `text` (default), `json`

---

## Testing

```bash
# All tests
cargo test --workspace

# Specific crate
cargo test -p zalo-http

# With output
cargo test -- --nocapture
```

**Coverage:** 234 tests passing

---

## Quality Gates

```bash
# Format
cargo +nightly fmt --

# Lint
cargo clippy -- -D warnings

# Build
cargo build --release

# Docs
cargo doc --no-deps --open
```

---

## Examples

### Bot Server (Axum)

**Location:** [`examples/bot-axum/`](../examples/bot-axum/)

```rust
// examples/bot-axum/src/main.rs
use zalo_bot::WebhookVerifier;

async fn webhook_handler(
    verifier: WebhookVerifier,
    body: Bytes,
) -> Result<String, AppError> {
    verifier.verify(&body, signature)?;
    // Handle event...
    Ok("OK".to_string())
}
```

### Mini App

**Location:** [`examples/miniapp-leptos/`](../examples/miniapp-leptos/)

```rust
// examples/miniapp-leptos/src/lib.rs
use zalo_sdk::MiniAppContext;

let ctx = MiniAppContext::new("app_id", "oa_id")?;
let payload = ctx.handshake_payload();
```

---

## Roadmap

### Q1 2026

- [x] OAuth Client
- [x] Token Manager
- [ ] Rate Limiter
- [ ] Media Upload
- [ ] Image Messages

### Q2 2026

- [ ] Template Messages
- [ ] Tag Management
- [ ] Conversation API
- [ ] Store API

**Full Plan:** [`docs/IMPLEMENTATION_PLAN.md`](IMPLEMENTATION_PLAN.md)

---

## Security

- ✅ SecureToken with memory zeroization
- ✅ HMAC-SHA256 webhook verification
- ✅ No secrets in logs
- ✅ TLS-only communication
- ✅ Token auto-refresh

---

## License

MIT — see [`LICENSE`](../LICENSE)

---

**Last Updated:** February 2026  
**Version:** 0.1.0
