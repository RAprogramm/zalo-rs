// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// Request to share content to Zalo chat or timeline.
///
/// Both title and thumbnail URL are required; the message is optional.
///
/// # Examples
///
/// ```
/// use zalo_sdk::share::ShareRequest;
///
/// let req = ShareRequest::new("Check this out!", "https://example.com/thumb.jpg")?;
/// assert_eq!(req.title, "Check this out!");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct ShareRequest {
    /// Title of the shared content.
    pub title:         String,
    /// URL of the thumbnail image.
    pub thumbnail_url: String,
    /// Optional accompanying message text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message:       Option<String>
}

impl ShareRequest {
    /// Constructs a share request.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidShareTitle`] when `title` is blank, or
    /// [`SdkError::InvalidUrl`] when `thumbnail_url` is blank.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::share::ShareRequest;
    ///
    /// let req = ShareRequest::new("Promo!", "https://img.example.com/promo.jpg")?;
    /// assert!(req.message.is_none());
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(title: impl Into<String>, thumbnail_url: impl Into<String>) -> SdkResult<Self> {
        let title = title.into();
        if title.trim().is_empty() {
            return Err(SdkError::InvalidShareTitle(title));
        }
        let thumbnail_url = thumbnail_url.into();
        if thumbnail_url.trim().is_empty() {
            return Err(SdkError::InvalidUrl(thumbnail_url));
        }
        Ok(Self {
            title,
            thumbnail_url,
            message: None
        })
    }

    /// Attaches an optional message to the share card.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::share::ShareRequest;
    ///
    /// let req = ShareRequest::new("Promo!", "https://img.example.com/p.jpg")?
    ///     .with_message("Don't miss it!");
    /// assert_eq!(req.message.as_deref(), Some("Don't miss it!"));
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// Outcome of a share action initiated by the user.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ShareResult {
    /// The user confirmed and sent the share.
    Shared,
    /// The user closed the share sheet without sending.
    Dismissed
}

/// Response returned after the share sheet is closed.
///
/// # Examples
///
/// ```
/// use zalo_sdk::share::{ShareResponse, ShareResult};
///
/// let json = r#"{"result":"shared"}"#;
/// let resp: ShareResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(resp.result, ShareResult::Shared);
/// ```
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ShareResponse {
    /// Outcome of the user interaction.
    pub result: ShareResult
}

impl ShareResponse {
    /// Returns `true` when the content was shared.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::share::{ShareResponse, ShareResult};
    ///
    /// let resp = ShareResponse {
    ///     result: ShareResult::Shared
    /// };
    /// assert!(resp.was_shared());
    /// ```
    #[must_use]
    pub fn was_shared(&self) -> bool {
        self.result == ShareResult::Shared
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn share_request_rejects_empty_title() {
        let err = ShareRequest::new("", "https://img.example.com/t.jpg").expect_err("empty title");
        assert!(matches!(err, SdkError::InvalidShareTitle(_)));
    }

    #[test]
    fn share_request_rejects_whitespace_title() {
        let err =
            ShareRequest::new("   ", "https://img.example.com/t.jpg").expect_err("whitespace");
        assert!(matches!(err, SdkError::InvalidShareTitle(_)));
    }

    #[test]
    fn share_request_rejects_empty_thumbnail() {
        let err = ShareRequest::new("Title", "").expect_err("empty url");
        assert!(matches!(err, SdkError::InvalidUrl(_)));
    }

    #[test]
    fn share_request_builds_without_message() {
        let req = ShareRequest::new("Title", "https://img.example.com/t.jpg").expect("valid");
        assert!(req.message.is_none());
    }

    #[test]
    fn share_request_attaches_message() {
        let req = ShareRequest::new("Title", "https://img.example.com/t.jpg")
            .expect("valid")
            .with_message("Hello!");
        assert_eq!(req.message.as_deref(), Some("Hello!"));
    }

    #[test]
    fn share_response_was_shared_true() {
        let resp = ShareResponse {
            result: ShareResult::Shared
        };
        assert!(resp.was_shared());
    }

    #[test]
    fn share_response_was_shared_false_on_dismiss() {
        let resp = ShareResponse {
            result: ShareResult::Dismissed
        };
        assert!(!resp.was_shared());
    }

    #[test]
    fn share_response_deserialises_shared() {
        let json = r#"{"result":"shared"}"#;
        let resp: ShareResponse = serde_json::from_str(json).expect("deserialize");
        assert_eq!(resp.result, ShareResult::Shared);
    }

    #[test]
    fn share_response_deserialises_dismissed() {
        let json = r#"{"result":"dismissed"}"#;
        let resp: ShareResponse = serde_json::from_str(json).expect("deserialize");
        assert_eq!(resp.result, ShareResult::Dismissed);
    }
}
