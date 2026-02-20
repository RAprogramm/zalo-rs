# Теги

Управление тегами подписчиков.

**Реализация:** ✅ [`client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

---

## Методы

| Метод | Статус | Файл |
|-------|--------|------|
| **List** | ✅ | [list/README.md](list/README.md) |
| **Operations** | ✅ | [operations/README.md](operations/README.md) |

---

## Структуры

**Файл:** [`zalo-types/src/tag.rs`](../../crates/zalo-types/src/tag.rs)

```rust
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub follower_count: u64,
}

pub struct TagList {
    pub tags: Vec<TagInfo>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
```

---

[← Users](../03-users/README.md) | [List →](list/README.md)
