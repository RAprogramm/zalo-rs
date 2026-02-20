# Mini App SDK

SDK для разработки Mini App в Zalo.

**Реализация:** [`crates/zalo-sdk/`](../../crates/zalo-sdk/)

---

## Категории

| Раздел | Файл | Реализация |
|--------|------|------------|
| **User** | [user/README.md](user/README.md) | [`auth.rs`](../../crates/zalo-sdk/src/auth.rs), [`user.rs`](../../crates/zalo-sdk/src/user.rs) |
| **Storage** | [storage/README.md](storage/README.md) | [`storage.rs`](../../crates/zalo-sdk/src/storage.rs) |
| **Payment** | [payment/README.md](payment/README.md) | [`payment.rs`](../../crates/zalo-sdk/src/payment.rs) |
| **Navigation** | [navigation/README.md](navigation/README.md) | [`navigation.rs`](../../crates/zalo-sdk/src/navigation.rs) |
| **Location** | [location/README.md](../07-miniapp/location.md) | [`location.rs`](../../crates/zalo-sdk/src/location.rs) |
| **Events** | [events/README.md](events/README.md) | [`lifecycle.rs`](../../crates/zalo-sdk/src/lifecycle.rs) |
| **Share** | [share/README.md](share.md) | [`share.rs`](../../crates/zalo-sdk/src/share.rs) |

---

## Быстрый старт

```rust
use zalo_sdk::MiniAppContext;

// Инициализация контекста
let ctx = MiniAppContext::new("app_id", "oa_id")?;

// Handshake payload
let payload = ctx.handshake_payload();
```

**Пример:** [`examples/miniapp-leptos/`](../../examples/miniapp-leptos/)

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
