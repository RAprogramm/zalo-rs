// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

use crate::error::{SdkError, SdkResult};

/// Validated in-app route path.
///
/// Paths must start with `/` and be non-empty after the prefix.
///
/// # Examples
///
/// ```
/// use zalo_sdk::navigation::RoutePath;
///
/// let path = RoutePath::new("/product/42")?;
/// assert_eq!(path.as_str(), "/product/42");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RoutePath(String);

impl RoutePath {
    /// Creates a validated route path.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidRoutePath`] when the path is empty, does not
    /// start with `/`, or contains only a bare `/`.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::navigation::RoutePath;
    ///
    /// let path = RoutePath::new("/home")?;
    /// assert_eq!(path.as_str(), "/home");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if !value.starts_with('/') || value.trim_end_matches('/').is_empty() {
            return Err(SdkError::InvalidRoutePath(value));
        }
        Ok(Self(value))
    }

    /// Returns the raw path string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Title displayed in the mini app navigation bar.
///
/// Must be non-empty and fit within 64 characters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NavTitle(String);

impl NavTitle {
    const MAX_LEN: usize = 64;

    /// Creates a validated navigation title.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidNavTitle`] when the title is blank or
    /// exceeds the character limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::navigation::NavTitle;
    ///
    /// let title = NavTitle::new("Product Details")?;
    /// assert_eq!(title.as_str(), "Product Details");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(value: impl Into<String>) -> SdkResult<Self> {
        let value = value.into();
        if value.trim().is_empty() || value.len() > Self::MAX_LEN {
            return Err(SdkError::InvalidNavTitle(value));
        }
        Ok(Self(value))
    }

    /// Returns the raw title string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Request to navigate to an internal route within the mini app.
///
/// # Examples
///
/// ```
/// use zalo_sdk::navigation::NavigateRequest;
///
/// let req = NavigateRequest::to("/cart")?;
/// assert_eq!(req.path, "/cart");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct NavigateRequest {
    /// Target route path.
    pub path: String
}

impl NavigateRequest {
    /// Constructs a navigation request to the given path.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`RoutePath::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::navigation::NavigateRequest;
    ///
    /// let req = NavigateRequest::to("/checkout")?;
    /// assert_eq!(req.path, "/checkout");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn to(path: impl Into<String>) -> SdkResult<Self> {
        let route = RoutePath::new(path)?;
        Ok(Self {
            path: route.0
        })
    }
}

/// Request to open an external web page inside the mini app's web view.
///
/// # Examples
///
/// ```
/// use zalo_sdk::navigation::OpenWebviewRequest;
///
/// let req = OpenWebviewRequest::new("https://example.com", "Help")?;
/// assert_eq!(req.url, "https://example.com");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct OpenWebviewRequest {
    /// Fully qualified URL to open.
    pub url:   String,
    /// Title shown in the web view header.
    pub title: String
}

impl OpenWebviewRequest {
    /// Constructs a web view request after validating both fields.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::InvalidUrl`] when the URL is blank, or
    /// [`SdkError::InvalidNavTitle`] from [`NavTitle::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::navigation::OpenWebviewRequest;
    ///
    /// let req = OpenWebviewRequest::new("https://docs.example.com", "Docs")?;
    /// assert_eq!(req.title, "Docs");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> SdkResult<Self> {
        let url = url.into();
        if url.trim().is_empty() {
            return Err(SdkError::InvalidUrl(url));
        }
        let title = NavTitle::new(title)?;
        Ok(Self {
            url,
            title: title.0
        })
    }
}

/// Request to set the title of the current navigation bar.
///
/// # Examples
///
/// ```
/// use zalo_sdk::navigation::SetTitleRequest;
///
/// let req = SetTitleRequest::new("My Cart")?;
/// assert_eq!(req.title, "My Cart");
/// # Ok::<_, Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Debug, Serialize)]
pub struct SetTitleRequest {
    /// New navigation bar title.
    pub title: String
}

impl SetTitleRequest {
    /// Constructs a title update request.
    ///
    /// # Errors
    ///
    /// Propagates errors from [`NavTitle::new`].
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::navigation::SetTitleRequest;
    ///
    /// let req = SetTitleRequest::new("Order Placed")?;
    /// assert_eq!(req.title, "Order Placed");
    /// # Ok::<_, Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(title: impl Into<String>) -> SdkResult<Self> {
        let title = NavTitle::new(title)?;
        Ok(Self {
            title: title.0
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_path_rejects_no_leading_slash() {
        let err = RoutePath::new("home").expect_err("no slash");
        assert!(matches!(err, SdkError::InvalidRoutePath(_)));
    }

    #[test]
    fn route_path_rejects_empty() {
        let err = RoutePath::new("").expect_err("empty");
        assert!(matches!(err, SdkError::InvalidRoutePath(_)));
    }

    #[test]
    fn route_path_rejects_slash_only() {
        let err = RoutePath::new("/").expect_err("slash only");
        assert!(matches!(err, SdkError::InvalidRoutePath(_)));
    }

    #[test]
    fn route_path_accepts_valid() {
        let path = RoutePath::new("/product/42").expect("valid");
        assert_eq!(path.as_str(), "/product/42");
    }

    #[test]
    fn nav_title_rejects_empty() {
        let err = NavTitle::new("").expect_err("empty");
        assert!(matches!(err, SdkError::InvalidNavTitle(_)));
    }

    #[test]
    fn nav_title_rejects_too_long() {
        let long = "x".repeat(NavTitle::MAX_LEN + 1);
        let err = NavTitle::new(long).expect_err("too long");
        assert!(matches!(err, SdkError::InvalidNavTitle(_)));
    }

    #[test]
    fn nav_title_accepts_boundary() {
        let exact = "x".repeat(NavTitle::MAX_LEN);
        NavTitle::new(exact).expect("exact limit");
    }

    #[test]
    fn navigate_request_validates_path() {
        let req = NavigateRequest::to("/cart").expect("valid");
        assert_eq!(req.path, "/cart");
    }

    #[test]
    fn navigate_request_rejects_invalid_path() {
        let err = NavigateRequest::to("cart").expect_err("no slash");
        assert!(matches!(err, SdkError::InvalidRoutePath(_)));
    }

    #[test]
    fn open_webview_rejects_empty_url() {
        let err = OpenWebviewRequest::new("", "title").expect_err("empty url");
        assert!(matches!(err, SdkError::InvalidUrl(_)));
    }

    #[test]
    fn open_webview_rejects_empty_title() {
        let err = OpenWebviewRequest::new("https://example.com", "").expect_err("empty title");
        assert!(matches!(err, SdkError::InvalidNavTitle(_)));
    }

    #[test]
    fn open_webview_accepts_valid() {
        let req = OpenWebviewRequest::new("https://example.com", "Help").expect("valid");
        assert_eq!(req.url, "https://example.com");
        assert_eq!(req.title, "Help");
    }

    #[test]
    fn set_title_request_validates_title() {
        let req = SetTitleRequest::new("My Orders").expect("valid");
        assert_eq!(req.title, "My Orders");
    }
}
