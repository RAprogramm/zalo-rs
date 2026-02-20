// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Conversation API endpoints.

use crate::client_inner::{endpoint_url, get_with_query};
use crate::error::HttpResult;
use zalo_types::conversation::{ConversationHistory, ConversationQuery, RecentChatList, RecentChatQuery};

/// Lists recent chats.
pub async fn list_recent_chats(
    token: &str,
    query: RecentChatQuery,
) -> HttpResult<RecentChatList> {
    let url = endpoint_url("user/listrecentchat")?;

    tracing::debug!(
        endpoint = %url,
        offset = ?query.offset,
        count = ?query.count,
        "listing recent chats"
    );

    get_with_query(token, url, &query).await
}

/// Gets conversation history.
pub async fn get_conversation(
    token: &str,
    query: ConversationQuery,
) -> HttpResult<ConversationHistory> {
    let url = endpoint_url("user/conversation")?;

    tracing::debug!(
        endpoint = %url,
        user_id = %query.user_id,
        offset = ?query.offset,
        count = ?query.count,
        "fetching conversation"
    );

    get_with_query(token, url, &query).await
}
