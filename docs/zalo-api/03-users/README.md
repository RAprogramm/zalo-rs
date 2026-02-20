# Управление подписчиками

API для работы с подписчиками OA.

**Реализация:** [`crates/zalo-http/src/client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

---

## Методы

| Метод | Статус | Файл |
|-------|--------|------|
| **Profile** | ✅ | [profile/README.md](profile/README.md) |
| **Followers** | ✅ | [followers/README.md](followers/README.md) |
| **Conversations** | ✅ | [conversations/README.md](conversations/README.md) |

---

## Структуры

**Файл:** [`zalo-types/src/user.rs`](../../crates/zalo-types/src/user.rs)

```rust
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub gender: Option<i32>,
    pub birthday: Option<String>,
}

pub struct FollowerList {
    pub data: Vec<String>,
    pub total: i32,
}
```

---

[← Сообщения](../02-messaging/README.md) | [Profile →](profile/README.md)
