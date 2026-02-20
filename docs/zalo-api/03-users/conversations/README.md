# Диалоги

История переписки.

**Реализация:** ✅ [`client_inner/client.rs`](../../crates/zalo-http/src/client_inner/client.rs)

---

## list_recent_chats

**Метод:** [`client_inner/client.rs:245-259`](../../crates/zalo-http/src/client_inner/client.rs#L245-L259)

```rust
use zalo_http::{OaClient, zalo_types::RecentChatQuery};

let client = OaClient::new("TOKEN")?;

let query = RecentChatQuery {
    offset: Some(0),
    count: Some(20),
};

let chats = client.list_recent_chats(query).await?;

for chat in &chats.chats {
    println!("{}: {}", chat.display_name, chat.last_message_preview);
}
```

**Endpoint:** `GET /v3.0/oa/user/listrecentchat`

**Структуры:**
- [`RecentChatList`](../../crates/zalo-types/src/conversation.rs#L28-L34)
- [`RecentChatQuery`](../../crates/zalo-types/src/conversation.rs#L37-L44)
- [`ConversationSummary`](../../crates/zalo-types/src/conversation.rs#L10-L25)

---

## get_conversation

**Метод:** [`client_inner/client.rs:261-279`](../../crates/zalo-http/src/client_inner/client.rs#L261-L279)

```rust
use zalo_http::{OaClient, zalo_types::ConversationQuery};

let client = OaClient::new("TOKEN")?;

let query = ConversationQuery {
    user_id: "USER_ID".to_string(),
    offset: Some(0),
    count: Some(50),
};

let messages = client.get_conversation(query).await?;

for msg in &messages.messages {
    println!("[{}] {}", msg.timestamp, msg.content);
}
```

**Endpoint:** `GET /v3.0/oa/user/conversation`

**Структуры:**
- [`ConversationHistory`](../../crates/zalo-types/src/conversation.rs#L62-L68)
- [`ConversationQuery`](../../crates/zalo-types/src/conversation.rs#L71-L79)
- [`ConversationMessage`](../../crates/zalo-types/src/conversation.rs#L47-L60)

---

[← Users](README.md) | [Tags →](../04-tags/README.md)
