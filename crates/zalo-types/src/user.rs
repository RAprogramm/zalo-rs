// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! User-related types for Zalo OA API.

use serde::{Deserialize, Serialize};

/// Official Account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OaInfo {
    /// OA ID.
    pub id: String,
    /// OA name.
    pub name: String,
    /// OA description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// OA avatar URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// OA cover image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover: Option<String>,
    /// Number of followers.
    pub follower_count: u64,
    /// OA website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// OA phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// OA address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

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

/// Request to update follower information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFollowerRequest {
    /// User ID.
    pub user_id: String,
    /// New name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// New email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// New address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// New city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// New birthday.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
}

impl UpdateFollowerRequest {
    /// Creates a new update request.
    #[must_use]
    pub fn new(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            name: None,
            phone: None,
            email: None,
            address: None,
            city: None,
            birthday: None,
        }
    }

    /// Sets name.
    #[must_use]
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets phone.
    #[must_use]
    pub fn with_phone(mut self, phone: impl Into<String>) -> Self {
        self.phone = Some(phone.into());
        self
    }

    /// Sets email.
    #[must_use]
    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Sets address.
    #[must_use]
    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    /// Sets city.
    #[must_use]
    pub fn with_city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    /// Sets birthday.
    #[must_use]
    pub fn with_birthday(mut self, birthday: impl Into<String>) -> Self {
        self.birthday = Some(birthday.into());
        self
    }
}
