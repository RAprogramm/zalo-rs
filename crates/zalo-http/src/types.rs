// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Envelope wrapping every Zalo OA API response.
///
/// A response is considered successful when [`error`](Self::error) is `0`.
/// Callers should use [`is_ok`](Self::is_ok) before extracting
/// [`data`](Self::data).
///
/// # Examples
///
/// ```rust
/// use zalo_http::types::ApiResponse;
///
/// let json = r#"{"error":0,"message":"Success","data":{"message_id":"abc"}}"#;
/// let response: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
/// assert!(response.is_ok());
/// ```
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// Zero on success, negative on failure.
    pub error:   i64,
    /// Human-readable status description from the platform.
    pub message: String,
    /// Payload present when [`error`](Self::error) is `0`.
    pub data:    Option<T>
}

impl<T> ApiResponse<T> {
    /// Returns `true` when the response indicates success.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::types::ApiResponse;
    ///
    /// let ok: ApiResponse<()> = ApiResponse {
    ///     error:   0,
    ///     message: "Success".to_owned(),
    ///     data:    None
    /// };
    /// assert!(ok.is_ok());
    ///
    /// let fail: ApiResponse<()> = ApiResponse {
    ///     error:   -204,
    ///     message: "Expired".to_owned(),
    ///     data:    None
    /// };
    /// assert!(!fail.is_ok());
    /// ```
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.error == 0
    }
}

/// Recipient descriptor for outbound messages.
///
/// # Examples
///
/// ```rust
/// use zalo_http::types::Recipient;
///
/// let r = Recipient::for_user("user-123");
/// assert_eq!(r.user_id, "user-123");
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct Recipient {
    /// Zalo user identifier of the message recipient.
    pub user_id: String
}

impl Recipient {
    /// Creates a recipient targeting the given user identifier.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::types::Recipient;
    ///
    /// let r = Recipient::for_user("u-42");
    /// assert_eq!(r.user_id, "u-42");
    /// ```
    #[must_use]
    pub fn for_user(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into()
        }
    }
}

/// Message type selector as defined by the Zalo OA API.
///
/// The choice of type determines delivery constraints and permissible content.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// Customer Service message — usable within 24 hours of the last user
    /// interaction.
    Cs,
    /// Transactional notification — delivery not restricted to the 24-hour
    /// window.
    Transaction,
    /// Promotional broadcast — subject to additional OA policy rules.
    Promotion
}

/// Request body for sending a plain-text message.
///
/// # Examples
///
/// ```rust
/// use zalo_http::types::{MessageType, Recipient, SendTextRequest};
///
/// let req = SendTextRequest::new("user-1", "Hello!", MessageType::Cs);
/// assert_eq!(req.recipient.user_id, "user-1");
/// ```
#[derive(Debug, Serialize)]
pub struct SendTextRequest {
    /// Message recipient.
    pub recipient:    Recipient,
    /// Inner message payload.
    pub message:      TextPayload,
    /// Message type controlling delivery constraints.
    #[serde(rename = "type")]
    pub message_type: MessageType
}

impl SendTextRequest {
    /// Constructs a new text message request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::types::{MessageType, SendTextRequest};
    ///
    /// let req = SendTextRequest::new("uid", "body", MessageType::Cs);
    /// assert_eq!(req.message.text, "body");
    /// ```
    #[must_use]
    pub fn new(
        user_id: impl Into<String>,
        text: impl Into<String>,
        message_type: MessageType
    ) -> Self {
        Self {
            recipient: Recipient::for_user(user_id),
            message: TextPayload {
                text: text.into()
            },
            message_type
        }
    }
}

/// Inner payload carrying the text content of a message.
#[derive(Clone, Debug, Serialize)]
pub struct TextPayload {
    /// UTF-8 text body of the message.
    pub text: String
}

/// Response returned when a message is delivered successfully.
///
/// # Examples
///
/// ```rust
/// use zalo_http::types::SendMessageResponse;
///
/// let json = r#"{"message_id":"msg-xyz"}"#;
/// let r: SendMessageResponse = serde_json::from_str(json).unwrap();
/// assert_eq!(r.message_id, "msg-xyz");
/// ```
#[derive(Debug, Deserialize)]
pub struct SendMessageResponse {
    /// Platform-assigned identifier for the delivered message.
    pub message_id: String
}

/// Subscriber (follower) profile returned by the OA API.
///
/// # Examples
///
/// ```rust
/// use zalo_http::types::UserProfile;
///
/// let json = r#"{"user_id":"u1","display_name":"Alice","avatar":"https://example.com/a.jpg","is_following":true}"#;
/// let p: UserProfile = serde_json::from_str(json).unwrap();
/// assert_eq!(p.display_name, "Alice");
/// ```
#[derive(Clone, Debug, Deserialize)]
pub struct UserProfile {
    /// Unique Zalo user identifier.
    pub user_id:      String,
    /// Display name shown to the user.
    pub display_name: String,
    /// URL of the user's avatar image.
    pub avatar:       String,
    /// Whether the user is currently following the OA.
    pub is_following: bool
}

