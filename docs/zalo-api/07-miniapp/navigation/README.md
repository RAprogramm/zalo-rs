# Navigation API

Навигация и WebView.

**Реализация:** ✅ [`crates/zalo-sdk/src/navigation.rs`](../../crates/zalo-sdk/src/navigation.rs)

---

## openWebview

```rust
use zalo_sdk::navigation::OpenWebviewRequest;

let request = OpenWebviewRequest::new(
    "https://example.com",
    "External Page"
);
```

**Файл:** [`navigation.rs`](../../crates/zalo-sdk/src/navigation.rs)

---

## closeApp

```rust
use zalo_sdk::navigation::close_app;

close_app();
```

**Файл:** [`navigation.rs`](../../crates/zalo-sdk/src/navigation.rs)

---

[← SDK](README.md) | [Events →](events/README.md)
