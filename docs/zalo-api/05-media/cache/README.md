# Кэширование медиа

Менеджер кэширования file_id.

**Статус:** В разработке

---

## MediaManager

```rust
// Планируемый API
let manager = MediaManager::new(client);

// Загрузка с кэшированием
let file_id = manager.upload_and_cache("image.jpg").await?;

// Получение из кэша
let file_id = manager.get_cached("image.jpg")?;
```

---

[← Media](README.md) | [Webhooks →](../06-webhooks/README.md)
