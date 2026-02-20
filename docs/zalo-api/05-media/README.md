# Медиа

Загрузка изображений, файлов, GIF.

**Реализация:** ✅ [`crates/zalo-http/src/media/`](../../crates/zalo-http/src/media/)

---

## Методы

| Метод | Статус | Файл |
|-------|--------|------|
| **Upload** | ✅ | [upload/README.md](upload/README.md) |

---

## MediaManager

**Файл:** [`media/client.rs`](../../crates/zalo-http/src/media/client.rs)

```rust
use zalo_http::media::MediaManager;

let manager = MediaManager::new("ACCESS_TOKEN")?;

// Загрузка изображения
let result = manager.upload_image("path/to/image.jpg").await?;
println!("File ID: {}", result.file_id);
println!("URL: {}", result.url);
```

---

## Ограничения

| Тип | Размер | Форматы |
|-----|--------|---------|
| Image | 1 MB | JPG, PNG |
| File | 5 MB | PDF, DOC, DOCX, XLS, XLSX |
| GIF | 1 MB | GIF |

---

[← Tags](../04-tags/README.md) | [Upload →](upload/README.md)
