// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Article API endpoints.

use crate::client_inner::{endpoint_url, get_with_query, post_json};
use crate::error::HttpResult;
use zalo_types::article::{ArticleVerification, ArticleVerificationQuery, CreateArticleRequest, VideoUploadPrepareRequest, VideoUploadPrepareResponse, VideoUploadVerifyRequest};

/// Creates an article.
pub async fn create_article(
    token: &str,
    request: CreateArticleRequest,
) -> HttpResult<ArticleVerification> {
    let url = endpoint_url("article/create")?;

    tracing::debug!(
        endpoint = %url,
        title = %request.title,
        "creating article"
    );

    post_json(token, url, &request).await
}

/// Verifies an article.
pub async fn verify_article(
    token: &str,
    query: ArticleVerificationQuery,
) -> HttpResult<ArticleVerification> {
    let url = endpoint_url("article/verify")?;

    tracing::debug!(
        endpoint = %url,
        token = %query.token,
        "verifying article"
    );

    get_with_query(token, url, &query).await
}

/// Prepares video upload.
pub async fn upload_video_prepare(
    token: &str,
    request: VideoUploadPrepareRequest,
) -> HttpResult<VideoUploadPrepareResponse> {
    let url = endpoint_url("article/upload_video/preparevideo")?;

    tracing::debug!(
        endpoint = %url,
        video_name = %request.video_name,
        video_size = request.video_size,
        "preparing video upload"
    );

    post_json(token, url, &request).await
}

/// Verifies video upload.
pub async fn upload_video_verify(
    token: &str,
    request: VideoUploadVerifyRequest,
) -> HttpResult<()> {
    let url = endpoint_url("article/upload_video/verify")?;

    tracing::debug!(
        endpoint = %url,
        upload_id = %request.upload_id,
        "verifying video upload"
    );

    let _: serde_json::Value = post_json(token, url, &request).await?;

    Ok(())
}
