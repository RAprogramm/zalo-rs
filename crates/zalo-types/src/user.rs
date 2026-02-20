// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User-related types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// User profile information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// User ID.
    pub user_id: String,
    /// User's display name.
    pub display_name: String,
    /// User's avatar URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// User's gender.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    /// User's birthday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
}

/// Query parameters for listing followers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowerListQuery {
    /// Offset for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
}

/// List of followers response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowerList {
    /// List of user IDs.
    pub data: Vec<String>,
    /// Total number of followers.
    pub total: i32,
}
