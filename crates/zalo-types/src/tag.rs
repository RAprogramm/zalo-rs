// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Tag management types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Tag information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagInfo {
    /// Tag ID.
    pub id: String,
    /// Tag name.
    pub name: String,
    /// Number of followers with this tag.
    pub follower_count: u64,
}

/// List of tags response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagList {
    /// List of tags.
    pub tags: Vec<TagInfo>,
    /// Total number of tags.
    pub total: u64,
    /// Current page number.
    pub page: u64,
    /// Page size.
    pub page_size: u64,
}

/// Query parameters for listing tags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagListQuery {
    /// Page number (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u64>,
    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u64>,
}

/// Request to tag/untag followers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagFollowerRequest {
    /// Tag ID.
    pub tag_id: String,
    /// List of user IDs (max 100).
    pub uids: Vec<String>,
}

/// Response from tag operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagOperationResponse {
    /// Number of successfully processed users.
    pub success_count: u64,
    /// List of failures.
    #[serde(default)]
    pub failures: Vec<TagFailure>,
}

/// Tag operation failure for a specific user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagFailure {
    /// User ID that failed.
    pub user_id: String,
    /// Error code.
    pub error_code: i64,
    /// Error message.
    pub message: String,
}
