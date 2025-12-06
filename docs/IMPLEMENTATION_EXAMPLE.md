# Implementation Example

This document provides code examples for integrating the selected `youtube_dl` crate into VDownloader.

## Basic Setup

### Cargo.toml

```toml
[package]
name = "vdownloader"
version = "0.1.0"
edition = "2021"

[dependencies]
# GTK4 for UI
gtk4 = { version = "0.9", package = "gtk4" }
glib = "0.20"

# Video downloading (SELECTED SOLUTION)
youtube_dl = "0.10.0"

# Async runtime
tokio = { version = "1", features = ["full"] }

# Error handling
anyhow = "1"
thiserror = "1"

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Core Components

### 1. Video Metadata Model

```rust
// src/models/video.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub id: String,
    pub title: String,
    pub duration: Option<f64>,
    pub thumbnail: Option<String>,
    pub uploader: Option<String>,
    pub description: Option<String>,
    pub url: String,
    pub platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Platform {
    YouTube,
    TikTok,
    Twitter,
    Instagram,
    Reddit,
    Other(String),
}

impl VideoInfo {
    /// Detect platform from URL
    pub fn detect_platform(url: &str) -> Platform {
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
            Platform::Other(url.to_string())
        }
    }
}
```

### 2. Download Service

```rust
// src/services/downloader.rs
use youtube_dl::{YoutubeDl, SingleVideo};
use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct VideoDownloader {
    output_dir: PathBuf,
}

impl VideoDownloader {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }

    /// Fetch video metadata without downloading
    pub async fn fetch_metadata(&self, url: &str) -> Result<SingleVideo> {
        let output = YoutubeDl::new(url)
            .socket_timeout("15")
            .run()
            .context("Failed to fetch video metadata")?;

        let video = output
            .into_single_video()
            .ok_or_else(|| anyhow::anyhow!("Not a single video"))?;

        Ok(video)
    }

    /// Download video with default settings
    pub async fn download(&self, url: &str) -> Result<PathBuf> {
        let output = YoutubeDl::new(url)
            .download(true)
            .output_directory(&self.output_dir)
            .output_template("%(title)s.%(ext)s")
            .run()
            .context("Failed to download video")?;

        // Get the downloaded file path
        let video = output
            .into_single_video()
            .ok_or_else(|| anyhow::anyhow!("Not a single video"))?;

        Ok(self.output_dir.join(format!("{}.mp4", video.title)))
    }

    /// Download with quality selection
    pub async fn download_with_quality(
        &self,
        url: &str,
        quality: VideoQuality,
    ) -> Result<PathBuf> {
        let format = match quality {
            VideoQuality::Best => "bestvideo+bestaudio/best",
            VideoQuality::Worst => "worstvideo+worstaudio/worst",
            VideoQuality::AudioOnly => "bestaudio/best",
            VideoQuality::Custom(ref f) => f,
        };

        YoutubeDl::new(url)
            .download(true)
            .format(format)
            .output_directory(&self.output_dir)
            .output_template("%(title)s.%(ext)s")
            .run()
            .context("Failed to download video")?;

        Ok(self.output_dir.clone())
    }

    /// Download with progress callback (for UI updates)
    pub async fn download_with_progress<F>(
        &self,
        url: &str,
        mut progress_callback: F,
    ) -> Result<PathBuf>
    where
        F: FnMut(f64) + Send + 'static,
    {
        // Note: youtube_dl crate doesn't have built-in progress tracking
        // For MVP, just report start and end
        progress_callback(0.0);

        let path = self.download(url).await?;

        progress_callback(100.0);
        Ok(path)
    }

    /// Check if yt-dlp is installed
    pub fn check_ytdlp_installed() -> Result<bool> {
        let output = std::process::Command::new("yt-dlp")
            .arg("--version")
            .output();

        Ok(output.is_ok())
    }
}

#[derive(Debug, Clone)]
pub enum VideoQuality {
    Best,
    Worst,
    AudioOnly,
    Custom(String),
}
```

### 3. URL Validator

```rust
// src/utils/validators.rs
use anyhow::{bail, Result};
use url::Url;

