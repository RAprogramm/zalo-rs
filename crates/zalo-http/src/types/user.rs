// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User types.

use serde::{Deserialize, Serialize};

/// User profile.
#[derive(Clone, Debug, Deserialize)]
pub struct UserProfile {
    /// User ID.
    pub user_id: String,
    /// Display name.
    pub display_name: String,
    /// Avatar URL.
    pub avatar: String,
    /// Is following.
    pub is_following: bool,
}

/// Follower list.
#[derive(Debug, Deserialize)]
pub struct FollowerList {
    /// Followers.
    pub followers: Vec<UserProfile>,
    /// Total count.
    pub total: u64,
}

/// Follower list query.
#[derive(Debug, Serialize)]
pub struct FollowerListQuery {
    /// Offset.
    pub offset: u64,
    /// Count.
    pub count: u64,
}

impl FollowerListQuery {
    /// First page.
    #[must_use]
    pub fn first_page(count: u64) -> Self {
        Self { offset: 0, count }
    }

    /// Next page.
    #[must_use]
    pub fn page_after(offset: u64, count: u64) -> Self {
        Self { offset, count }
    }
}
