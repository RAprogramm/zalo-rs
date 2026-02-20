// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User management API endpoints.

use crate::client_inner::{endpoint_url, get_simple, get_with_query, post_json};
use crate::error::HttpResult;
use zalo_types::user::{FollowerList, FollowerListQuery, OaInfo, UpdateFollowerRequest, UserProfile};

/// Gets user profile.
pub async fn get_user_profile(
    token: &str,
    user_id: impl AsRef<str>,
) -> HttpResult<UserProfile> {
    let url = endpoint_url("user/detail")?;

    tracing::debug!(
        endpoint = %url,
        user_id = user_id.as_ref(),
        "fetching user profile"
    );

    get_with_query(token, url, &[("user_id", user_id.as_ref())]).await
}

/// Lists followers.
pub async fn list_followers(
    token: &str,
    query: FollowerListQuery,
) -> HttpResult<FollowerList> {
    let url = endpoint_url("user/getlist")?;

    tracing::debug!(
        endpoint = %url,
        offset = ?query.offset,
        count = ?query.count,
        "listing followers"
    );

    get_with_query(token, url, &query).await
}

/// Gets OA information.
pub async fn get_oa_info(token: &str) -> HttpResult<OaInfo> {
    let url = endpoint_url("getoa")?;

    tracing::debug!(
        endpoint = %url,
        "fetching OA information"
    );

    get_simple(token, url).await
}

/// Updates follower information.
pub async fn update_follower_info(
    token: &str,
    request: UpdateFollowerRequest,
) -> HttpResult<UserProfile> {
    let url = endpoint_url("user/update")?;

    tracing::debug!(
        endpoint = %url,
        user_id = %request.user_id,
        "updating follower info"
    );

    post_json(token, url, &request).await
}
