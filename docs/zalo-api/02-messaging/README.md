# Сообщения

Отправка сообщений.

**Реализация:** [`crates/zalo-http/src/client.rs`](../../crates/zalo-http/src/client.rs)

---

## Типы

| Тип | Ограничения |
|-----|-------------|
| `cs` | 24-часовое окно |
| `transaction` | Без ограничений |
| `promotion` | Требуется верификация |

---

## Текстовое сообщение

**Файл:** [`client.rs`](../../crates/zalo-http/src/client.rs#L79-L101)

```rust
let client = OaClient::new("TOKEN")?;
let msg_id = client
    .send_text_message("USER_ID", "Привет!")
    .await?;
```

**Структуры:** [`types.rs`](../../crates/zalo-http/src/types.rs#L89-L110)

---

## Медиа

**Статус:** В разработке

```rust
// Планируется
client.send_image_message("USER_ID", file_id, caption).await?;
client.send_file_message("USER_ID", file_id, filename).await?;
```

---

## Ошибки

| Код | Значение |
|-----|----------|
| `-214` | Вне 24h окна |
| `-210` | Лимит 10 req/s |

---

[← Auth](01-auth/README.md) | [Users →](03-users/README.md)
