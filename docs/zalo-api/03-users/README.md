# Управление подписчиками

API для работы с подписчиками OA.

**Реализация:** [`crates/zalo-http/src/client.rs`](../../crates/zalo-http/src/client.rs)

---

## Методы

| Метод | Файл | Описание |
|-------|------|----------|
| **Profile** | [profile/README.md](profile/README.md) | Профиль пользователя |
| **Followers** | [followers/README.md](followers/README.md) | Список подписчиков |
| **Conversations** | [conversations/README.md](conversations/README.md) | Диалоги |

---

## Структуры

**Файл:** [`types.rs`](../../crates/zalo-http/src/types.rs)

```rust
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub avatar: String,
    pub is_following: bool,
}

pub struct FollowerList {
    pub followers: Vec<UserProfile>,
    pub total: u64,
}
```

---

[← Сообщения](../02-messaging/README.md) | [Profile →](profile/README.md)
