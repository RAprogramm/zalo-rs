# Кэширование медиа

**Статус:** ⏳ В разработке

---

## Планируемый API

```rust
use zalo_http::media::MediaManager;

let manager = MediaManager::new("ACCESS_TOKEN")?;

// Загрузка с кэшированием
let file_id = manager.upload_and_cache("image.jpg").await?;

// Получение из кэша
let file_id = manager.get_cached("image.jpg")?;
```

---

[← Media](README.md) | [Webhooks →](../06-webhooks/README.md)
