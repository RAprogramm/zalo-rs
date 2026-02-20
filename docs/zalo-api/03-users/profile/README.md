# Профиль пользователя

Получение информации о подписчике.

**Реализация:** [`client.rs`](../../crates/zalo-http/src/client.rs#L134-L156)

---

## get_user_profile

```rust
use zalo_http::OaClient;

let client = OaClient::new("TOKEN")?;
let profile = client.get_user_profile("USER_ID").await?;

println!("Имя: {}", profile.display_name);
println!("Подписан: {}", profile.is_following);
```

**Endpoint:** `GET /v3.0/oa/user/detail`

**Структуры:** [`types.rs`](../../crates/zalo-http/src/types.rs#L140-L160)

---

## Ошибки

| Код | Значение |
|-----|----------|
| `-213` | Пользователь не подписан |
| `-204` | Токен истёк |

---

[← Users](README.md) | [Followers →](followers/README.md)
