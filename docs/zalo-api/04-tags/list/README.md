# Список тегов

Получение всех тегов OA.

**Статус:** В разработке

---

## get_tags_ofoa

```rust
// Планируемый API
let tags = client.get_tags_ofoa(1, 20).await?;

for tag in tags.tags {
    println!("{}: {} подписчиков", tag.name, tag.follower_count);
}
```

**Endpoint:** `GET /v3.0/oa/tag/gettagsofoa`

---

[← Tags](README.md) | [Operations →](operations/README.md)
