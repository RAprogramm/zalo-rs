// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Navigation types.

use serde::{Deserialize, Serialize};

use crate::error::SdkResult;

/// Route path wrapper.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RoutePath(String);

impl RoutePath {
    /// Creates a validated route path.
    ///
    /// # Errors
    ///
    /// Returns error if path doesn't start with `/`.
    pub fn new(path: impl Into<String>) -> SdkResult<Self> {
        let path = path.into();
        if !path.starts_with('/') || path.len() < 2 {
            return Err(crate::SdkError::InvalidRoutePath(path));
        }
        Ok(Self(path))
    }

    /// Returns the path string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Navigate request.
#[derive(Clone, Debug, Serialize)]
pub struct NavigateRequest {
    path: String,
}

impl NavigateRequest {
    /// Creates a new navigate request.
    #[must_use]
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

/// Open webview request.
#[derive(Clone, Debug, Serialize)]
pub struct OpenWebviewRequest {
    url: String,
    title: String,
}

impl OpenWebviewRequest {
    /// Creates a new open webview request.
    #[must_use]
    pub fn new(url: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            title: title.into(),
        }
    }
}

/// Set title request.
#[derive(Clone, Debug, Serialize)]
pub struct SetTitleRequest {
    title: String,
}

impl SetTitleRequest {
    /// Creates a new set title request.
    ///
    /// # Errors
    ///
    /// Returns error if title is empty or too long.
    pub fn new(title: impl Into<String>) -> SdkResult<Self> {
        let title = title.into();
        if title.trim().is_empty() || title.len() > 50 {
            return Err(crate::SdkError::InvalidNavTitle(title));
        }
        Ok(Self { title })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_path_accepts_valid() {
        let path = RoutePath::new("/home").unwrap();
        assert_eq!(path.as_str(), "/home");
    }

    #[test]
    fn route_path_rejects_no_slash() {
        assert!(RoutePath::new("home").is_err());
    }

    #[test]
    fn set_title_accepts_valid() {
        let title = SetTitleRequest::new("Home").unwrap();
        assert_eq!(title.title, "Home");
    }
}
