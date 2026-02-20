# Список подписчиков

Пагинированный список followers.

**Реализация:** ✅ [`client_inner/client.rs:176-191`](../../crates/zalo-http/src/client_inner/client.rs#L176-L191)

---

## list_followers

```rust
use zalo_http::{OaClient, zalo_types::FollowerListQuery};

let client = OaClient::new("TOKEN")?;

// Первая страница
let query = FollowerListQuery {
    offset: Some(0),
    count: Some(50),
};
let page = client.list_followers(query).await?;

// Следующая страница
let query = FollowerListQuery {
    offset: Some(50),
    count: Some(50),
};
```

**Endpoint:** `GET /v3.0/oa/user/getlist`

**Структуры:**
- [`FollowerList`](../../crates/zalo-types/src/user.rs#L38-L43)
- [`FollowerListQuery`](../../crates/zalo-types/src/user.rs#L28-L35)

```rust
pub struct FollowerListQuery {
    pub offset: Option<i32>,
    pub count: Option<i32>,
}

pub struct FollowerList {
    pub data: Vec<String>,
    pub total: i32,
}
```

---

[← Users](README.md) | [Conversations →](conversations/README.md)
