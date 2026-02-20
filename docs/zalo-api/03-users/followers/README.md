# Список подписчиков

Пагинированный список followers.

**Реализация:** [`client.rs`](../../crates/zalo-http/src/client.rs#L158-L180)

---

## list_followers

```rust
use zalo_http::{OaClient, types::FollowerListQuery};

let client = OaClient::new("TOKEN")?;

// Первая страница
let query = FollowerListQuery::first_page(50);
let page = client.list_followers(query).await?;

// Следующая страница
let query = FollowerListQuery::page_after(50, 50);
```

**Endpoint:** `GET /v3.0/oa/user/getlist`

**Структуры:** [`types.rs`](../../crates/zalo-http/src/types.rs#L165-L190)

```rust
pub struct FollowerListQuery {
    pub offset: u64,
    pub count: u64,  // макс. 50
}
```

---

[← Users](README.md) | [Conversations →](conversations/README.md)
