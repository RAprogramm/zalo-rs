# Загрузка медиа

Multipart/form-data загрузка.

**Реализация:** ✅ [`media/client.rs`](../../crates/zalo-http/src/media/client.rs)

---

## upload_image

**Метод:** [`media/client.rs:38-43`](../../crates/zalo-http/src/media/client.rs#L38-L43)

```rust
use zalo_http::media::MediaManager;

let manager = MediaManager::new("ACCESS_TOKEN")?;
let result = manager.upload_image("path/to/file.jpg").await?;

println!("File ID: {}", result.file_id);
println!("URL: {}", result.url);
```

**Endpoint:** `POST /v3.0/oa/upload/image`

**Ограничения:**
- Размер: до 1 MB
- Форматы: JPG, PNG

---

## upload_document

**Метод:** [`media/client.rs:45-50`](../../crates/zalo-http/src/media/client.rs#L45-L50)

```rust
let result = manager.upload_document("path/to/doc.pdf").await?;
```

**Endpoint:** `POST /v3.0/oa/upload/file`

**Ограничения:**
- Размер: до 5 MB
- Форматы: PDF, DOC, DOCX, XLS, XLSX

---

## upload_gif

**Метод:** [`media/client.rs:52-57`](../../crates/zalo-http/src/media/client.rs#L52-L57)

```rust
let result = manager.upload_gif("path/to/animation.gif").await?;
```

**Endpoint:** `POST /v3.0/oa/upload/gif`

**Ограничения:**
- Размер: до 1 MB

---

## Загрузка из URL

### upload_image_from_url

**Метод:** [`media/client.rs:59-69`](../../crates/zalo-http/src/media/client.rs#L59-L69)

```rust
let result = manager
    .upload_image_from_url("https://example.com/image.jpg")
    .await?;
```

### upload_document_from_url

**Метод:** [`media/client.rs:71-81`](../../crates/zalo-http/src/media/client.rs#L71-L81)

```rust
let result = manager
    .upload_document_from_url("https://example.com/doc.pdf")
    .await?;
```

---

## Структуры

**Файл:** [`media/types.rs`](../../crates/zalo-http/src/media/types.rs)

- [`MediaUploadResponse`](../../crates/zalo-http/src/media/types.rs#L48-L53)
- [`UploadType`](../../crates/zalo-http/src/media/types.rs#L8-L45)
- [`MediaError`](../../crates/zalo-http/src/media/error.rs#L9-L38)

---

[← Media](README.md) | [Webhooks →](../06-webhooks/README.md)
