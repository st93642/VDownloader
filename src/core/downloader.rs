use crate::core::error::{DownloadError, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    YouTube,
    TikTok,
    Twitter,
    Instagram,
    Reddit,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub platform: Platform,
    pub output_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DownloadStatus {
    Pending,
    Downloading { progress: f32 },
    Completed { file_path: String },
    Failed { error: String },
}

pub struct VideoDownloader {
    output_directory: String,
}

impl VideoDownloader {
    pub fn new(output_directory: String) -> Self {
        info!(
            "Creating VideoDownloader with output directory: {}",
            output_directory
        );
        Self { output_directory }
    }

    pub fn detect_platform(url: &str) -> Platform {
        debug!("Detecting platform for URL: {}", url);

        if url.contains("youtube.com") || url.contains("youtu.be") {
            Platform::YouTube
        } else if url.contains("tiktok.com") {
            Platform::TikTok
        } else if url.contains("twitter.com") || url.contains("x.com") {
            Platform::Twitter
        } else if url.contains("instagram.com") {
            Platform::Instagram
        } else if url.contains("reddit.com") {
            Platform::Reddit
        } else {
            Platform::Other
        }
    }

    pub async fn download(&self, request: DownloadRequest) -> Result<String> {
        info!("Starting download for URL: {}", request.url);

        if request.url.is_empty() {
            return Err(DownloadError::InvalidUrl("URL cannot be empty".to_string()));
        }

        Ok("Download functionality to be implemented with youtube_dl crate".to_string())
    }

    pub fn get_output_directory(&self) -> &str {
        &self.output_directory
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_platform_youtube() {
        assert!(matches!(
            VideoDownloader::detect_platform("https://www.youtube.com/watch?v=abc123"),
            Platform::YouTube
        ));
        assert!(matches!(
            VideoDownloader::detect_platform("https://youtu.be/abc123"),
            Platform::YouTube
        ));
    }

    #[test]
    fn test_detect_platform_tiktok() {
        assert!(matches!(
            VideoDownloader::detect_platform("https://www.tiktok.com/@user/video/123"),
            Platform::TikTok
        ));
    }

    #[test]
    fn test_detect_platform_twitter() {
        assert!(matches!(
            VideoDownloader::detect_platform("https://twitter.com/user/status/123"),
            Platform::Twitter
        ));
        assert!(matches!(
            VideoDownloader::detect_platform("https://x.com/user/status/123"),
            Platform::Twitter
        ));
    }
}
