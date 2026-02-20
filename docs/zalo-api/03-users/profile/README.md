# Профиль пользователя

Получение информации о подписчике.

**Реализация:** ✅ [`client_inner/client.rs:162-174`](../../crates/zalo-http/src/client_inner/client.rs#L162-L174)

---

## get_user_profile

```rust
use zalo_http::OaClient;

let client = OaClient::new("TOKEN")?;
let profile = client.get_user_profile("USER_ID").await?;

println!("Имя: {}", profile.display_name);
println!("Аватар: {:?}", profile.avatar);
```

**Endpoint:** `GET /v3.0/oa/user/detail`

**Структура:** [`UserProfile`](../../crates/zalo-types/src/user.rs#L10-L23)

```rust
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub avatar: Option<String>,
    pub gender: Option<i32>,
    pub birthday: Option<String>,
}
```

---

## Ошибки

| Код | Значение | Обработка |
|-----|----------|-----------|
| `-213` | Пользователь не подписан | [`error.rs`](../../crates/zalo-http/src/error.rs) |
| `-204` | Токен истёк | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |

---

[← Users](README.md) | [Followers →](followers/README.md)
