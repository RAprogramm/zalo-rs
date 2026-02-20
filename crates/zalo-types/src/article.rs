// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Article types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Article status.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ArticleStatus {
    /// Draft article.
    Draft,
    /// Article pending verification.
    Pending,
    /// Published article.
    Published,
    /// Rejected article.
    Rejected,
}

/// Article draft.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleDraft {
    /// Article token/ID.
    pub token: String,
    /// Article title.
    pub title: String,
    /// Article description.
    pub description: String,
    /// Author name.
    pub author: String,
    /// Cover image URL.
    pub cover: String,
    /// Article body content.
    pub body: String,
    /// Article status.
    pub status: ArticleStatus,
    /// Whether comments are enabled.
    pub comment_enabled: bool,
    /// Creation timestamp.
    pub created_at: u64,
    /// Last update timestamp.
    pub updated_at: u64,
}

/// Request to create an article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateArticleRequest {
    /// Article title.
    pub title: String,
    /// Article description.
    pub description: String,
    /// Author name.
    pub author: String,
    /// Cover image URL.
    pub cover: String,
    /// Article body content (HTML).
    pub body: String,
    /// Article status.
    pub status: ArticleStatus,
    /// Whether comments are enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_enabled: Option<bool>,
}

/// Video upload preparation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUploadPrepareResponse {
    /// Upload ID for tracking.
    pub upload_id: String,
    /// Upload URL for the video file.
    pub upload_url: String,
}

/// Request to prepare video upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUploadPrepareRequest {
    /// Video file name.
    pub video_name: String,
    /// Video file size in bytes.
    pub video_size: u64,
}

/// Request to verify video upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoUploadVerifyRequest {
    /// Upload ID from prepare response.
    pub upload_id: String,
}

/// Article verification response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleVerification {
    /// Article token.
    pub token: String,
    /// Article title.
    pub title: String,
    /// Article URL.
    pub url: String,
    /// Verification status.
    pub status: ArticleStatus,
    /// Rejection reason (if rejected).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejection_reason: Option<String>,
}

/// Query parameters for article verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleVerificationQuery {
    /// Article token.
    pub token: String,
}