/// Paginated list of followers returned by the OA API.
///
/// # Examples
///
/// ```rust
/// use zalo_http::types::FollowerList;
///
/// let json = r#"{"followers":[{"user_id":"u1","display_name":"A","avatar":"url","is_following":true}],"total":1}"#;
/// let list: FollowerList = serde_json::from_str(json).unwrap();
/// assert_eq!(list.total, 1);
/// ```
#[derive(Debug, Deserialize)]
pub struct FollowerList {
    /// Profiles of followers in this page.
    pub followers: Vec<UserProfile>,
    /// Total number of followers across all pages.
    pub total:     u64
}

/// Query parameters for paginating follower lists.
#[derive(Debug, Serialize)]
pub struct FollowerListQuery {
    /// Zero-based page offset.
    pub offset: u64,
    /// Maximum number of records to return (platform maximum: 50).
    pub count:  u64
}

impl FollowerListQuery {
    /// Creates a new query targeting the first page with the given page size.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::types::FollowerListQuery;
    ///
    /// let q = FollowerListQuery::first_page(10);
    /// assert_eq!(q.offset, 0);
    /// assert_eq!(q.count, 10);
    /// ```
    #[must_use]
    pub fn first_page(count: u64) -> Self {
        Self {
            offset: 0,
            count
        }
    }

    /// Creates a query for the next page following the given offset.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::types::FollowerListQuery;
    ///
    /// let q = FollowerListQuery::page_after(20, 10);
    /// assert_eq!(q.offset, 20);
    /// assert_eq!(q.count, 10);
    /// ```
    #[must_use]
    pub fn page_after(offset: u64, count: u64) -> Self {
        Self {
            offset,
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_response_is_ok_on_zero_code() {
        let response: ApiResponse<()> = ApiResponse {
            error:   0,
            message: "Success".to_owned(),
            data:    None
        };
        assert!(response.is_ok());
    }

    #[test]
    fn api_response_is_not_ok_on_negative_code() {
        let response: ApiResponse<()> = ApiResponse {
            error:   -204,
            message: "Expired".to_owned(),
            data:    None
        };
        assert!(!response.is_ok());
    }

    #[test]
    fn recipient_for_user_sets_user_id() {
        let r = Recipient::for_user("u-99");
        assert_eq!(r.user_id, "u-99");
    }

    #[test]
    fn send_text_request_sets_all_fields() {
        let req = SendTextRequest::new("u-1", "hello", MessageType::Cs);
        assert_eq!(req.recipient.user_id, "u-1");
        assert_eq!(req.message.text, "hello");
        assert!(matches!(req.message_type, MessageType::Cs));
    }

    #[test]
    fn follower_list_query_first_page() {
        let q = FollowerListQuery::first_page(25);
        assert_eq!(q.offset, 0);
        assert_eq!(q.count, 25);
    }

    #[test]
    fn follower_list_query_page_after() {
        let q = FollowerListQuery::page_after(50, 25);
        assert_eq!(q.offset, 50);
        assert_eq!(q.count, 25);
    }

    #[test]
    fn send_message_response_deserialises() {
        let json = r#"{"message_id":"xyz"}"#;
        let r: SendMessageResponse = serde_json::from_str(json).expect("deserialise");
        assert_eq!(r.message_id, "xyz");
    }

    #[test]
    fn user_profile_deserialises() {
        let json = r#"{"user_id":"u1","display_name":"Alice","avatar":"https://a.example.com/a.jpg","is_following":true}"#;
        let p: UserProfile = serde_json::from_str(json).expect("deserialise");
        assert_eq!(p.user_id, "u1");
        assert!(p.is_following);
    }

    #[test]
    fn follower_list_deserialises() {
        let json = r#"{"followers":[{"user_id":"u1","display_name":"A","avatar":"url","is_following":true}],"total":1}"#;
        let list: FollowerList = serde_json::from_str(json).expect("deserialise");
        assert_eq!(list.total, 1);
        assert_eq!(list.followers.len(), 1);
    }

    #[test]
    fn api_response_envelope_deserialises() {
        let json = r#"{"error":0,"message":"Success","data":{"message_id":"msg-abc"}}"#;
        let r: ApiResponse<SendMessageResponse> = serde_json::from_str(json).expect("deserialise");
        assert!(r.is_ok());
        assert_eq!(r.data.expect("data present").message_id, "msg-abc");
    }
}
