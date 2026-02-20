// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Tag management API endpoints.

use crate::client_inner::{endpoint_url, get_with_query, post_json};
use crate::error::HttpResult;
use zalo_types::tag::{TagFollowerRequest, TagList, TagListQuery, TagOperationResponse};

/// Lists tags.
pub async fn get_tags(
    token: &str,
    query: TagListQuery,
) -> HttpResult<TagList> {
    let url = endpoint_url("tag/gettagsofoa")?;

    tracing::debug!(
        endpoint = %url,
        page = ?query.page,
        page_size = ?query.page_size,
        "listing tags"
    );

    get_with_query(token, url, &query).await
}

/// Tags followers.
pub async fn tag_followers(
    token: &str,
    request: TagFollowerRequest,
) -> HttpResult<TagOperationResponse> {
    let url = endpoint_url("tag/tagfollower")?;

    tracing::debug!(
        endpoint = %url,
        tag_id = %request.tag_id,
        count = request.uids.len(),
        "tagging followers"
    );

    post_json(token, url, &request).await
}

/// Removes tag from followers.
pub async fn untag_followers(
    token: &str,
    request: TagFollowerRequest,
) -> HttpResult<TagOperationResponse> {
    let url = endpoint_url("tag/rmfollowerfromtag")?;

    tracing::debug!(
        endpoint = %url,
        tag_id = %request.tag_id,
        count = request.uids.len(),
        "removing tag from followers"
    );

    post_json(token, url, &request).await
}