/// Validate URL format
pub fn validate_url(url: &str) -> Result<()> {
    let parsed = Url::parse(url)?;

    if parsed.scheme() != "http" && parsed.scheme() != "https" {
        bail!("URL must use http or https protocol");
    }

    Ok(())
}

/// Check if URL is from a supported platform
pub fn is_supported_platform(url: &str) -> bool {
    let supported_domains = [
        "youtube.com",
        "youtu.be",
        "tiktok.com",
        "twitter.com",
        "x.com",
        "instagram.com",
        "reddit.com",
        "v.redd.it",
    ];

    supported_domains.iter().any(|domain| url.contains(domain))
}
```

### 4. GTK Integration Example

```rust
// src/ui/window.rs
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Entry, Label, Orientation};
use crate::services::downloader::{VideoDownloader, VideoQuality};
use std::path::PathBuf;

pub fn build_ui(app: &Application) {
    // Create main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("VDownloader")
        .default_width(600)
        .default_height(400)
        .build();

    // Create layout
    let vbox = Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);

    // URL input
    let url_label = Label::new(Some("Video URL:"));
    vbox.append(&url_label);

    let url_entry = Entry::new();
    url_entry.set_placeholder_text(Some("Enter video URL (YouTube, TikTok, etc.)"));
    vbox.append(&url_entry);

    // Status label
    let status_label = Label::new(Some("Ready"));
    vbox.append(&status_label);

    // Download button
    let download_button = Button::with_label("Download");
    vbox.append(&download_button);

    // Connect download button
    let url_entry_clone = url_entry.clone();
    let status_label_clone = status_label.clone();
    download_button.connect_clicked(move |_| {
        let url = url_entry_clone.text().to_string();
        let status = status_label_clone.clone();

        if url.is_empty() {
            status.set_text("Error: Please enter a URL");
            return;
        }

        // Spawn async download task
        glib::spawn_future_local(async move {
            status.set_text("Downloading...");

            let downloader = VideoDownloader::new(
                PathBuf::from(std::env::var("HOME").unwrap()).join("Downloads")
            );

            match downloader.download(&url).await {
                Ok(path) => {
                    status.set_text(&format!("Downloaded: {}", path.display()));
                }
                Err(e) => {
                    status.set_text(&format!("Error: {}", e));
                }
            }
        });
    });

    window.set_child(Some(&vbox));
    window.present();
}
```

### 5. Main Application

```rust
// src/main.rs
mod models;
mod services;
mod ui;
mod utils;

use gtk4::prelude::*;
use gtk4::Application;

fn main() {
    // Check if yt-dlp is installed
    if !services::downloader::VideoDownloader::check_ytdlp_installed()
        .unwrap_or(false)
    {
        eprintln!("Error: yt-dlp is not installed!");
        eprintln!("Please install yt-dlp:");
        eprintln!("  Linux: sudo apt install yt-dlp");
        eprintln!("  macOS: brew install yt-dlp");
        eprintln!("  Windows: pip install yt-dlp");
        std::process::exit(1);
    }

    // Create GTK application
    let app = Application::builder()
        .application_id("com.vdownloader.app")
        .build();

    app.connect_activate(ui::window::build_ui);

    app.run();
}
```

## Advanced Features

### Download Queue Management

```rust
// src/services/queue.rs
use std::collections::VecDeque;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct DownloadQueue {
    queue: Arc<Mutex<VecDeque<DownloadTask>>>,
    downloader: VideoDownloader,
}

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub id: String,
    pub url: String,
    pub status: DownloadStatus,
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
}

