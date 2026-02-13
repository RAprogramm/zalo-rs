// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use std::time::Duration;

use reqwest::{Client as ReqwestClient, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use tracing::{debug, warn};

use crate::{
    error::{HttpError, HttpResult},
    types::{
        ApiResponse, FollowerList, FollowerListQuery, MessageType, SendMessageResponse,
        SendTextRequest, UserProfile
    }
};

const BASE_URL: &str = "https://openapi.zalo.me/v3.0/oa/";
const REQUEST_TIMEOUT_SECS: u64 = 15;
const ACCESS_TOKEN_HEADER: &str = "access_token";

/// Async HTTP client for the Zalo Official Account API v3.
///
/// The client holds a single [`reqwest::Client`] instance and attaches the
/// `access_token` header to every request. Use [`OaClient::new`] to construct
/// an instance after obtaining a valid token from the Zalo developer portal.
///
/// # Examples
///
/// ```rust,no_run
/// use zalo_http::client::OaClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = OaClient::new("YOUR_ACCESS_TOKEN")?;
///     let id = client.send_text_message("USER_ID", "hi").await?;
///     println!("{id}");
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct OaClient {
    inner: ReqwestClient,
    token: String
}

impl OaClient {
    /// Creates a new client authenticated with the given access token.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Configuration`] when the token is empty, or
    /// [`HttpError::Transport`] when the underlying HTTP client cannot be
    /// built.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use zalo_http::client::OaClient;
    ///
    /// let result = OaClient::new("");
    /// assert!(result.is_err());
    ///
    /// let result = OaClient::new("valid-token");
    /// assert!(result.is_ok());
    /// ```
    pub fn new(access_token: impl Into<String>) -> HttpResult<Self> {
        let token = access_token.into();
        if token.trim().is_empty() {
            return Err(HttpError::configuration("access token must not be empty"));
        }

        let inner = ReqwestClient::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .map_err(HttpError::from)?;

        Ok(Self {
            inner,
            token
        })
    }

    /// Sends a plain-text Customer Service message to the given user.
    ///
    /// Customer Service messages are only deliverable within 24 hours of the
    /// last user interaction. Use
    /// [`send_typed_text_message`](Self::send_typed_text_message)
    /// when a different delivery type is required.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`HttpError::Unauthorized`] when the token has expired (code
    ///   -204/-240).
    /// - [`HttpError::RateLimited`] when the OA quota is exhausted (code -210).
    /// - [`HttpError::Api`] for other platform-level errors.
    /// - [`HttpError::Transport`] on network failures.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::client::OaClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OaClient::new("TOKEN")?;
    ///     let msg_id = client.send_text_message("USER_ID", "Hello!").await?;
    ///     println!("{msg_id}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_text_message(
        &self,
        user_id: impl Into<String>,
        text: impl Into<String>
    ) -> HttpResult<String> {
        self.send_typed_text_message(user_id, text, MessageType::Cs)
            .await
    }

    /// Sends a text message with an explicitly chosen delivery type.
    ///
    /// See [`MessageType`] for the semantics of each variant.
    ///
    /// # Errors
    ///
    /// Same as [`send_text_message`](Self::send_text_message).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::{client::OaClient, types::MessageType};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OaClient::new("TOKEN")?;
    ///     let msg_id = client
    ///         .send_typed_text_message("USER_ID", "Notification", MessageType::Transaction)
    ///         .await?;
    ///     println!("{msg_id}");
    ///     Ok(())
    /// }
    /// ```
    pub async fn send_typed_text_message(
        &self,
        user_id: impl Into<String>,
        text: impl Into<String>,
        message_type: MessageType
    ) -> HttpResult<String> {
        let url = self.endpoint("message/cs")?;
        let body = SendTextRequest::new(user_id, text, message_type);

        debug!(endpoint = %url, "sending text message");

        let response: SendMessageResponse = self.post(url).json(&body).send_and_parse().await?;

        Ok(response.message_id)
    }

    /// Retrieves the profile of a subscriber by their user identifier.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Unauthorized`], [`HttpError::RateLimited`], or
    /// [`HttpError::Api`] with code -213 when the user is not subscribed to
    /// the OA.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::client::OaClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OaClient::new("TOKEN")?;
    ///     let profile = client.get_user_profile("USER_ID").await?;
    ///     println!("{}", profile.display_name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_user_profile(&self, user_id: impl AsRef<str>) -> HttpResult<UserProfile> {
        let url = self.endpoint("user/detail")?;

        debug!(endpoint = %url, user_id = user_id.as_ref(), "fetching user profile");

        let profile: UserProfile = self
            .get(url)
            .query(&[("user_id", user_id.as_ref())])
            .send_and_parse()
            .await?;

        Ok(profile)
    }

    /// Returns a paginated list of users following this OA.
    ///
    /// Use [`FollowerListQuery::first_page`] to start iteration and
    /// [`FollowerListQuery::page_after`] to continue.
    ///
    /// # Errors
    ///
    /// Returns [`HttpError::Unauthorized`] or [`HttpError::RateLimited`] on
    /// platform-level failures.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use zalo_http::{client::OaClient, types::FollowerListQuery};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = OaClient::new("TOKEN")?;
    ///     let page = client
    ///         .list_followers(FollowerListQuery::first_page(50))
    ///         .await?;
    ///     println!("total followers: {}", page.total);
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_followers(&self, query: FollowerListQuery) -> HttpResult<FollowerList> {
        let url = self.endpoint("user/getlist")?;

        debug!(
            endpoint = %url,
            offset = query.offset,
            count = query.count,
            "listing followers"
        );

        let list: FollowerList = self.get(url).query(&query).send_and_parse().await?;

        Ok(list)
    }

    fn endpoint(&self, path: &str) -> HttpResult<Url> {
        Url::parse(&format!("{BASE_URL}{path}")).map_err(|err| {
            HttpError::configuration(format!("could not build endpoint URL: {err}"))
        })
    }

    fn get(&self, url: Url) -> AuthenticatedRequest {
        AuthenticatedRequest {
            inner: self.inner.get(url).header(ACCESS_TOKEN_HEADER, &self.token)
        }
    }

    fn post(&self, url: Url) -> AuthenticatedRequest {
        AuthenticatedRequest {
            inner: self
                .inner
                .post(url)
                .header(ACCESS_TOKEN_HEADER, &self.token)
        }
    }
}

