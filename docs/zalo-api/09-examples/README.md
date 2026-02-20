# Примеры использования

## 1. Отправка сообщения ✅

**Файл:** [`client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

```rust
use zalo_http::OaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OaClient::new("YOUR_ACCESS_TOKEN")?;

    // Текстовое сообщение
    let msg_id = client
        .send_text_message("USER_ID", "Привет!")
        .await?;

    println!("Отправлено: {}", msg_id);

    Ok(())
}
```

---

## 2. Обработка вебхука на Axum ✅

**Файл:** [`zalo-bot/src/webhook_event.rs`](../../crates/zalo-bot/src/webhook_event.rs)

```rust
use axum::{extract::State, http::HeaderMap, body::Bytes};
use zalo_bot::{WebhookVerifier, ValidatedWebhookEvent};

#[derive(Clone)]
struct AppState {
    verifier: WebhookVerifier,
}

async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<String, AppError> {
    let signature = headers
        .get("x-zalo-hmac-sha256")
        .and_then(|v| v.to_str().ok());

    let event = ValidatedWebhookEvent::parse(&body, signature, &state.verifier)?;

    match event.event_type() {
        WebhookEventType::Follow => {
            println!("Новый подписчик: {}", event.sender_id());
        }
        WebhookEventType::UserSendText => {
            if let Some(text) = event.message_text() {
                println!("Сообщение: {}", text);
            }
        }
        _ => {}
    }

    Ok("OK".to_string())
}
```

---

## 3. Mini App инициализация ✅

**Файл:** [`zalo-sdk/src/context.rs`](../../crates/zalo-sdk/src/context.rs)

```rust
use zalo_sdk::MiniAppContext;

fn init_mini_app() -> Result<(), Box<dyn std::error::Error>> {
    let context = MiniAppContext::new("my-app-id", "my-oa-id")?;
    let payload = context.handshake_payload();

    let json = serde_json::to_string(&payload)?;
    println!("Handshake payload: {}", json);

    Ok(())
}
```

---

## 4. Получение списка подписчиков ✅

**Файл:** [`client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

```rust
use zalo_http::{OaClient, zalo_types::FollowerListQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OaClient::new("TOKEN")?;

    let query = FollowerListQuery {
        offset: Some(0),
        count: Some(50),
    };

    let page = client.list_followers(query).await?;

    println!("Всего: {}", page.total);
    for id in &page.data {
        println!("- {}", id);
    }

    Ok(())
}
```

---

## 5. Массовое добавление тега ✅

**Файл:** [`client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

```rust
use zalo_http::{OaClient, zalo_types::TagFollowerRequest};

pub async fn tag_users(
    client: &OaClient,
    tag_id: &str,
    user_ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let request = TagFollowerRequest {
        tag_id: tag_id.to_string(),
        uids: user_ids,
    };

    let result = client.tag_followers(request).await?;

    println!("Успешно: {}", result.success_count);
    for failure in &result.failures {
        eprintln!("Ошибка {}: {}", failure.user_id, failure.message);
    }

    Ok(())
}
```

---

## 6. Загрузка изображения ✅

**Файл:** [`media/client.rs`](../../crates/zalo-http/src/media/client.rs)

```rust
use zalo_http::media::MediaManager;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = MediaManager::new("ACCESS_TOKEN")?;

    // Из файла
    let result = manager.upload_image("image.jpg").await?;
    println!("File ID: {}", result.file_id);

    // Из URL
    let result = manager
        .upload_image_from_url("https://example.com/image.jpg")
        .await?;
    println!("URL: {}", result.url);

    Ok(())
}
```

---

## 7. Конфигурация приложения ✅

**Файл:** [`zalo-types/src/config.rs`](../../crates/zalo-types/src/config.rs)

```toml
# config.toml
environment = "production"

[logging]
filter = "info,zalo_http=debug"
format = "json"
```

```rust
use zalo_types::{ConfigLoader, AppConfig};

fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config = ConfigLoader::default()
        .with_file_path("config.toml")
        .load()?;

    println!("Environment: {}", config.environment().as_str());
    println!("Log filter: {}", config.logging().filter());

    Ok(config)
}
```

---

## 8. Полное приложение бота ✅

```rust
use axum::{Router, routing::post, extract::State, http::HeaderMap, body::Bytes};
use zalo_bot::{init_tracing, WebhookVerifier, ValidatedWebhookEvent};
use zalo_http::OaClient;
use zalo_types::ConfigLoader;

#[derive(Clone)]
struct AppState {
    verifier: WebhookVerifier,
    client: OaClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigLoader::default().load()?;
    init_tracing(&config)?;

    let verifier = WebhookVerifier::new("WEBHOOK_SECRET")?;
    let client = OaClient::new("ACCESS_TOKEN")?;

    let state = AppState { verifier, client };

    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<String, Box<dyn std::error::Error>> {
    let signature = headers
        .get("x-zalo-hmac-sha256")
        .and_then(|v| v.to_str().ok());

    let event = ValidatedWebhookEvent::parse(&body, signature, &state.verifier)?;

    match event.event_type() {
        WebhookEventType::Follow => {
            state.client
                .send_text_message(event.sender_id(), "Добро пожаловать!")
                .await?;
        }
        WebhookEventType::UserSendText => {
            if let Some(text) = event.message_text() {
                state.client
                    .send_text_message(event.sender_id(), format!("Вы: {}", text))
                    .await?;
            }
        }
        _ => {}
    }

    Ok("OK".to_string())
}
```

---

[← Ошибки](08-errors/README.md) | [← К началу](README.md)
