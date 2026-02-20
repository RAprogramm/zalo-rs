// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

//! Media upload client.

use std::path::Path;

use reqwest::{Client, multipart};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use super::error::{MediaError, MediaResult};
use super::types::{MediaUploadResponse, UploadType};

const BASE_URL: &str = "https://openapi.zalo.me/v3.0/oa/upload/";

/// Media manager for uploading files.
#[derive(Clone, Debug)]
pub struct MediaManager {
    client: Client,
    token: String,
}

impl MediaManager {
    /// Creates new manager.
    pub fn new(access_token: impl Into<String>) -> MediaResult<Self> {
        let token = access_token.into();
        if token.trim().is_empty() {
            return Err(MediaError::InvalidUrl("access token must not be empty".into()));
        }
        Ok(Self {
            client: Client::new(),
            token,
        })
    }

    /// Uploads image (JPG, PNG, max 1MB).
    pub async fn upload_image(&self, path: impl AsRef<Path>) -> MediaResult<MediaUploadResponse> {
        let path = path.as_ref();
        validate_image_ext(path)?;
        self.upload_inner(path, UploadType::Image).await
    }

    /// Uploads file (PDF, DOC, DOCX, XLS, XLSX, max 5MB).
    pub async fn upload_document(&self, path: impl AsRef<Path>) -> MediaResult<MediaUploadResponse> {
        let path = path.as_ref();
        validate_file_ext(path)?;
        self.upload_inner(path, UploadType::File).await
    }

    /// Uploads GIF (max 1MB).
    pub async fn upload_gif(&self, path: impl AsRef<Path>) -> MediaResult<MediaUploadResponse> {
        let path = path.as_ref();
        validate_gif_ext(path)?;
        self.upload_inner(path, UploadType::Gif).await
    }

    /// Uploads image from HTTPS URL.
    pub async fn upload_image_from_url(
        &self,
        url: impl Into<String>,
    ) -> MediaResult<MediaUploadResponse> {
        let url = url.into();
        validate_https(&url)?;
        let bytes = download_url(&self.client, &url).await?;
        UploadType::Image.check_size(bytes.len())?;
        self.upload_bytes(&bytes, "image.jpg", UploadType::Image).await
    }

    /// Uploads file from HTTPS URL.
    pub async fn upload_document_from_url(
        &self,
        url: impl Into<String>,
    ) -> MediaResult<MediaUploadResponse> {
        let url = url.into();
        validate_https(&url)?;
        let bytes = download_url(&self.client, &url).await?;
        UploadType::File.check_size(bytes.len())?;
        self.upload_bytes(&bytes, "file.dat", UploadType::File).await
    }

    async fn upload_inner(
        &self,
        path: &Path,
        upload_type: UploadType,
    ) -> MediaResult<MediaUploadResponse> {
        let mut file = File::open(path)
            .await
            .map_err(|e| MediaError::NotFound(format!("{}: {}", path.display(), e)))?;

        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .await
            .map_err(|e| MediaError::NotFound(format!("{}: {}", path.display(), e)))?;

        upload_type.check_size(bytes.len())?;

        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("upload");

        self.upload_bytes(&bytes, filename, upload_type).await
    }

    async fn upload_bytes(
        &self,
        bytes: &[u8],
        filename: &str,
        upload_type: UploadType,
    ) -> MediaResult<MediaUploadResponse> {
        let url = format!("{}{}", BASE_URL, upload_type.endpoint());

        let part = multipart::Part::bytes(bytes.to_vec())
            .file_name(filename.to_string())
            .mime_str(upload_type.mime_type())
            .map_err(|e| MediaError::InvalidFormat(e.to_string()))?;

        let form = multipart::Form::new().part("file", part);

        let response = self
            .client
            .post(&url)
            .header("access_token", &self.token)
            .multipart(form)
            .send()
            .await
            .map_err(|e| MediaError::Http(crate::error::HttpError::from(e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response
                .text()
                .await
                .map_err(|e| MediaError::Http(crate::error::HttpError::from(e)))?;
            return Err(MediaError::Http(crate::error::HttpError::UnexpectedStatus {
                status: status.as_u16(),
                body,
            }));
        }

        response
            .json()
            .await
            .map_err(|e| MediaError::Http(crate::error::HttpError::from(e)))
    }
}

fn validate_image_ext(path: &Path) -> MediaResult<()> {
    let ext = get_extension(path)?;
    if !matches!(ext.as_str(), "jpg" | "jpeg" | "png") {
        return Err(MediaError::InvalidFormat(
            "image must be JPG or PNG".into(),
        ));
    }
    Ok(())
}

fn validate_file_ext(path: &Path) -> MediaResult<()> {
    let ext = get_extension(path)?;
    if !matches!(ext.as_str(), "pdf" | "doc" | "docx" | "xls" | "xlsx") {
        return Err(MediaError::InvalidFormat(
            "file must be PDF, DOC, DOCX, XLS, or XLSX".into(),
        ));
    }
    Ok(())
}

fn validate_gif_ext(path: &Path) -> MediaResult<()> {
    let ext = get_extension(path)?;
    if ext != "gif" {
        return Err(MediaError::InvalidFormat("file must be GIF".into()));
    }
    Ok(())
}

fn get_extension(path: &Path) -> MediaResult<String> {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_lowercase())
        .ok_or_else(|| MediaError::InvalidFormat("file must have an extension".into()))
}

fn validate_https(url: &str) -> MediaResult<()> {
    if !url.starts_with("https://") {
        return Err(MediaError::InvalidUrl("URL must use HTTPS".into()));
    }
    Ok(())
}

async fn download_url(client: &Client, url: &str) -> MediaResult<Vec<u8>> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| MediaError::Http(crate::error::HttpError::from(e)))?;
    let bytes = response
        .bytes()
        .await
        .map_err(|e| MediaError::Http(crate::error::HttpError::from(e)))?;
    Ok(bytes.to_vec())
}

impl UploadType {
    fn check_size(&self, size: usize) -> MediaResult<()> {
        let max = self.max_size();
        if size > max {
            return Err(MediaError::TooLarge { size, max });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_token() {
        assert!(MediaManager::new("").is_err());
    }

    #[test]
    fn accepts_valid_token() {
        assert!(MediaManager::new("token").is_ok());
    }

    #[test]
    fn validates_image_extensions() {
        assert!(validate_image_ext(Path::new("test.jpg")).is_ok());
        assert!(validate_image_ext(Path::new("test.png")).is_ok());
        assert!(validate_image_ext(Path::new("test.gif")).is_err());
    }

    #[test]
    fn validates_file_extensions() {
        assert!(validate_file_ext(Path::new("doc.pdf")).is_ok());
        assert!(validate_file_ext(Path::new("doc.docx")).is_ok());
        assert!(validate_file_ext(Path::new("img.jpg")).is_err());
    }

    #[test]
    fn validates_https() {
        assert!(validate_https("https://example.com").is_ok());
        assert!(validate_https("http://example.com").is_err());
    }
}
