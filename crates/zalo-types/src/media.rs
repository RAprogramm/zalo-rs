// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Media upload types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Media type for upload.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    /// Image (JPG, PNG).
    Image,
    /// File (PDF, DOC, DOCX, XLS, XLSX).
    File,
    /// GIF animation.
    Gif,
}

/// Response from media upload API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUploadResponse {
    /// Public URL of the uploaded media.
    pub url: String,
    /// File ID for referencing in messages.
    pub file_id: String,
}
