# Navigation API

Навигация и WebView.

**Реализация:** [`crates/zalo-sdk/src/navigation.rs`](../../crates/zalo-sdk/src/navigation.rs)

---

## openWebview

```rust
use zalo_sdk::navigation::OpenWebviewRequest;

let request = OpenWebviewRequest::new(
    "https://example.com",
    "External Page"
);
```

## closeApp

```rust
use zalo_sdk::navigation::close_app;

close_app();
```

**Структуры:** [`navigation.rs`](../../crates/zalo-sdk/src/navigation.rs#L15-L50)

---

[← SDK](README.md) | [Events →](events/README.md)
