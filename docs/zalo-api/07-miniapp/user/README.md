# User API

Аутентификация и получение данных пользователя.

**Реализация:** [`crates/zalo-sdk/src/auth.rs`](../../crates/zalo-sdk/src/auth.rs), [`user.rs`](../../crates/zalo-sdk/src/user.rs)

---

## authorize

Запрос прав доступа.

```rust
use zalo_sdk::auth::{AuthorizeRequest, AuthorizeResponse};

let request = AuthorizeRequest::new(app_id)
    .with_scopes(vec!["user_info", "phone_number"]);
```

**Структуры:** [`auth.rs`](../../crates/zalo-sdk/src/auth.rs#L15-L40)

---

## getUserInfo

Получение профиля пользователя.

```rust
use zalo_sdk::user::UserInfo;

let info: UserInfo = get_user_info()?;
println!("{}", info.display_name);
```

**Структуры:** [`user.rs`](../../crates/zalo-sdk/src/user.rs#L20-L50)

```rust
pub struct UserInfo {
    pub user_id: String,
    pub display_name: String,
    pub avatar: String,
    pub gender: Gender,
    pub birthday: Option<Birthday>,
}
```

---

## getPhoneNumber

Запрос номера телефона.

```rust
use zalo_sdk::user::GetPhoneNumberRequest;

let request = GetPhoneNumberRequest::new(app_id);
```

**Структуры:** [`user.rs`](../../crates/zalo-sdk/src/user.rs#L55-L80)

---

[← SDK](README.md) | [Storage →](storage/README.md)
