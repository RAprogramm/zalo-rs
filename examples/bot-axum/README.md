# Zalo OA Bot Example (Axum)

Пример вебхук-бота для Zalo Official Account на базе фреймворка Axum.

## Быстрый старт

### 1. Настройка переменных окружения

```bash
export ZALO_ACCESS_TOKEN="your_access_token_here"
export ZALO_WEBHOOK_SECRET="your_webhook_secret_here"
export BIND_ADDRESS="0.0.0.0:3000"
export RUST_LOG="info,bot_axum=debug,zalo_http=debug"
```

### 2. Запуск

```bash
cargo run --example bot-axum
```

Или напрямую из примера:

```bash
cd examples/bot-axum
cargo run
```

## Конфигурация

| Переменная | Описание | По умолчанию |
|------------|----------|--------------|
| `ZALO_ACCESS_TOKEN` | Токен доступа OA | `YOUR_ACCESS_TOKEN` |
| `ZALO_WEBHOOK_SECRET` | Секрет для проверки подписи | `YOUR_WEBHOOK_SECRET` |
| `BIND_ADDRESS` | Адрес для прослушивания | `0.0.0.0:3000` |
| `RUST_LOG` | Уровень логирования | `info` |

## Endpoints

### `POST /webhook`

Основной endpoint для получения вебхуков от Zalo.

**Заголовки:**
- `x-zalo-hmac-sha256` — подпись сообщения

**Тело:** JSON payload от Zalo

**Ответ:** `200 OK` с телом `OK`

### `POST /health`

Health check endpoint.

**Ответ:** `200 OK` с телом `OK`

## Обработка событий

Пример реализует обработку следующих событий:

- `follow` — отправка приветственного сообщения
- `unfollow` — логирование
- `user_send_text` — эхо-ответ
- `user_send_image` — логирование

## Расширение

Для добавления новой логики реализуйте трейт `WebhookHandler`:

```rust
use zalo_bot::WebhookHandler;
use zalo_types::WebhookEventType;

struct MyHandler;

impl WebhookHandler for MyHandler {
    fn on_follow(&self, event: &ValidatedWebhookEvent) {
        // Логика при новой подписке
    }

    fn on_text_message(&self, event: &ValidatedWebhookEvent) {
        // Логика при текстовом сообщении
        if let Some(text) = event.message_text() {
            println!("Message: {}", text);
        }
    }
}
```

## Production deployment

### Docker

```dockerfile
FROM rust:1.93 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --example bot-axum

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/examples/bot-axum /usr/local/bin/bot-axum
CMD ["bot-axum"]
```

### Systemd

```ini
[Unit]
Description=Zalo OA Bot
After=network.target

[Service]
Type=simple
User=zalo-bot
Environment=ZALO_ACCESS_TOKEN=your_token
Environment=ZALO_WEBHOOK_SECRET=your_secret
ExecStart=/usr/local/bin/bot-axum
Restart=always

[Install]
WantedBy=multi-user.target
```

## Лицензия

MIT
