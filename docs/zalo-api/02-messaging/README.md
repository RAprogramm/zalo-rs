# Сообщения

Отправка сообщений через Zalo OA API.

**Реализация:** [`crates/zalo-http/src/client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

---

## Типы сообщений

| Тип | Ограничения | Статус |
|-----|-------------|--------|
| `cs` | 24-часовое окно | ✅ |
| `transaction` | Без ограничений | ✅ |
| `promotion` | Требуется верификация | ✅ |

**Структура:** [`message.rs:28-38`](../../crates/zalo-types/src/message.rs#L28-L38)

---

## ✅ Текстовое сообщение

**Методы:**
- `send_text_message()` — [`client.rs:51-56`](../../crates/zalo-http/src/client_inner/client.rs#L51-L56)
- `send_typed_text_message()` — [`client.rs:58-72`](../../crates/zalo-http/src/client_inner/client.rs#L58-L72)

```rust
let client = OaClient::new("TOKEN")?;
let msg_id = client
    .send_text_message("USER_ID", "Привет!")
    .await?;
```

**Структуры:**
- `SendTextRequest` — [`message.rs:48-71`](../../crates/zalo-types/src/message.rs#L48-L71)
- `TextPayload` — [`message.rs:42-45`](../../crates/zalo-types/src/message.rs#L42-L45)
- `SendMessageResponse` — [`message.rs:217-221`](../../crates/zalo-types/src/message.rs#L217-L221)

---

## ✅ Изображение

**Методы:**
- `send_image_message()` — [`client.rs:78-85`](../../crates/zalo-http/src/client_inner/client.rs#L78-L85)
- `send_typed_image_message()` — [`client.rs:87-102`](../../crates/zalo-http/src/client_inner/client.rs#L87-L102)

```rust
let msg_id = client
    .send_image_message("USER_ID", "https://...", Some("caption"))
    .await?;
```

**Структуры:**
- `SendImageRequest` — [`image.rs:28-56`](../../crates/zalo-types/src/image.rs#L28-L56)
- `ImagePayload` — [`image.rs:20-24`](../../crates/zalo-types/src/image.rs#L20-L24)
- `ImageAttachment` — [`image.rs:11-17`](../../crates/zalo-types/src/image.rs#L11-L17)

---

## ✅ Файл

**Методы:**
- `send_file_message()` — [`client.rs:104-111`](../../crates/zalo-http/src/client_inner/client.rs#L104-L111)
- `send_typed_file_message()` — [`client.rs:113-128`](../../crates/zalo-http/src/client_inner/client.rs#L113-L128)

```rust
let msg_id = client
    .send_file_message("USER_ID", "https://...", "doc.pdf")
    .await?;
```

**Структуры:**
- `SendFileRequest` — [`message.rs:78-101`](../../crates/zalo-types/src/message.rs#L78-L101)
- `FilePayload` — [`message.rs:73-76`](../../crates/zalo-types/src/message.rs#L73-L76)

---

## ✅ Шаблон (Template)

**Методы:**
- `send_template_message()` — [`client.rs:130-142`](../../crates/zalo-http/src/client_inner/client.rs#L130-L142)
- `send_template_message_with_elements()` — [`client.rs:144-160`](../../crates/zalo-http/src/client_inner/client.rs#L144-L160)

```rust
use zalo_types::TemplateElement;

let elements = vec![
    TemplateElement { /* ... */ }
];

let msg_id = client
    .send_template_message_with_elements("USER_ID", "list", MessageType::Cs, elements)
    .await?;
```

**Структуры:**
- `SendTemplateRequest` — [`message.rs:162-201`](../../crates/zalo-types/src/message.rs#L162-L201)
- `TemplatePayload` — [`message.rs:147-159`](../../crates/zalo-types/src/message.rs#L147-L159)
- `TemplateElement` — [`message.rs:131-144`](../../crates/zalo-types/src/message.rs#L131-L144)
- `TemplateButton` — [`message.rs:120-128`](../../crates/zalo-types/src/message.rs#L120-L128)

---

## Ошибки

| Код | Значение | Обработка |
|-----|----------|-----------|
| `-214` | Вне 24h окна | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |
| `-210` | Лимит 10 req/s | [`error.rs:75`](../../crates/zalo-http/src/error.rs#L75) |
| `-204` | Токен истёк | [`error.rs:74`](../../crates/zalo-http/src/error.rs#L74) |

---

[← Auth](01-auth/README.md) | [Users →](03-users/README.md)
