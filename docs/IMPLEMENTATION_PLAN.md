# План реализации Zalo Bot Platform

> Актуальное состояние и дорожная карта разработки

---

## 📊 Текущее состояние (Февраль 2026)

### ✅ Реализовано

| Крейт | Компоненты | Статус |
|-------|------------|--------|
| **zalo-types** | `ConfigLoader`, `AppConfig`, `TypesError` | ✅ Готово |
| **zalo-bot** | `WebhookVerifier`, `init_tracing` | ✅ Готово |
| **zalo-sdk** | `MiniAppContext`, базовые типы | ⚠️ Частично |
| **zalo-http** | `OaClient`, `send_text_message` | ⚠️ Частично |

### ❌ Не реализовано

- OAuth клиент (получение/обновление токенов)
- Менеджер токенов с авто-обновлением
- Rate limiter (10 req/s)
- Загрузка медиа (multipart/form-data)
- Большинство API методов OA
- Обработчик вебхуков (Axum handler)
- Store/Product API
- Article API

---

## 🎯 Приоритеты реализации

### Приоритет 1: Core (Неделя 1-2)

#### 1.1 OAuth Client (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/oauth.rs` (новый)
- `crates/zalo-http/src/types.rs` (дополнить)

**Задачи:**
- [ ] `OAuthClient::get_access_token()` — обмен code на token
- [ ] `OAuthClient::refresh_token()` — обновление токена
- [ ] Структуры: `OAuthTokenResponse`, `OAuthError`

**API:**
```rust
pub struct OAuthClient {
    app_id: String,
    secret_key: String,
    redirect_uri: String,
}

impl OAuthClient {
    pub async fn get_access_token(&self, code: &str) -> Result<OAuthTokenResponse>;
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<OAuthTokenResponse>;
}
```

---

#### 1.2 Token Manager (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/token_manager.rs` (новый)

**Задачи:**
- [ ] Хранение токенов (Arc<RwLock<>>)
- [ ] Авто-обновление за 5 мин до истечения
- [ ] SecureToken с zeroize
- [ ] Интеграция с OaClient

**API:**
```rust
pub struct TokenManager {
    tokens: Arc<RwLock<AccessTokenInfo>>,
    oauth_client: OAuthClient,
}

impl TokenManager {
    pub async fn get_valid_token(&self) -> Result<String>;
    pub async fn refresh_if_needed(&self) -> Result<()>;
}
```

---

#### 1.3 Rate Limiter (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/rate_limiter.rs` (новый)

**Задачи:**
- [ ] Token bucket алгоритм (10 токенов/сек)
- [ ] Блокировка при превышении
- [ ] Интеграция с OaClient

**API:**
```rust
pub struct RateLimiter {
    tokens: Arc<Mutex<f64>>,
    refill_rate: f64,
}

impl RateLimiter {
    pub async fn acquire(&self) -> Result<(), RateLimitError>;
}
```

---

#### 1.4 Расширение OaClient

**Файлы:**
- `crates/zalo-http/src/client.rs` (дополнить)
- `crates/zalo-http/src/types.rs` (дополнить)

**Задачи:**
- [ ] `send_image_message()` — изображение по file_id
- [ ] `send_file_message()` — файл по file_id
- [ ] `send_template_message()` — шаблон
- [ ] `send_list_message()` — список с кнопками
- [ ] Поддержка типов: `cs`, `transaction`, `promotion`

**Структуры для добавления:**
```rust
// types.rs
pub struct SendImageRequest { /* ... */ }
pub struct SendFileRequest { /* ... */ }
pub struct SendTemplateRequest { /* ... */ }
pub struct ListTemplate { /* ... */ }
pub enum MessageType { Cs, Transaction, Promotion }
```

---

### Приоритет 2: User Management (Неделя 3)

#### 2.1 User API (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/client.rs` (дополнить)
- `crates/zalo-http/src/types.rs` (дополнить)

**Задачи:**
- [ ] `get_user_profile()` — профиль пользователя ✅ (есть)
- [ ] `list_followers()` — список подписчиков ✅ (есть)
- [ ] `update_follower_info()` — обновление данных
- [ ] `get_oa_info()` — информация об OA

**Структуры:**
```rust
pub struct UpdateFollowerRequest {
    pub user_id: String,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub birthday: Option<String>,
}
```

---

#### 2.2 Tag API (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/client.rs` (дополнить)
- `crates/zalo-http/src/types.rs` (дополнить)

**Задачи:**
- [ ] `get_tags_ofoa()` — список тегов
- [ ] `tag_follower()` — добавить тег
- [ ] `remove_follower_from_tag()` — удалить тег
- [ ] Массовые операции (batch 100 users)

**Структуры:**
```rust
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub follower_count: u64,
}

pub struct TagFollowerRequest {
    pub tag_id: String,
    pub uids: Vec<String>,
}
```

---

### Приоритет 3: Media (Неделя 4)

#### 3.1 Media Upload (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/media.rs` (новый)
- `crates/zalo-http/src/client.rs` (дополнить)

**Задачи:**
- [ ] `upload_image()` — загрузка изображения
- [ ] `upload_file()` — загрузка файла
- [ ] `upload_gif()` — загрузка GIF
- [ ] `MediaManager` с кэшированием file_id

