# Обработка ошибок

Унифицированная система ошибок.

**Реализация:** [`crates/zalo-http/src/error.rs`](../../crates/zalo-http/src/error.rs)

---

## HttpError

**Файл:** [`error.rs`](../../crates/zalo-http/src/error.rs#L10-L35)

```rust
pub enum HttpError {
    Transport(#[from] reqwest::Error),
    Api { code: i64, message: String },
    RateLimited,      // -210
    Unauthorized,     // -204, -240
    Configuration(String),
}
```

---

## Коды ошибок

| Код | Тип | Значение |
|-----|-----|----------|
| `-204` | `Unauthorized` | Токен истёк |
| `-210` | `RateLimited` | Лимит 10 req/s |
| `-213` | `Api` | Не подписан |
| `-214` | `Api` | Вне 24h окна |

**Полный список:** [`error.rs`](../../crates/zalo-http/src/error.rs#L45-L55)

---

## Маппинг

**Файл:** [`error.rs`](../../crates/zalo-http/src/error.rs#L58-L80)

```rust
impl From<HttpError> for AppError {
    fn from(error: HttpError) -> Self {
        match &error {
            HttpError::Unauthorized => AppError::with(Unauthorized, ..),
            HttpError::RateLimited => AppError::with(RateLimited, ..),
            _ => AppError::with(Internal, ..),
        }
    }
}
```

---

[← Вебхуки](06-webhooks/README.md) | [Примеры →](09-examples/README.md)
