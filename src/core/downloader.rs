use crate::core::error::{DownloadError, Result};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use std::path::Path;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

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

    pub fn validate_url(url: &str) -> Result<()> {
        if url.is_empty() {
            return Err(DownloadError::InvalidUrl("URL cannot be empty".to_string()));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(DownloadError::InvalidUrl(
                "URL must start with http:// or https://".to_string(),
            ));
        }

        if url.len() > 2048 {
            return Err(DownloadError::InvalidUrl("URL is too long".to_string()));
        }

        Ok(())
    }

    pub fn validate_output_directory(output_dir: &str) -> Result<()> {
        let path = Path::new(output_dir);

        if !path.exists() {
            return Err(DownloadError::InvalidOutputDirectory);
        }

        if !path.is_dir() {
            return Err(DownloadError::InvalidOutputDirectory);
        }

        match std::fs::metadata(output_dir) {
            Ok(metadata) => {
                if metadata.permissions().readonly() {
                    return Err(DownloadError::InvalidOutputDirectory);
                }
                Ok(())
            }
            Err(_) => Err(DownloadError::InvalidOutputDirectory),
        }
    }

    pub async fn download(&self, request: DownloadRequest) -> Result<String> {
        info!("Starting download for URL: {}", request.url);

        Self::validate_url(&request.url)?;
        Self::validate_output_directory(&self.output_directory)?;

        let output_dir = self.output_directory.clone();
        let url = request.url.clone();

        tokio::task::spawn_blocking(move || Self::perform_download(&url, &output_dir))
            .await
            .map_err(|e| DownloadError::DownloadFailed(format!("Task join error: {}", e)))?
    }

    fn perform_download(url: &str, output_dir: &str) -> Result<String> {
        info!("Performing download of {} to {}", url, output_dir);

        let output_template = format!("{}/%%(title)s.%%(ext)s", output_dir);

        let result = YoutubeDl::new(url)
            .socket_timeout("30")
            .extract_audio(false)
            .output_template(&output_template)
            .run();

        match result {
            Ok(YoutubeDlOutput::Playlist(_playlist)) => {
                warn!("Playlist detected, downloading first video only");
                Self::handle_playlist_download(url, output_dir)
            }
            Ok(YoutubeDlOutput::SingleVideo(video)) => {
                let video_title = video.title.clone().unwrap_or_else(|| "video".to_string());
                info!("Download completed: {}", video_title);
                Ok(format!("{}/{}", output_dir, video_title))
            }
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("Video unavailable") || error_msg.contains("not found") {
                    warn!("Video not found or unavailable: {}", e);
                    Err(DownloadError::DownloadFailed(
                        "Video not found or unavailable".to_string(),
                    ))
                } else if error_msg.contains("Network") || error_msg.contains("Connection") {
                    warn!("Network error: {}", e);
                    Err(DownloadError::DownloadFailed(error_msg))
                } else {
                    warn!("Download error: {}", e);
                    Err(DownloadError::DownloadFailed(error_msg))
                }
            }
        }
    }

    fn handle_playlist_download(url: &str, output_dir: &str) -> Result<String> {
        let output_template = format!("{}/%%(title)s.%%(ext)s", output_dir);

        match YoutubeDl::new(url)
            .socket_timeout("30")
            .extract_audio(false)
            .output_template(&output_template)
            .playlist_items(1u32)
            .run()
        {
            Ok(YoutubeDlOutput::Playlist(playlist)) => playlist
                .entries
                .and_then(|mut entries| entries.pop())
                .and_then(|video| video.title.clone())
                .map(|title| format!("{}/{}", output_dir, title))
                .ok_or_else(|| {
                    DownloadError::DownloadFailed("Failed to download from playlist".to_string())
                }),
            Ok(YoutubeDlOutput::SingleVideo(video)) => {
                let video_title = video.title.clone().unwrap_or_else(|| "video".to_string());
                Ok(format!("{}/{}", output_dir, video_title))
            }
            Err(e) => Err(DownloadError::DownloadFailed(e.to_string())),
        }
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

    #[test]
    fn test_validate_url_valid() {
        assert!(VideoDownloader::validate_url("https://www.youtube.com/watch?v=abc123").is_ok());
        assert!(VideoDownloader::validate_url("http://example.com/video").is_ok());
    }

    #[test]
    fn test_validate_url_invalid() {
        assert!(VideoDownloader::validate_url("").is_err());
        assert!(VideoDownloader::validate_url("not-a-url").is_err());
        assert!(VideoDownloader::validate_url("ftp://example.com/video").is_err());
        let long_url = format!("https://{}", "a".repeat(2100));
        assert!(VideoDownloader::validate_url(&long_url).is_err());
    }

    #[test]
    fn test_validate_output_directory_valid() {
        let temp_dir = std::env::temp_dir().to_string_lossy().to_string();
        assert!(VideoDownloader::validate_output_directory(&temp_dir).is_ok());
    }

    #[test]
    fn test_validate_output_directory_invalid() {
        assert!(VideoDownloader::validate_output_directory("/nonexistent/path/12345").is_err());
    }
}