impl DownloadQueue {
    pub fn new(downloader: VideoDownloader) -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            downloader,
        }
    }

    pub async fn add_task(&self, url: String) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let task = DownloadTask {
            id: id.clone(),
            url,
            status: DownloadStatus::Pending,
        };

        let mut queue = self.queue.lock().await;
        queue.push_back(task);
        id
    }

    pub async fn process_queue(&self) {
        loop {
            let task = {
                let mut queue = self.queue.lock().await;
                queue.pop_front()
            };

            if let Some(mut task) = task {
                task.status = DownloadStatus::Downloading;
                
                match self.downloader.download(&task.url).await {
                    Ok(_) => {
                        task.status = DownloadStatus::Completed;
                    }
                    Err(e) => {
                        task.status = DownloadStatus::Failed(e.to_string());
                    }
                }
            } else {
                break;
            }
        }
    }
}
```

### Error Handling

```rust
// src/services/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Unsupported platform")]
    UnsupportedPlatform,

    #[error("yt-dlp not found. Please install yt-dlp.")]
    YtDlpNotFound,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Download failed: {0}")]
    DownloadFailed(String),

    #[error("File system error: {0}")]
    FileSystemError(#[from] std::io::Error),
}
```

## Testing Examples

```rust
// tests/downloader_test.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_metadata() {
        let downloader = VideoDownloader::new(PathBuf::from("/tmp"));
        
        // Test with a stable public video
        let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
        let result = downloader.fetch_metadata(url).await;
        
        assert!(result.is_ok());
        let video = result.unwrap();
        assert!(!video.title.is_empty());
    }

    #[test]
    fn test_url_validation() {
        assert!(validate_url("https://youtube.com/watch?v=abc123").is_ok());
        assert!(validate_url("http://youtube.com/watch?v=abc123").is_ok());
        assert!(validate_url("ftp://youtube.com").is_err());
        assert!(validate_url("not a url").is_err());
    }

    #[test]
    fn test_platform_detection() {
        assert!(is_supported_platform("https://youtube.com/watch?v=123"));
        assert!(is_supported_platform("https://tiktok.com/@user/video/123"));
        assert!(is_supported_platform("https://twitter.com/user/status/123"));
        assert!(!is_supported_platform("https://unknown.com/video"));
    }
}
```

## Usage Examples

### Simple Download

```rust
use vdownloader::services::downloader::VideoDownloader;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = VideoDownloader::new(PathBuf::from("./downloads"));
    
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let path = downloader.download(url).await?;
    
    println!("Downloaded to: {}", path.display());
    Ok(())
}
```

### With Metadata Fetch

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = VideoDownloader::new(PathBuf::from("./downloads"));
    
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    
    // First, fetch metadata
    let metadata = downloader.fetch_metadata(url).await?;
    println!("Title: {}", metadata.title);
    println!("Duration: {:?} seconds", metadata.duration);
    
    // Then download
    let path = downloader.download(url).await?;
    println!("Downloaded to: {}", path.display());
    
    Ok(())
}
```

### Multiple Platforms

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let downloader = VideoDownloader::new(PathBuf::from("./downloads"));
    
    let urls = vec![
        "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
        "https://www.tiktok.com/@user/video/123456",
        "https://twitter.com/user/status/123456",
    ];
    
    for url in urls {
        match downloader.download(url).await {
            Ok(path) => println!("✓ Downloaded: {}", path.display()),
            Err(e) => eprintln!("✗ Failed {}: {}", url, e),
        }
    }
    
    Ok(())
}
```

## Deployment Checklist

- [ ] Add `youtube_dl = "0.10.0"` to Cargo.toml
- [ ] Implement VideoDownloader service
- [ ] Add URL validation
- [ ] Create GTK4 UI
- [ ] Add yt-dlp installation check
- [ ] Write user documentation
- [ ] Test on all platforms:
  - [ ] YouTube
  - [ ] TikTok
  - [ ] Twitter
  - [ ] Instagram
  - [ ] Reddit
- [ ] Test on all operating systems:
  - [ ] Linux
  - [ ] Windows
  - [ ] macOS
- [ ] Add error handling for common cases
- [ ] Write unit tests
- [ ] Create installation guide

## Next Steps

1. Copy relevant code snippets to your project
2. Adapt to your specific architecture
3. Test with real URLs from each platform
4. Iterate based on user feedback

## References

- [youtube_dl crate docs](https://docs.rs/youtube_dl/)
- [GTK4-rs book](https://gtk-rs.org/gtk4-rs/stable/latest/book/)
- [yt-dlp documentation](https://github.com/yt-dlp/yt-dlp/wiki)
