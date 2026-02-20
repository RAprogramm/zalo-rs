# Zalo API Reference 2026

> Полный справочник методов и структур для реализации Zalo Official Account API и Mini App SDK

---

## 📋 Содержание

1. [Official Account API v3.0](#official-account-api-v30)
2. [Аутентификация и токены](#аутентификация-и-токены)
3. [Сообщения](#сообщения)
4. [Управление подписчиками](#управление-подписчиками)
5. [Теги](#теги)
6. [Медиа](#медиа)
7. [Вебхуки](#вебхуки)
8. [Mini App SDK](#mini-app-sdk)
9. [Коды ошибок](#коды-ошибок)
10. [План реализации](#план-реализации)

---

## Official Account API v3.0

### Базовый URL

```
https://openapi.zalo.me/v3.0/oa/
```

### Заголовки запросов

| Заголовок | Значение |
|-----------|----------|
| `access_token` | Токен доступа OA (обязательно) |
| `Content-Type` | `application/json` или `multipart/form-data` |

---

## Аутентификация и токены

### OAuth 2.0 Flow

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   Client    │────▶│  Zalo OAuth  │────▶│  Redirect   │
│             │     │   Endpoint   │     │   URI       │
└─────────────┘     └──────────────┘     └─────────────┘
       ▲                                        │
       │                                        ▼
       │                              ┌─────────────────┐
       │                              │ Authorization   │
       │                              │ Code            │
       │                              └─────────────────┘
       │                                        │
       ▼                                        ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│ Access Token│◀────│   Token      │◀────│  Client ID  │
│ + Refresh   │     │   Endpoint   │     │ + Secret    │
└─────────────┘     └──────────────┘     └─────────────┘
```

### Параметры OAuth

| Параметр | Значение |
|----------|----------|
| `app_id` | ID приложения из Zalo Developers |
| `secret_key` | Секретный ключ приложения |
| `redirect_uri` | URI для перенаправления |
| `scope` | Запрашиваемые права |

### Токены

```rust
pub struct AccessTokenInfo {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,  // секунды до истечения
    pub expires_at: Option<u64>,  // Unix timestamp
}
```

### Endpoint получения токена

```
POST https://oauth.zalo.me/v4/access_token
```

**Параметры:**
- `app_id` — ID приложения
- `secret_key` — секретный ключ
- `code` — authorization code
- `redirect_uri` — URI перенаправления

**Ответ:**
```json
{
  "access_token": "eyJhbG...",
  "refresh_token": "eyJhbG...",
  "expires_in": 3600
}
```

---

## Сообщения

### Типы сообщений

| Тип | Описание | Ограничения |
|-----|----------|-------------|
| `cs` | Customer Service | 24-часовое окно |
| `transaction` | Транзакционное | Без ограничений |
| `promotion` | Промоциональное | Требуется верификация OA |

### 24-часовое окно

```
Пользователь отправляет сообщение ──▶ 24 часа ──▶ Окно закрывается
                                            │
                                            └──▶ OA может отправлять только
                                                 transaction/promotion
```

### Методы отправки

#### 1. Текстовое сообщение

```
POST /message/cs
POST /message/transaction
POST /message/promotion
```

**Request:**
```json
{
  "recipient": {
    "user_id": "USER_ID"
  },
  "message": {
    "text": "Привет! Это тестовое сообщение."
  },
  "type": "cs"
}
```

**Response:**
```json
{
  "error": 0,
  "message": "Success",
  "data": {
    "message_id": "msg_123456"
  }
}
```

**Структуры Rust:**
```rust
pub struct SendTextRequest {
    pub recipient: Recipient,
    pub message: TextPayload,
    pub message_type: MessageType,
}

pub struct Recipient {
    pub user_id: String,
}

pub struct TextPayload {
    pub text: String,
}

pub enum MessageType {
    Cs,           // Customer Service
    Transaction,  // Транзакционное
    Promotion,    // Промоциональное
}

pub struct SendMessageResponse {
    pub message_id: String,
}
```

#### 2. Изображение

```
POST /message/cs  (с attachment)
```

**Request:**
```json
{
  "recipient": {
    "user_id": "USER_ID"
  },
  "message": {
    "attachment": {
      "type": "image",
      "payload": {
        "url": "https://example.com/image.jpg",
        "caption": "Описание изображения"
      }
    }
  },
  "type": "cs"
}
```

**Ограничения:**
- Размер: до 10 MB
- Форматы: JPG, PNG, GIF
- URL должен быть HTTPS и публичным

#### 3. Файл

```json
{
  "recipient": { "user_id": "USER_ID" },
  "message": {
    "attachment": {
      "type": "file",
      "payload": {
        "url": "https://example.com/doc.pdf",
        "filename": "document.pdf"
      }
    }
  },
  "type": "transaction"
}
```

**Ограничения:**
- Размер: до 50 MB
- Форматы: PDF, DOC, DOCX, XLS, XLSX

#### 4. Шаблон (Template)

```json
{
  "recipient": { "user_id": "USER_ID" },
  "message": {
    "attachment": {
      "type": "template",
      "payload": {
        "template_id": "ORDER_CONFIRM_001",
        "template_data": {
          "order_id": "12345",
          "customer_name": "Иван Иванов",
          "total_amount": "500000"
        }
      }
    }
  },
  "type": "transaction"
}
```

**Требования:**
- Шаблон должен быть создан в Zalo OA Manager
- Требуется модерация и одобрение Zalo

#### 5. Список (List Message)

```json
{
  "recipient": { "user_id": "USER_ID" },
  "message": {
    "attachment": {
      "type": "template",
      "payload": {
        "template_type": "list",
        "header": {
          "title": "Выберите товар",
          "subtitle": "Доступные категории"
        },
        "elements": [
          {
            "title": "Товар 1",
            "subtitle": "Описание товара 1",
            "image_url": "https://...",
            "buttons": [
              {
                "type": "url",
                "title": "Купить",
                "url": "https://shop.com/item/1"
              }
            ]
          }
        ],
        "buttons": [
          {
            "type": "url",
            "title": "Показать ещё",
            "url": "https://shop.com/more"
          }
        ]
      }
    }
  },
  "type": "cs"
}
```

---

## Управление подписчиками

### 1. Получение профиля пользователя

```
GET /user/detail?user_id=USER_ID
```

**Response:**
```json
{
  "error": 0,
  "message": "Success",
  "data": {
    "user_id": "1234567890",
    "display_name": "Nguyen Van A",
    "avatar": "https://avatar.zdn.vn/...",
    "is_following": true
  }
}
```

**Структуры Rust:**
```rust
pub struct UserProfile {
    pub user_id: String,
    pub display_name: String,
    pub avatar: String,
    pub is_following: bool,
}
```

### 2. Список подписчиков

```
GET /user/getlist?offset=0&count=50
```

**Параметры:**
- `offset` — смещение (0-based)
- `count` — количество (макс. 50)

**Response:**
```json
{
  "error": 0,
  "message": "Success",
  "data": {
    "followers": [
      {
        "user_id": "1234567890",
        "display_name": "Nguyen Van A",
        "avatar": "https://...",
        "is_following": true
      }
    ],
    "total": 1500
  }
}
```

**Структуры Rust:**
```rust
pub struct FollowerList {
    pub followers: Vec<UserProfile>,
    pub total: u64,
}

pub struct FollowerListQuery {
    pub offset: u64,
    pub count: u64,
}

impl FollowerListQuery {
    pub fn first_page(count: u64) -> Self {
        Self { offset: 0, count }
    }
    
    pub fn page_after(offset: u64, count: u64) -> Self {
        Self { offset, count }
    }
}
```

### 3. Обновление информации о подписчике

```
POST /user/update
```

**Request:**
```json
{
  "user_id": "USER_ID",
  "name": "Новое имя",
  "phone": "+84123456789",
  "email": "user@example.com",
  "address": "Ha Noi",
  "city": "Hanoi",
  "birthday": "01/01/1990"
}
```

---

## Теги

### 1. Список тегов OA

```
GET /tag/gettagsofoa?page=1&page_size=20
```

**Параметры:**
- `page` — номер страницы (default: 1)
- `page_size` — количество (default: 10, max: 100)

**Response:**
```json
{
  "error": 0,
  "message": "Success",
  "data": {
    "tags": [
      {
        "id": "123456",
        "name": "VIP Customer",
        "follower_count": 150
      }
    ],
    "total": 5,
    "page": 1,
    "page_size": 20
  }
}
```

**Структуры Rust:**
```rust
pub struct TagInfo {
    pub id: String,
    pub name: String,
    pub follower_count: u64,
}

pub struct TagList {
    pub tags: Vec<TagInfo>,
    pub total: u64,
    pub page: u64,
    pub page_size: u64,
}
```

### 2. Добавление тега пользователю

```
POST /tag/tagfollower
```

**Request:**
```json
{
  "tag_id": "123456",
  "uids": ["user_id_1", "user_id_2", "user_id_3"]
}
```

**Ограничения:**
- Макс. 100 пользователей за один запрос

**Response:**
```json
{
  "error": 0,
  "message": "Success",
  "data": {
    "success_count": 3,
    "failures": []
  }
}
```

### 3. Удаление тега у пользователя

```
POST /tag/rmfollowerfromtag
```

**Request:**
```json
{
  "tag_id": "123456",
  "uids": ["user_id_1", "user_id_2"]
}
```

**Структуры Rust:**
```rust
pub struct TagFollowerRequest {
    pub tag_id: String,
    pub uids: Vec<String>,
}

pub struct TagOperationResponse {
    pub success_count: u64,
    pub failures: Vec<TagFailure>,
}

pub struct TagFailure {
    pub user_id: String,
    pub error_code: i64,
    pub message: String,
}
```

---

## Медиа

### Загрузка изображений

```
POST /upload/image
Content-Type: multipart/form-data
```

**Параметры:**
- `file` — файл изображения (JPG, PNG)

**Ограничения:**
- Размер: до 1 MB
- Форматы: JPG, PNG

**Response:**
```json
{
  "error": 0,
  "message": "Success",
  "data": {
    "url": "https://zalo.me/...",
    "file_id": "abc123xyz"
  }
}
```

### Загрузка файлов

```
POST /upload/file
Content-Type: multipart/form-data
```

**Ограничения:**
- Размер: до 5 MB
- Форматы: PDF, DOC, DOCX, XLS, XLSX

### Загрузка GIF

```
POST /upload/gif
Content-Type: multipart/form-data
```

**Ограничения:**
- Размер: до 1 MB

**Структуры Rust:**
```rust
pub struct MediaUploadResponse {
    pub url: String,
    pub file_id: String,
}

pub enum MediaType {
    Image,
    File,
    Gif,
}
```

---

## Вебхуки

### URL вебхука

Настраивается в Zalo OA Manager → Settings → Webhook

### Формат payload

```json
{
  "app_id": "APP_ID",
  "sender": {
    "id": "USER_ID",
    "name": "User Name"
  },
  "recipient": {
    "id": "OA_ID"
  },
  "event_name": "follow",
  "timestamp": 1708435200,
  "message": {
    "type": "text",
    "text": "Привет"
  },
  "mac": "HMAC_SHA256_SIGNATURE"
}
```

### События

#### User Lifecycle
- `follow` — пользователь подписался
- `unfollow` — пользователь отписался

#### Messaging
- `user_send_text` — текстовое сообщение
- `user_send_image` — изображение
- `user_send_file` — файл
- `user_send_sticker` — стикер
- `user_send_gif` — GIF
- `user_send_location` — геолокация

#### Interactions
- `user_click_link` — клик по ссылке
- `user_click_button` — клик по кнопке
- `user_received_message` — сообщение доставлено
- `user_seen_message` — сообщение прочитано

### MAC-подпись

**Алгоритм проверки:**

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub struct WebhookVerifier {
    secret: Vec<u8>,
}

impl WebhookVerifier {
    pub fn new(secret: impl AsRef<[u8]>) -> Result<Self, SignatureError> {
        let secret_bytes = secret.as_ref();
        HmacSha256::new_from_slice(secret_bytes)?;
        Ok(Self { secret: secret_bytes.to_vec() })
    }

    pub fn verify(&self, payload: &[u8], signature: &str) -> Result<(), SignatureError> {
        let mut mac = HmacSha256::new_from_slice(&self.secret)?;
        mac.update(payload);
        mac.verify_str(signature)?;
        Ok(())
    }
}
```

**Структуры Rust:**
```rust
pub struct WebhookEvent {
    pub app_id: String,
    pub sender: SenderInfo,
    pub recipient: RecipientInfo,
    pub event_name: WebhookEventType,
    pub timestamp: u64,
    pub message: Option<WebhookMessage>,
    pub mac: String,
}

pub enum WebhookEventType {
    Follow,
    Unfollow,
    UserSendText,
    UserSendImage,
    UserSendFile,
    UserSendSticker,
    UserSendGif,
    UserSendLocation,
    UserClickLink,
    UserClickButton,
    UserReceivedMessage,
    UserSeenMessage,
}

pub struct WebhookMessage {
    pub r#type: String,
    pub text: Option<String>,
    pub attachment_id: Option<String>,
}
```

---

## Mini App SDK

### Категории API

1. **User** — аутентификация, профиль
2. **Basic** — информация о приложении, устройстве
3. **Routing** — навигация, закрытие
4. **Storage** — кэш, ключ-значение
5. **UI** — интерфейс, уведомления
6. **Location** — геолокация
7. **Media** — камера, фото, файлы
8. **Device** — сеть, контакты, NFC
9. **Permission** — разрешения
10. **Zalo** — интеграция с Zalo
11. **Advertising** — реклама
12. **Widgets** — виджеты OA
13. **Events** — события приложения

### User API

#### authorize

```typescript
import { authorize } from "zmp-sdk/apis";

const result = await authorize({
  scopes: ["user_info", "phone_number"]
});
```

**Структуры Rust:**
```rust
pub struct AuthorizeRequest {
    pub app_id: String,
    pub scopes: Vec<String>,
    pub redirect_uri: String,
}

pub struct AuthorizeResponse {
    pub auth_code: String,
    pub state: Option<String>,
}
```

#### getUserInfo

```typescript
import { getUserInfo } from "zmp-sdk/apis";

const userInfo = await getUserInfo();
```

**Response:**
```json
{
  "user_id": "1234567890",
  "display_name": "Nguyen Van A",
  "avatar": "https://...",
  "gender": 1,
  "birthday": "01/01/1990"
}
```

**Структуры Rust:**
```rust
pub struct UserInfo {
    pub user_id: String,
    pub display_name: String,
    pub avatar: String,
    pub gender: Gender,
    pub birthday: Option<Birthday>,
}

pub enum Gender {
    Male = 1,
    Female = 2,
    Other = 3,
}

pub struct Birthday {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}
```

#### getPhoneNumber

```typescript
import { getPhoneNumber } from "zmp-sdk/apis";

const phone = await getPhoneNumber();
```

**Структуры Rust:**
```rust
pub struct GetPhoneNumberRequest {
    pub app_id: String,
}

pub struct PhoneNumberResponse {
    pub phone_number: String,
    pub status: PhoneNumberStatus,
}

pub enum PhoneNumberStatus {
    Verified,
    Unverified,
}
```

### Storage API

#### setItem / getItem

```typescript
import { setItem, getItem } from "zmp-sdk/apis";

await setItem("user_prefs", "dark_mode");
const value = await getItem("user_prefs");
```

**Ограничения:**
- Ключ: макс. 128 байт
- Значение: макс. 4096 байт

**Структуры Rust:**
```rust
pub struct SetStorageRequest {
    pub key: StorageKey,
    pub value: StorageValue,
}

pub struct GetStorageRequest {
    pub key: StorageKey,
}

pub struct GetStorageResponse {
    pub key: String,
    pub value: Option<String>,
}

pub struct StorageKey(String);  // макс. 128 байт

impl StorageKey {
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SdkError::StorageKeyEmpty);
        }
        if value.len() > MAX_KEY_LEN {
            return Err(SdkError::StorageKeyTooLong {
                length: value.len(),
                maximum: MAX_KEY_LEN,
            });
        }
        Ok(Self(value))
    }
}

pub struct StorageValue(String);  // макс. 4096 байт
```

### Payment API

```typescript
import { payment } from "zmp-sdk/apis";

const result = await payment.checkout({
  order_id: "ORDER_123",
  amount: 500000,
  description: "Оплата заказа",
});
```

**Структуры Rust:**
```rust
pub struct CheckoutRequest {
    pub order_id: OrderId,
    pub amount: Amount,
    pub description: String,
}

pub struct OrderId(String);

pub struct Amount(u64);  // в VND

pub struct CheckoutResponse {
    pub status: PaymentStatus,
    pub transaction_id: Option<String>,
}

pub enum PaymentStatus {
    Success,
    Cancelled,
    Failed,
    Pending,
}
```

### Location API

```typescript
import { getLocation } from "zmp-sdk/apis";

const location = await getLocation({
  accuracy: "high"
});
```

**Структуры Rust:**
```rust
pub struct GetLocationRequest {
    pub accuracy: LocationAccuracy,
}

pub enum LocationAccuracy {
    Low,
    Balanced,
    High,
}

pub struct LocationResponse {
    pub coordinates: Coordinates,
    pub timestamp: u64,
}

pub struct Coordinates {
    pub latitude: f64,   // -90..=90
    pub longitude: f64,  // -180..=180
}
```

### Navigation API

#### openWebview

```typescript
import { openWebview } from "zmp-sdk/apis";

await openWebview({
  url: "https://example.com",
  title: "External Page",
});
```

**Структуры Rust:**
```rust
pub struct OpenWebviewRequest {
    pub url: String,
    pub title: String,
}

pub struct NavigateRequest {
    pub path: RoutePath,
}

pub struct RoutePath(String);  // начинается с /

impl RoutePath {
    pub fn new(path: impl Into<String>) -> SdkResult<Self> {
        let path = path.into();
        if !path.starts_with('/') || path.len() < 2 {
            return Err(SdkError::InvalidRoutePath(path));
        }
        Ok(Self(path))
    }
}

pub struct SetTitleRequest {
    pub title: String,  // макс. 50 символов
}
```

### Share API

```typescript
import { share } from "zmp-sdk/apis";

const result = await share({
  title: "Check this out!",
  thumbnail: "https://...",
  message: "Interesting content",
});
```

**Структуры Rust:**
```rust
pub struct ShareRequest {
    pub title: String,
    pub thumbnail: String,
    pub message: Option<String>,
}

pub struct ShareResponse {
    pub was_shared: bool,
}
```

### Events API

```typescript
import { on, off } from "zmp-sdk/apis";

const unsubscribe = on("AppResumed", () => {
  console.log("App resumed");
});

// Later...
off(unsubscribe);
```

**Структуры Rust:**
```rust
pub enum AppLifecycleEvent {
    AppPaused,
    AppResumed,
    NetworkChanged,
    OnDataCallback,
    OpenApp,
}

pub struct LifecyclePayload {
    pub event: AppLifecycleEvent,
    pub timestamp: u64,
}
```

---

## Коды ошибок

### OAuth / Authentication

| Код | Значение |
|-----|----------|
| `-204` | Access Token недействителен или истёк |
| `-240` | API v2 устарел, требуется миграция на v3 |

### Permissions

| Код | Значение |
|-----|----------|
| `-205` | Недостаточно прав у OA |
| `-211` | OA не прошёл верификацию |

### Rate Limiting

| Код | Значение |
|-----|----------|
| `-210` | Превышен лимит запросов (10 req/s) |

### User Restrictions

| Код | Значение |
|-----|----------|
| `-213` | Пользователь не подписан на OA |
| `-214` | Сообщение вне 24-часового окна |

### Content Policy

| Код | Значение |
|-----|----------|
| `-215` | Контент нарушает политику |
| `-216` | Дубликат сообщения |

### Validation

| Код | Значение |
|-----|----------|
| `-201` | Отсутствуют обязательные параметры |
| `-202` | Неверные значения параметров |

---

## План реализации

### Приоритет 1 (Core)

- [ ] **OAuth Client** — получение/обновление токенов
- [ ] **Token Manager** — хранение, ротация, refresh
- [ ] **Message Client** — отправка text/image/file
- [ ] **Webhook Handler** — проверка MAC, парсинг событий
- [ ] **Error Handler** — маппинг кодов ошибок

### Приоритет 2 (User Management)

- [ ] **Profile API** — getprofile, getfollowers
- [ ] **Tag API** — getTagsOfOA, tagFollower, rmFollowerFromTag
- [ ] **Update Follower** — updatefollowerinfo

### Приоритет 3 (Media)

- [ ] **Upload Client** — multipart/form-data загрузка
- [ ] **Media Storage** — кэширование file_id
- [ ] **Attachment Helper** — конвертация в message payload

### Приоритет 4 (Mini App SDK)

- [ ] **Auth Module** — authorize, getUserInfo, getPhoneNumber
- [ ] **Storage Module** — setItem, getItem, removeItem
- [ ] **Payment Module** — checkout, payment status
- [ ] **Navigation Module** — openWebview, closeApp
- [ ] **Location Module** — getLocation
- [ ] **Share Module** — openShareSheet

### Приоритет 5 (Advanced)

- [ ] **Template Builder** — конструктор шаблонов
- [ ] **List Message Builder** — списки с кнопками
- [ ] **Conversation API** — listrecentchat, conversation
- [ ] **Article API** — create, upload_video, verify
- [ ] **Store API** — product, order management

### Приоритет 6 (Observability)

- [ ] **Rate Limiter** — token bucket, 10 req/s
- [ ] **Retry Logic** — exponential backoff
- [ ] **Metrics** — запросы, ошибки, latency
- [ ] **Logging** — structured logs, tracing

---

## Взаимодействие компонентов

```
┌─────────────────────────────────────────────────────────────────┐
│                     Zalo Bot Platform                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐         │
│  │   Bot App   │    │  Mini App   │    │  Web Server │         │
│  │  (CLI/TUI)  │    │  (WASM)     │    │  (Axum)     │         │
│  └──────┬──────┘    └──────┬──────┘    └──────┬──────┘         │
│         │                  │                  │                 │
│         │                  │                  │                 │
│         ▼                  ▼                  ▼                 │
│  ┌─────────────────────────────────────────────────────┐       │
│  │              zalo-bot (utilities)                   │       │
│  │  • init_tracing                                     │       │
│  │  • WebhookVerifier                                  │       │
│  │  • ConfigLoader                                     │       │
│  └─────────────────────────────────────────────────────┘       │
│         │                  │                  │                 │
│         ▼                  ▼                  ▼                 │
│  ┌─────────────────────────────────────────────────────┐       │
│  │              zalo-sdk (Mini App)                    │       │
│  │  • MiniAppContext                                   │       │
│  │  • Auth, Storage, Payment, Navigation               │       │
│  └─────────────────────────────────────────────────────┘       │
│         │                                                       │
│         ▼                                                       │
│  ┌─────────────────────────────────────────────────────┐       │
│  │              zalo-http (OA API Client)              │       │
│  │  • OaClient                                         │       │
│  │  • Message, User, Tag, Media endpoints              │       │
│  │  • Token management                                 │       │
│  └─────────────────────────────────────────────────────┘       │
│         │                                                       │
│         ▼                                                       │
│  ┌─────────────────────────────────────────────────────┐       │
│  │              zalo-types (shared)                    │       │
│  │  • AppConfig, Environment                           │       │
│  │  • TypesError, BotError, SdkError                   │       │
│  │  • Request/Response models                          │       │
│  └─────────────────────────────────────────────────────┘       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
                           │
                           ▼
                  ┌─────────────────┐
                  │  Zalo OA API    │
                  │  v3.0           │
                  └─────────────────┘
```

---

## Примеры использования

### Отправка сообщения

```rust
use zalo_http::client::OaClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = OaClient::new("YOUR_ACCESS_TOKEN")?;
    
    // Текстовое сообщение
    let msg_id = client
        .send_text_message("USER_ID", "Привет!")
        .await?;
    
    println!("Отправлено: {}", msg_id);
    
    Ok(())
}
```

### Обработка вебхука

```rust
use zalo_bot::{WebhookVerifier, init_tracing};
use zalo_types::ConfigLoader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ConfigLoader::default().load()?;
    init_tracing(&config)?;
    
    let verifier = WebhookVerifier::new("WEBHOOK_SECRET")?;
    
    // В обработчике HTTP POST
    let body = /* raw body */;
    let signature = /* from header */;
    
    verifier.verify(&body, signature)?;
    
    let event: WebhookEvent = serde_json::from_slice(&body)?;
    
    match event.event_name {
        WebhookEventType::Follow => {
            println!("Новый подписчик: {}", event.sender.id);
        }
        WebhookEventType::UserSendText => {
            if let Some(msg) = event.message {
                println!("Сообщение: {}", msg.text.unwrap());
            }
        }
        _ => {}
    }
    
    Ok(())
}
```

### Mini App контекст

```rust
use zalo_sdk::{MiniAppContext, auth::AuthorizeRequest};

fn init_mini_app() -> Result<(), Box<dyn std::error::Error>> {
    let context = MiniAppContext::new("my-app-id", "my-oa-id")?;
    
    let payload = context.handshake_payload();
    
    // Serialise и отправить хосту
    let json = serde_json::to_string(&payload)?;
    
    Ok(())
}
```

---

## Ссылки

- [Zalo Developers](https://developers.zalo.me/)
- [Official Account API v3](https://developers.zalo.me/docs/official-account)
- [Mini App SDK](https://mini.zalo.me/documents/api/)
- [OAuth 2.0 Guide](https://developers.zalo.me/docs/oauth)

---

*Документ обновлён: Февраль 2026*
*Версия API: v3.0*
*SDK Version: 2.35.0+*
