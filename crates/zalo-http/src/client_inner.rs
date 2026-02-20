// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! OaClient module.

mod articles;
mod client;
mod conversations;
mod messaging;
mod request;
mod store;
mod tags;
mod users;

pub use client::OaClient;

pub(crate) use request::{endpoint_url, get_simple, get_with_query, post_json};
pub(crate) use messaging::{send_text_message, send_typed_text_message, send_image_message, send_typed_image_message, send_file_message, send_typed_file_message, send_template_message, send_template_message_with_elements};
pub(crate) use users::{get_user_profile, list_followers, get_oa_info, update_follower_info};
pub(crate) use tags::{get_tags, tag_followers, untag_followers};
pub(crate) use conversations::{list_recent_chats, get_conversation};
pub(crate) use store::{create_product, update_product, get_product, list_products, create_order, update_order, get_order, list_orders};
pub(crate) use articles::{create_article, verify_article, upload_video_prepare, upload_video_verify};
