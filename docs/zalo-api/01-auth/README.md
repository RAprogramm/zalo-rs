# Аутентификация

OAuth 2.0 и управление токенами.

**Реализация:**
- OAuth: [`crates/zalo-http/src/oauth.rs`](../../crates/zalo-http/src/oauth.rs)
- Tokens: [`crates/zalo-http/src/token_manager.rs`](../../crates/zalo-http/src/token_manager.rs)

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

---

## TokenManager

**Файл:** [`token_manager.rs`](../../crates/zalo-http/src/token_manager.rs#L140-L180)

Авто-обновление токенов.

```rust
let manager = TokenManager::new(oauth);
manager.initialize_with_code("auth_code").await?;

// Авто-refresh при необходимости
let token = manager.get_valid_token().await?;
```

**Особенности:**
- ✅ Авто-обновление за 5 мин до истечения
- ✅ SecureToken с zeroize
- ✅ Thread-safe (Arc\<RwLock\>)

---

## SecureToken

Безопасное хранение.

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

| Код | Тип |
|-----|-----|
| `-204` | `Unauthorized` |
| `-240` | `Unauthorized` |

---

[Главная](README.md) | [Сообщения →](02-messaging/README.md)
