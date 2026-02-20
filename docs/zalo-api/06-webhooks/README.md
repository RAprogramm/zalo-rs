# Вебхуки

Приём событий от Zalo OA.

**Реализация:** [`crates/zalo-bot/src/webhook.rs`](../../crates/zalo-bot/src/webhook.rs)  
**Пример:** [`examples/bot-axum/`](../../examples/bot-axum/)

---

## События

### Lifecycle
- `follow` — подписка
- `unfollow` — отписка

### Messages
- `user_send_text/image/file/sticker/gif/location`

### Interactions
- `user_click_button/link`
- `user_received/seen_message`

---

## Проверка подписи

**Файл:** [`webhook.rs`](../../crates/zalo-bot/src/webhook.rs#L26-L62)

```rust
use zalo_bot::WebhookVerifier;

let verifier = WebhookVerifier::new("SECRET")?;
verifier.verify(&body, signature)?;
```

**Алгоритм:** HMAC-SHA256

---

## Пример (Axum)

```rust
async fn webhook_handler(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<String, AppError> {
    state.verifier.verify(&body, signature)?;
    let event: WebhookEvent = serde_json::from_slice(&body)?;
    // Handle...
    Ok("OK".to_string())
}
```

---

## Тесты

```bash
cargo test -p zalo-bot webhook
```

---

[← Media](../05-media/README.md) | [Ошибки →](08-errors/README.md)
