// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Media types.

use serde::{Deserialize, Serialize};

/// Media type for upload.
#[derive(Debug, Clone, Copy)]
pub enum UploadType {
    /// Image (JPG, PNG).
    Image,
    /// File (PDF, DOC, DOCX, XLS, XLSX).
    File,
    /// GIF animation.
    Gif,
}

impl UploadType {
    /// Returns endpoint suffix.
    #[must_use]
    pub fn endpoint(&self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::File => "file",
            Self::Gif => "gif",
        }
    }

    /// Returns MIME type.
    #[must_use]
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::Image => "image/jpeg",
            Self::File => "application/octet-stream",
            Self::Gif => "image/gif",
        }
    }

    /// Returns max size in bytes.
    #[must_use]
    pub fn max_size(&self) -> usize {
        match self {
            Self::Image | Self::Gif => 1024 * 1024, // 1 MB
            Self::File => 5 * 1024 * 1024,          // 5 MB
        }
    }
}

/// Response from media upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUploadResponse {
    /// Public URL of uploaded media.
    pub url: String,
    /// File ID for message references.
    pub file_id: String,
}
