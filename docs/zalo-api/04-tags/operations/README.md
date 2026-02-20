# Операции с тегами

Добавление и удаление тегов у пользователей.

**Статус:** В разработке

---

## tag_follower

Добавить тег.

```rust
// Планируемый API
let user_ids = vec!["user1", "user2"];
client.tag_follower("tag_id", user_ids).await?;
```

**Лимит:** 100 пользователей за запрос

---

## remove_follower_from_tag

Удалить тег.

```rust
// Планируемый API
client.remove_follower_from_tag("tag_id", user_ids).await?;
```

---

[← Tags](README.md) | [Media →](../05-media/README.md)
