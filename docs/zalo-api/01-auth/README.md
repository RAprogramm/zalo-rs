# Аутентификация

OAuth 2.0 и управление токенами.

**Реализация:**
- ✅ OAuth: [`crates/zalo-http/src/oauth.rs`](../../crates/zalo-http/src/oauth.rs)
- ✅ Tokens: [`crates/zalo-http/src/client/token.rs`](../../crates/zalo-http/src/client/token.rs)

---

## OAuthClient

**Файл:** [`oauth.rs`](../../crates/zalo-http/src/oauth.rs)

```rust
let oauth = OAuthClient::new("app_id", "secret", "redirect_uri");

// Получение токена
let tokens = oauth.get_access_token("auth_code").await?;

// Обновление
let new_tokens = oauth.refresh_token("refresh_token").await?;
```

**Методы:**
- ✅ `get_access_token()` — [`oauth.rs:54-78`](../../crates/zalo-http/src/oauth.rs#L54-L78)
- ✅ `refresh_token()` — [`oauth.rs:85-109`](../../crates/zalo-http/src/oauth.rs#L85-L109)

---

## TokenManager

**Файл:** [`client/token.rs`](../../crates/zalo-http/src/client/token.rs)

Авто-обновление токенов.

```rust
let manager = TokenManager::new(oauth);
manager.initialize_with_code("auth_code").await?;

// Авто-refresh при необходимости
let token = manager.get_valid_token().await?;
```

**Методы:**
- ✅ `initialize_with_code()` — [`token.rs:48-55`](../../crates/zalo-http/src/client/token.rs#L48-L55)
- ✅ `initialize_with_tokens()` — [`token.rs:57-69`](../../crates/zalo-http/src/client/token.rs#L57-L69)
- ✅ `get_valid_token()` — [`token.rs:71-82`](../../crates/zalo-http/src/client/token.rs#L71-L82)
- ✅ `refresh_tokens()` — [`token.rs:103-118`](../../crates/zalo-http/src/client/token.rs#L103-L118)

**Особенности:**
- ✅ Авто-обновление за 5 мин до истечения
- ✅ SecureToken с zeroize
- ✅ Thread-safe (Arc\<RwLock\>)

---

## SecureToken

**Файл:** [`client/token/secure.rs`](../../crates/zalo-http/src/client/token/secure.rs)

```rust
pub struct SecureToken {
    inner: Arc<RwLock<String>>,
}

impl Drop for SecureToken {
    fn drop(&mut self) {
        self.inner.zeroize();  // Очистка
    }
}
```

---

## Структуры

| Структура | Файл |
|-----------|------|
| `OAuthClient` | [`oauth.rs:36-52`](../../crates/zalo-http/src/oauth.rs#L36-L52) |
| `OAuthTokenResponse` | [`oauth_types.rs:8-15`](../../crates/zalo-http/src/oauth_types.rs#L8-L15) |
| `TokenManager` | [`token.rs:16-24`](../../crates/zalo-http/src/client/token.rs#L16-L24) |
| `AccessTokenInfo` | [`token/info.rs:14-26`](../../crates/zalo-http/src/client/token/info.rs#L14-L26) |
| `SecureToken` | [`token/secure.rs:13-47`](../../crates/zalo-http/src/client/token/secure.rs#L13-L47) |

---

## Конфигурация

**Файл:** [`zalo-types/src/config.rs`](../../crates/zalo-types/src/config.rs)

```toml
[zalo_oauth]
app_id = "YOUR_APP_ID"
redirect_uri = "https://redirect"

# Env: ZALO_OAUTH__SECRET_KEY=...
```

---

## Ошибки

| Код | Тип | Обработка |
|-----|-----|-----------|
| `-204` | `Unauthorized` | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |
| `-240` | `Unauthorized` | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |

---

[Главная](README.md) | [Сообщения →](02-messaging/README.md)
