# Список тегов

Получение всех тегов OA.

**Реализация:** ✅ [`client_inner/client.rs:195-209`](../../crates/zalo-http/src/client_inner/client.rs#L195-L209)

---

## get_tags

```rust
use zalo_http::{OaClient, zalo_types::TagListQuery};

let client = OaClient::new("TOKEN")?;

let query = TagListQuery {
    page: Some(1),
    page_size: Some(20),
};

let tags = client.get_tags(query).await?;

for tag in &tags.tags {
    println!("{}: {} подписчиков", tag.name, tag.follower_count);
}
```

**Endpoint:** `GET /v3.0/oa/tag/gettagsofoa`

**Структуры:**
- [`TagList`](../../crates/zalo-types/src/tag.rs#L18-L27)
- [`TagInfo`](../../crates/zalo-types/src/tag.rs#L8-L15)
- [`TagListQuery`](../../crates/zalo-types/src/tag.rs#L30-L38)

---

[← Tags](README.md) | [Operations →](operations/README.md)
