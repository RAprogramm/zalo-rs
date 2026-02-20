# Примеры использования

## 1. Отправка сообщения

```rust
use zalo_http::client::OaClient;

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

## 2. Обработка вебхука на Axum

```rust
use axum::{
    extract::State,
    http::HeaderMap,
    routing::post,
    Router,
};
use bytes::Bytes;
use zalo_bot::WebhookVerifier;
use zalo_types::AppError;

#[derive(Clone)]
struct AppState {
    verifier: WebhookVerifier,
    client: OaClient,
}

async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<String, AppError> {
    // Проверяем подпись
    let signature = headers
        .get("x-zalo-hmac-sha256")
        .and_then(|v| v.to_str().ok());
    
    state.verifier.verify(&body, signature)?;
    
    // Парсим событие
    let event: WebhookEvent = serde_json::from_slice(&body)?;
    
    // Обрабатываем
    match event.event_name {
        WebhookEventType::Follow => {
            println!("Новый подписчик: {}", event.sender.id);
            
            // Отправляем приветственное сообщение
            state.client
                .send_text_message(&event.sender.id, "Добро пожаловать!")
                .await?;
        }
        WebhookEventType::UserSendText => {
            if let Some(msg) = event.message {
                let text = msg.text.unwrap_or_default();
                println!("Сообщение: {}", text);
                
                // Эхо-ответ
                state.client
                    .send_text_message(&event.sender.id, format!("Вы написали: {}", text))
                    .await?;
            }
        }
        _ => {
            println!("Событие: {:?}", event.event_name);
        }
    }
    
    Ok("OK".to_string())
}

#[tokio::main]
async fn main() {
    let verifier = WebhookVerifier::new("SECRET")?;
    let client = OaClient::new("TOKEN")?;
    
    let state = AppState { verifier, client };
    
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
}
```

---

## 3. Mini App инициализация

```rust
use zalo_sdk::{MiniAppContext, auth::AuthorizeRequest};

fn init_mini_app() -> Result<(), Box<dyn std::error::Error>> {
    let context = MiniAppContext::new("my-app-id", "my-oa-id")?;
    
    let payload = context.handshake_payload();
    
    // Serialise и отправить хосту
    let json = serde_json::to_string(&payload)?;
    
    println!("Handshake payload: {}", json);
    
    Ok(())
}
```

---

## 4. Получение списка подписчиков

```rust
use zalo_http::{client::OaClient, types::FollowerListQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OaClient::new("TOKEN")?;
    
    // Первая страница
    let mut query = FollowerListQuery::first_page(50);
    let mut all_followers = Vec::new();
    
    loop {
        let page = client.list_followers(query).await?;
        all_followers.extend(page.followers);
        
        if all_followers.len() >= page.total as usize {
            break;
        }
        
        query = FollowerListQuery::page_after(
            query.offset + query.count,
            query.count
        );
    }
    
    println!("Всего подписчиков: {}", all_followers.len());
    
    for follower in all_followers {
        println!("- {} ({})", follower.display_name, follower.user_id);
    }
    
    Ok(())
}
```

---

## 5. Массовое добавление тега

```rust
use zalo_http::client::OaClient;

const BATCH_SIZE: usize = 100;

pub async fn tag_users_in_batches(
    client: &OaClient,
    tag_id: &str,
    user_ids: Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut total_success = 0;
    let mut total_failures = 0;
    
    for chunk in user_ids.chunks(BATCH_SIZE) {
        let result = client
            .tag_follower(tag_id, chunk.to_vec())
            .await?;
        
        total_success += result.success_count;
        total_failures += result.failures.len() as u64;
        
        println!("Обработано: {}", chunk.len());
    }
    
    println!("Успешно: {}", total_success);
    println!("Ошибок: {}", total_failures);
    
    Ok(())
}
```

---

## 6. Загрузка и отправка изображения

```rust
use reqwest::multipart::{Form, Part};
use std::fs::File;

pub async fn send_image_with_upload(
    client: &OaClient,
    user_id: &str,
    image_path: &str,
    caption: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    // Загружаем изображение
    let file = File::open(image_path)?;
    let part = Part::reader(file)
        .file_name("image.jpg")
        .mime_str("image/jpeg")?;
    
    let form = Form::new().part("file", part);
    
    let http_client = reqwest::Client::new();
    let response = http_client
        .post("https://openapi.zalo.me/v3.0/oa/upload/image")
        .header("access_token", "YOUR_TOKEN")
        .multipart(form)
        .send()
        .await?;
    
    let upload_result: MediaUploadResponse = response.json().await?;
    
    // Отправляем сообщение с file_id
    let msg_id = client
        .send_image_message(user_id, &upload_result.file_id, caption)
        .await?;
    
    Ok(msg_id)
}
```

---

## 7. Конфигурация приложения

```toml
# config.toml
environment = "production"

[logging]
filter = "info,zalo_http=debug"
format = "json"

[zalo_oauth]
app_id = "YOUR_APP_ID"
redirect_uri = "https://yourapp.com/callback"

# Секреты лучше хранить в env:
# ZALO_OAUTH__SECRET_KEY=...
# ZALO_BOT__WEBHOOK_SECRET=...
```

```rust
use zalo_types::{AppConfig, ConfigLoader, Environment};

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

## 8. Полное приложение бота

```rust
use axum::{Router, routing::post};
use tracing::{info, error};
use zalo_bot::{init_tracing, WebhookVerifier};
use zalo_http::client::OaClient;
use zalo_types::{ConfigLoader, AppError};

#[derive(Clone)]
struct AppState {
    verifier: WebhookVerifier,
    client: OaClient,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Загружаем конфигурацию
    let config = ConfigLoader::default().load()?;
    
    // Инициализируем логирование
    init_tracing(&config)?;
    
    info!("Starting Zalo Bot...");
    info!("Environment: {}", config.environment().as_str());
    
    // Создаём клиенты
    let verifier = WebhookVerifier::new("WEBHOOK_SECRET")?;
    let client = OaClient::new("ACCESS_TOKEN")?;
    
    let state = AppState { verifier, client };
    
    // Настраиваем роуты
    let app = Router::new()
        .route("/webhook", post(webhook_handler))
        .route("/health", post(|| async { "OK" }))
        .with_state(state);
    
    // Запускаем сервер
    let addr = "0.0.0.0:3000";
    info!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<String, AppError> {
    // ... обработка вебхука
    Ok("OK".to_string())
}
```

---

[← Ошибки](../08-errors/README.md) | [← К началу](../README.md)