**API:**
```rust
pub struct MediaManager {
    client: reqwest::Client,
    cache: RwLock<HashMap<String, MediaUploadResponse>>,
}

impl MediaManager {
    pub async fn upload_image(&self, path: &str) -> Result<MediaUploadResponse>;
    pub async fn upload_file(&self, path: &str) -> Result<MediaUploadResponse>;
}
```

---

### Приоритет 4: Webhooks (Неделя 5)

#### 4.1 Webhook Handler (`zalo-bot` + пример)

**Файлы:**
- `crates/zalo-bot/src/webhook.rs` (дополнить)
- `examples/bot-axum/src/main.rs` (переписать)

**Задачи:**
- [ ] `WebhookEvent` enum со всеми типами событий
- [ ] Парсинг JSON payload
- [ ] Обработка событий: follow, unfollow, user_send_*
- [ ] Пример на Axum с проверкой подписи

**Структуры:**
```rust
pub enum WebhookEventType {
    Follow,
    Unfollow,
    UserSendText,
    UserSendImage,
    // ...
}

pub struct WebhookEvent {
    pub event_name: WebhookEventType,
    pub sender: SenderInfo,
    pub message: Option<WebhookMessage>,
    // ...
}
```

---

#### 4.2 Conversation API (`zalo-http`)

**Файлы:**
- `crates/zalo-http/src/client.rs` (дополнить)

**Задачи:**
- [ ] `list_recent_chats()` — последние диалоги
- [ ] `get_conversation()` — история переписки

---

### Приоритет 5: Mini App SDK (Неделя 6-7)

#### 5.1 Расширение zalo-sdk

**Файлы:**
- `crates/zalo-sdk/src/*.rs` (дополнить существующие)

**Задачи:**
- [ ] `auth::authorize()` — OAuth flow
- [ ] `user::get_user_info()` — информация о пользователе
- [ ] `user::get_phone_number()` — телефон
- [ ] `storage::set_item()`, `storage::get_item()` — кэш
- [ ] `payment::checkout()` — оплата
- [ ] `location::get_location()` — геолокация
- [ ] `navigation::open_webview()` — WebView
- [ ] `share::share()` — шеринг

---

### Приоритет 6: Advanced (Неделя 8+)

#### 6.1 Article API

**Файлы:**
- `crates/zalo-http/src/article.rs` (новый)

**Задачи:**
- [ ] `create_article()` — создание статьи
- [ ] `upload_video()` — загрузка видео
- [ ] `verify_article()` — проверка статьи

---

#### 6.2 Store API

**Файлы:**
- `crates/zalo-http/src/store.rs` (новый)

**Задачи:**
- [ ] `create_product()` — создание товара
- [ ] `update_product()` — обновление
- [ ] `create_order()` — создание заказа
- [ ] `get_order()` — получение заказа

---

#### 6.3 Observability

**Файлы:**
- `crates/zalo-http/src/metrics.rs` (новый)

**Задачи:**
- [ ] Метрики: запросы, ошибки, latency
- [ ] tracing spans для API вызовов
- [ ] Логирование без утечки секретов

---

## 📈 Прогресс по крейтам

### zalo-types

```
✅ ConfigLoader, AppConfig
✅ TypesError, ConfigError
⏳ AccessTokenInfo (нужно добавить)
⏳ WebhookEvent структуры
⏳ Message типы (Image, File, Template)
```

### zalo-bot

```
✅ WebhookVerifier (MAC проверка)
✅ init_tracing, build_tracing_dispatch
⏳ WebhookEvent парсинг
⏳ Event handlers (follow, user_send_*)
```

### zalo-sdk

```
✅ MiniAppContext, HandshakePayload
✅ auth типы (AuthorizeRequest/Response)
✅ storage типы
✅ payment типы
✅ location типы
⏳ Реализация API вызовов (WASM)
```

### zalo-http

```
✅ OaClient (базовый)
✅ send_text_message()
✅ get_user_profile()
✅ list_followers()
⏳ OAuthClient
⏳ TokenManager
⏳ RateLimiter
⏳ send_image/file/template_message()
⏳ upload_image/file/gif()
⏳ Tag API методы
⏳ Conversation API
```

---

## 🎯 Ближайшие задачи (Sprint 1)

### Неделя 1

1. **OAuth Client** (`zalo-http/src/oauth.rs`)
   - [ ] `get_access_token()`
   - [ ] `refresh_token()`
   - [ ] Тесты

2. **Token Manager** (`zalo-http/src/token_manager.rs`)
   - [ ] Хранение токенов
   - [ ] Авто-обновление
   - [ ] Интеграция с OaClient

3. **Rate Limiter** (`zalo-http/src/rate_limiter.rs`)
   - [ ] Token bucket
   - [ ] Интеграция с OaClient

### Неделя 2

4. **Расширение OaClient**
   - [ ] `send_image_message()`
   - [ ] `send_file_message()`
   - [ ] `send_template_message()`

5. **Media Upload** (`zalo-http/src/media.rs`)
   - [ ] `upload_image()`
   - [ ] `upload_file()`
   - [ ] `MediaManager`

---

## 📝 Notes

- Все новые модули должны иметь 100% покрытие тестами
- Использовать `masterror` для ошибок
- Логирование через `tracing`
- Документация через rustdoc

---

*Последнее обновление: Февраль 2026*
