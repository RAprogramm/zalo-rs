# Share API

Шеринг в Zalo.

**Реализация:** ✅ [`crates/zalo-sdk/src/share.rs`](../../crates/zalo-sdk/src/share.rs)

---

## share

```rust
use zalo_sdk::share::ShareRequest;

let request = ShareRequest::new()
    .title("Check this out!")
    .thumbnail("https://...")
    .message("Interesting content");

let response = share(request)?;
```

**Файл:** [`share.rs`](../../crates/zalo-sdk/src/share.rs)

---

[← SDK](README.md)
