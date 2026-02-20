# Операции с тегами

Добавление и удаление тегов у пользователей.

**Реализация:** ✅ [`client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

---

## tag_followers

Добавить тег.

**Метод:** [`client_inner/client.rs:211-226`](../../crates/zalo-http/src/client_inner/client.rs#L211-L226)

```rust
use zalo_http::{OaClient, zalo_types::TagFollowerRequest};

let client = OaClient::new("TOKEN")?;

let user_ids = vec!["user1".to_string(), "user2".to_string()];
let request = TagFollowerRequest {
    tag_id: "tag_id".to_string(),
    uids: user_ids,
};

let result = client.tag_followers(request).await?;

println!("Успешно: {}", result.success_count);
for failure in &result.failures {
    eprintln!("Ошибка {}: {}", failure.user_id, failure.message);
}
```

**Лимит:** 100 пользователей за запрос

**Endpoint:** `POST /v3.0/oa/tag/tagfollower`

---

## untag_followers

Удалить тег.

**Метод:** [`client_inner/client.rs:228-243`](../../crates/zalo-http/src/client_inner/client.rs#L228-L243)

```rust
use zalo_http::{OaClient, zalo_types::TagFollowerRequest};

let client = OaClient::new("TOKEN")?;

let user_ids = vec!["user1".to_string(), "user2".to_string()];
let request = TagFollowerRequest {
    tag_id: "tag_id".to_string(),
    uids: user_ids,
};

let result = client.untag_followers(request).await?;
```

**Endpoint:** `POST /v3.0/oa/tag/rmfollowerfromtag`

---

## Структуры

**Файл:** [`zalo-types/src/tag.rs`](../../crates/zalo-types/src/tag.rs)

- [`TagFollowerRequest`](../../crates/zalo-types/src/tag.rs#L41-L46)
- [`TagOperationResponse`](../../crates/zalo-types/src/tag.rs#L49-L56)
- [`TagFailure`](../../crates/zalo-types/src/tag.rs#L59-L66)

---

[← Tags](README.md) | [Media →](../05-media/README.md)
