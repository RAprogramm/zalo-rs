# Events API

События жизненного цикла Mini App.

**Реализация:** [`crates/zalo-sdk/src/lifecycle.rs`](../../crates/zalo-sdk/src/lifecycle.rs)

---

## События

- `AppPaused` — переход в background
- `AppResumed` — переход в foreground
- `NetworkChanged` — изменение сети

```rust
use zalo_sdk::lifecycle::{AppLifecycleEvent, LifecyclePayload};

fn on_event(event: AppLifecycleEvent) {
    match event {
        AppLifecycleEvent::AppPaused => { /* ... */ }
        AppLifecycleEvent::AppResumed => { /* ... */ }
        _ => {}
    }
}
```

**Структуры:** [`lifecycle.rs`](../../crates/zalo-sdk/src/lifecycle.rs#L15-L40)

---

[← SDK](README.md) | [Share →](share.md)
