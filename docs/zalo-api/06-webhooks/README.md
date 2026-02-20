# Вебхуки

Приём событий от Zalo OA.

**Реализация:** ✅ [`crates/zalo-bot/src/webhook.rs`](../../crates/zalo-bot/src/webhook.rs), [`webhook_event.rs`](../../crates/zalo-bot/src/webhook_event.rs)

---

## Проверка подписи

**Файл:** [`webhook.rs`](../../crates/zalo-bot/src/webhook.rs)

```rust
use zalo_bot::WebhookVerifier;

let verifier = WebhookVerifier::new("SECRET")?;
verifier.verify(&body, signature)?;
```

**Методы:**
- ✅ `WebhookVerifier::new()` — [`webhook.rs:26-36`](../../crates/zalo-bot/src/webhook.rs#L26-L36)
- ✅ `WebhookVerifier::verify()` — [`webhook.rs:70-85`](../../crates/zalo-bot/src/webhook.rs#L70-L85)
- ✅ `WebhookVerifier::sign_payload()` — [`webhook.rs:52-60`](../../crates/zalo-bot/src/webhook.rs#L52-L60)

**Алгоритм:** HMAC-SHA256

---

## Парсинг событий

**Файл:** [`webhook_event.rs`](../../crates/zalo-bot/src/webhook_event.rs)

```rust
use zalo_bot::{WebhookVerifier, ValidatedWebhookEvent, WebhookDispatcher};

let verifier = WebhookVerifier::new("SECRET")?;
let event = ValidatedWebhookEvent::parse(&body, signature, &verifier)?;

match event.event_type() {
    WebhookEventType::Follow => { /* ... */ }
    WebhookEventType::UserSendText => { /* ... */ }
    _ => {}
}
```

**Методы:**
- ✅ `ValidatedWebhookEvent::parse()` — [`webhook_event.rs:22-43`](../../crates/zalo-bot/src/webhook_event.rs#L22-L43)
- ✅ `WebhookDispatcher::dispatch()` — [`webhook_event.rs:143-164`](../../crates/zalo-bot/src/webhook_event.rs#L143-L164)

---

## События

### Lifecycle
- ✅ `follow` — [`webhook.rs:48`](../../crates/zalo-types/src/webhook.rs#L48)
- ✅ `unfollow` — [`webhook.rs:49`](../../crates/zalo-types/src/webhook.rs#L49)

### Messages
- ✅ `user_send_text` — [`webhook.rs:50`](../../crates/zalo-types/src/webhook.rs#L50)
- ✅ `user_send_image` — [`webhook.rs:51`](../../crates/zalo-types/src/webhook.rs#L51)
- ✅ `user_send_file` — [`webhook.rs:52`](../../crates/zalo-types/src/webhook.rs#L52)
- ✅ `user_send_sticker` — [`webhook.rs:53`](../../crates/zalo-types/src/webhook.rs#L53)
- ✅ `user_send_gif` — [`webhook.rs:54`](../../crates/zalo-types/src/webhook.rs#L54)
- ✅ `user_send_location` — [`webhook.rs:55`](../../crates/zalo-types/src/webhook.rs#L55)

### Interactions
- ✅ `user_click_button` — [`webhook.rs:57`](../../crates/zalo-types/src/webhook.rs#L57)
- ✅ `user_click_link` — [`webhook.rs:56`](../../crates/zalo-types/src/webhook.rs#L56)
- ✅ `user_received_message` — [`webhook.rs:58`](../../crates/zalo-types/src/webhook.rs#L58)
- ✅ `user_seen_message` — [`webhook.rs:59`](../../crates/zalo-types/src/webhook.rs#L59)

---

## Пример (Axum)

```rust
use axum::{extract::State, body::Bytes};
use zalo_bot::{WebhookVerifier, ValidatedWebhookEvent};

async fn webhook_handler(
    State(verifier): State<WebhookVerifier>,
    body: Bytes,
) -> Result<String, AppError> {
    let signature = headers.get("x-zalo-signature").and_then(|v| v.to_str().ok());
    let event = ValidatedWebhookEvent::parse(&body, signature, &verifier)?;
    
    // Handle event...
    Ok("OK".to_string())
}
```

---

## Тесты

```bash
cargo test -p zalo-bot webhook
```

**Результаты:**
- ✅ 7 тестов в `webhook.rs`
- ✅ 2 теста в `webhook_event.rs`

---

[← Media](../05-media/README.md) | [Ошибки →](08-errors/README.md)
