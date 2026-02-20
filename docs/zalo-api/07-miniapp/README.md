# Mini App SDK

SDK для разработки Mini App в Zalo.

**Реализация:** ✅ [`crates/zalo-sdk/`](../../crates/zalo-sdk/)

---

## Категории

| Раздел | Статус | Файл |
|--------|--------|------|
| **User** | ✅ | [user/README.md](user/README.md) |
| **Storage** | ✅ | [storage/README.md](storage/README.md) |
| **Payment** | ✅ | [payment/README.md](payment/README.md) |
| **Navigation** | ✅ | [navigation/README.md](navigation/README.md) |
| **Events** | ✅ | [events/README.md](events/README.md) |
| **Share** | ✅ | [share.md](share.md) |

---

## Быстрый старт

```rust
use zalo_sdk::MiniAppContext;

// Инициализация контекста
let ctx = MiniAppContext::new("app_id", "oa_id")?;

// Handshake payload
let payload = ctx.handshake_payload();
```

**Файл:** [`context.rs`](../../crates/zalo-sdk/src/context.rs)

---

## Структуры

**Файл:** [`context.rs`](../../crates/zalo-sdk/src/context.rs)

```rust
pub struct MiniAppContext {
    app_id: String,
    oa_id: String,
}

impl MiniAppContext {
    pub fn new(app_id: impl Into<String>, oa_id: impl Into<String>) -> SdkResult<Self>;
    pub fn handshake_payload(&self) -> HandshakePayload;
}
```

---

[← Вебхуки](../06-webhooks/README.md) | [User API →](user/README.md)
