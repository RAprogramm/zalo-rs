# Обработка ошибок

Унифицированная система ошибок.

**Реализация:** ✅ [`crates/zalo-http/src/error.rs`](../../crates/zalo-http/src/error.rs)

---

## HttpError

**Файл:** [`error.rs`](../../crates/zalo-http/src/error.rs#L14-L38)

```rust
pub enum HttpError {
    Transport(#[from] reqwest::Error),
    Api { code: i64, message: String },
    RateLimited,      // -210
    Unauthorized,     // -204, -240
    Configuration(String),
    UnexpectedStatus { status: u16, body: String },
    Deserialization(#[from] serde_json::Error),
}
```

---

## Коды ошибок

| Код | Тип | Значение | Обработка |
|-----|-----|----------|-----------|
| `-204` | `Unauthorized` | Токен истёк | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |
| `-240` | `Unauthorized` | Токен недействителен | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |
| `-210` | `RateLimited` | Лимит 10 req/s | [`error.rs:75`](../../crates/zalo-http/src/error.rs#L75) |
| `-213` | `Api` | Не подписан | [`error.rs:76`](../../crates/zalo-http/src/error.rs#L76) |
| `-214` | `Api` | Вне 24h окна | [`error.rs:76`](../../crates/zalo-http/src/error.rs#L76) |

**Полный список:** [`error.rs:70-82`](../../crates/zalo-http/src/error.rs#L70-L82)

---

## Маппинг на AppError

**Файл:** [`error.rs`](../../crates/zalo-http/src/error.rs#L90-L110)

```rust
impl From<HttpError> for AppError {
    fn from(error: HttpError) -> Self {
        match &error {
            HttpError::Transport(_) => AppError::with(Network, ..),
            HttpError::Unauthorized => AppError::with(Unauthorized, ..),
            HttpError::RateLimited => AppError::with(RateLimited, ..),
            HttpError::Configuration(_) => AppError::with(Config, ..),
            _ => AppError::with(Internal, ..),
        }
    }
}
```

---

## Тесты

```bash
cargo test -p zalo-http error
```

**Результаты:** ✅ 10 тестов passing

---

[← Вебхуки](06-webhooks/README.md) | [Примеры →](09-examples/README.md)