struct AuthenticatedRequest {
    inner: RequestBuilder
}

impl AuthenticatedRequest {
    fn json<S: serde::Serialize>(self, body: &S) -> Self {
        Self {
            inner: self.inner.json(body)
        }
    }

    fn query<S: serde::Serialize>(self, params: &S) -> Self {
        Self {
            inner: self.inner.query(params)
        }
    }

    async fn send_and_parse<T: DeserializeOwned>(self) -> HttpResult<T> {
        let response = self.inner.send().await.map_err(HttpError::from)?;

        let status = response.status();
        let body = response.text().await.map_err(HttpError::from)?;

        if !status.is_success() {
            warn!(status = %status, "unexpected HTTP status from Zalo API");
            return Err(HttpError::UnexpectedStatus {
                status: status.as_u16(),
                body
            });
        }

        let envelope: ApiResponse<T> = serde_json::from_str(&body).map_err(HttpError::from)?;

        if !envelope.is_ok() {
            warn!(
                code = envelope.error,
                message = %envelope.message,
                "Zalo API returned non-zero error code"
            );
            return Err(HttpError::from_api_response(
                envelope.error,
                envelope.message
            ));
        }

        envelope.data.ok_or_else(|| HttpError::Api {
            code:    0,
            message: "API returned success but data field was absent".to_owned()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_token() {
        let err = OaClient::new("").expect_err("empty token should fail");
        assert!(matches!(err, HttpError::Configuration(_)));
    }

    #[test]
    fn rejects_whitespace_only_token() {
        let err = OaClient::new("   ").expect_err("whitespace token should fail");
        assert!(matches!(err, HttpError::Configuration(_)));
    }

    #[test]
    fn accepts_valid_token() {
        OaClient::new("valid-token").expect("valid token should build client");
    }

    #[test]
    fn endpoint_builds_correct_url() {
        let client = OaClient::new("tok").expect("client");
        let url = client.endpoint("message/cs").expect("url");
        assert_eq!(url.as_str(), "https://openapi.zalo.me/v3.0/oa/message/cs");
    }
}
