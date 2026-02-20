# Загрузка медиа

Multipart/form-data загрузка.

**Статус:** В разработке

---

## upload_image

```rust
// Планируемый API
let result = client.upload_image("path/to/file.jpg").await?;
println!("File ID: {}", result.file_id);
```

**Endpoint:** `POST /v3.0/oa/upload/image`

---

## upload_file

```rust
// Планируемый API
let result = client.upload_file("path/to/doc.pdf").await?;
```

**Endpoint:** `POST /v3.0/oa/upload/file`

---

[← Media](README.md) | [Cache →](cache/README.md)
