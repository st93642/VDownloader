/*****************************************************************************/
/*                                                                           */
/*  downloader.rs                                        TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 07 2025 13:36 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 07 2025 22:55 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/

use crate::core::error::{DownloadError, Result};
use log::{debug, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use youtube_dl::{YoutubeDl, YoutubeDlOutput};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    YouTube,
    TikTok,
    Twitter,
    Instagram,
    Reddit,
    Vk,
    Rutube,
    Dzen,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadRequest {
    pub url: String,
    pub platform: Platform,
    pub output_path: Option<String>,
    #[serde(default)]
    pub overwrite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
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
        } else if url.contains("vk.com") || url.contains("vkvideo.ru") {
            Platform::Vk
        } else if url.contains("rutube.ru") {
            Platform::Rutube
        } else if url.contains("dzen.ru") {
            Platform::Dzen
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

    #[allow(dead_code)]
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
    pub async fn download<F>(&self, request: DownloadRequest, on_progress: F) -> Result<String>
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        info!("Starting download for URL: {}", request.url);

        let url = Self::sanitize_url(&request.url);
        if url != request.url {
            info!("Sanitized URL to: {}", url);
        }

        Self::validate_url(&url)?;
        // Self::validate_output_directory(&self.output_directory)?; // Skip directory validation as we might have a file path

        let output_path = self.output_directory.clone();
        let overwrite = request.overwrite;

        tokio::task::spawn_blocking(move || {
            Self::perform_download(&url, &output_path, overwrite, on_progress)
        })
        .await
        .map_err(|e| DownloadError::DownloadFailed(format!("Task join error: {}", e)))?
    }

    fn sanitize_url(url: &str) -> String {
        // Handle VK playlist+video URLs by extracting the video ID
        if url.contains("vk.com") || url.contains("vkvideo.ru") {
            if let Ok(re) = Regex::new(r"video-?\d+_\d+") {
                if let Some(mat) = re.find(url) {
                    return format!("https://vk.com/{}", mat.as_str());
                }
            }
        }
        // Handle Dzen URLs - strip query parameters and normalize article URLs
        if url.contains("dzen.ru") {
            let clean_url = if let Some(idx) = url.find('?') {
                &url[..idx]
            } else {
                url
            };

            // Convert article URLs (/a/) to video URLs if they redirect to video content
            // Article URLs often trigger a broken extractor, so we skip them
            // and let yt-dlp handle the URL as-is after cleaning
            return clean_url.to_string();
        }
        url.to_string()
    }

    fn perform_download<F>(
        url: &str,
        output_path: &str,
        overwrite: bool,
        on_progress: F,
    ) -> Result<String>
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        info!("Performing download of {} to {}", url, output_path);

        // If output_path ends with an extension, treat it as a full file path
        // Otherwise treat it as a directory (legacy behavior, though UI now provides full path)
        let is_file_path = output_path.ends_with(".mp4")
            || output_path.ends_with(".mkv")
            || output_path.ends_with(".webm");

        // Determine working directory for temp files
        let working_dir = if is_file_path {
            Path::new(output_path).parent().unwrap_or(Path::new("."))
        } else {
            Path::new(output_path)
        };

        let output_template = if is_file_path {
            output_path.to_string()
        } else {
            format!("{}/%%(title)s.%%(ext)s", output_path)
        };

        info!("Using output template for validation: {}", output_template);

        // The youtube_dl crate needs the template without %% escaping
        let crate_template = if is_file_path {
            output_path.to_string()
        } else {
            format!("{}/%(title)s.%(ext)s", output_path)
        };

        let result = YoutubeDl::new(url)
            .socket_timeout("30")
            .extract_audio(false)
            .output_template(&crate_template)
            .extra_arg("--simulate")
            .run();

        match result {
            Ok(YoutubeDlOutput::Playlist(_playlist)) => {
                warn!("Playlist detected, downloading first video only");
                Self::handle_playlist_download(
                    url,
                    output_path,
                    is_file_path,
                    overwrite,
                    on_progress,
                )
            }
            Ok(YoutubeDlOutput::SingleVideo(video)) => {
                let video_title = video.title.clone().unwrap_or_else(|| "video".to_string());
                info!("Metadata fetched: {}", video_title);

                // Perform actual download
                info!("Executing yt-dlp with output template: {}", output_template);

                // Get list of files before download to detect new file
                let files_before: std::collections::HashSet<_> = if !is_file_path {
                    std::fs::read_dir(output_path)
                        .ok()
                        .map(|entries| {
                            entries
                                .filter_map(|e| e.ok())
                                .filter_map(|e| e.file_name().into_string().ok())
                                .collect()
                        })
                        .unwrap_or_default()
                } else {
                    std::collections::HashSet::new()
                };

                let mut cmd = Command::new("yt-dlp");
                cmd.current_dir(working_dir);

                // For Command::arg(), we don't need %% escaping since it doesn't go through shell
                let cmd_template = if is_file_path {
                    output_path.to_string()
                } else {
                    format!("{}/%(title)s.%(ext)s", output_path)
                };

                info!("Passing to yt-dlp command: {}", cmd_template);

                cmd.arg(url).arg("-o").arg(&cmd_template).arg("--newline"); // Force newlines for progress parsing

                if overwrite {
                    cmd.arg("--force-overwrite");
                }

                cmd.stdout(Stdio::piped());

                let mut child = cmd.spawn().map_err(|e| {
                    DownloadError::IoError(format!("Failed to execute yt-dlp: {}", e))
                })?;

                if let Some(stdout) = child.stdout.take() {
                    let reader = BufReader::new(stdout);
                    for line in reader.lines().map_while(std::result::Result::ok) {
                        // Print to terminal so user sees progress there too
                        println!("{}", line);

                        // Parse progress
                        // [download]  45.0% of 10.00MiB at 2.00MiB/s ETA 00:05
                        if line.starts_with("[download]") && line.contains("%") {
                            if let Some(pct_str) = line.split_whitespace().nth(1) {
                                if let Ok(pct_val) = pct_str.trim_end_matches('%').parse::<f32>() {
                                    on_progress(pct_val / 100.0);
                                }
                            }
                        }
                    }
                }

                let status = child.wait().map_err(|e| {
                    DownloadError::IoError(format!("Failed to wait for yt-dlp: {}", e))
                })?;

                if !status.success() {
                    return Err(DownloadError::DownloadFailed(
                        "Download process failed".to_string(),
                    ));
                }

                info!("Download completed: {}", video_title);

                // Detect the actual downloaded file by comparing directory contents
                if is_file_path {
                    Ok(output_path.to_string())
                } else {
                    // Find newly created file
                    let files_after: std::collections::HashSet<_> = std::fs::read_dir(output_path)
                        .ok()
                        .map(|entries| {
                            entries
                                .filter_map(|e| e.ok())
                                .filter_map(|e| e.file_name().into_string().ok())
                                .collect()
                        })
                        .unwrap_or_default();

                    let new_files: Vec<_> = files_after.difference(&files_before).collect();

                    if let Some(filename) = new_files.first() {
                        let full_path = format!("{}/{}", output_path, filename);
                        info!("Detected downloaded file: {}", full_path);
                        Ok(full_path)
                    } else {
                        // Fallback to constructed path
                        let ext = video.ext.clone().unwrap_or("mp4".to_string());
                        Ok(format!("{}/{}.{}", output_path, video_title, ext))
                    }
                }
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
                } else if error_msg.contains("KeyError")
                    && error_msg.contains("exportResponse")
                    && url.contains("dzen.ru")
                {
                    warn!(
                        "Dzen article/channel URL detected (extractor broken): {}",
                        e
                    );
                    Err(DownloadError::DownloadFailed(
                        "Dzen article/channel URLs are currently not supported by yt-dlp. Please use direct video URLs (dzen.ru/video/watch/...)".to_string(),
                    ))
                } else {
                    warn!("Download error: {}", e);
                    Err(DownloadError::DownloadFailed(error_msg))
                }
            }
        }
    }

    fn handle_playlist_download<F>(
        url: &str,
        output_path: &str,
        is_file_path: bool,
        overwrite: bool,
        on_progress: F,
    ) -> Result<String>
    where
        F: Fn(f32) + Send + Sync + 'static,
    {
        // Determine working directory for temp files
        let working_dir = if is_file_path {
            Path::new(output_path).parent().unwrap_or(Path::new("."))
        } else {
            Path::new(output_path)
        };

        let output_template = if is_file_path {
            output_path.to_string()
        } else {
            format!("{}/%%(title)s.%%(ext)s", output_path)
        };

        info!(
            "Using output template for playlist download: {}",
            output_template
        );

        // The youtube_dl crate needs the template without %% escaping
        let crate_template = if is_file_path {
            output_path.to_string()
        } else {
            format!("{}/%(title)s.%(ext)s", output_path)
        };

        match YoutubeDl::new(url)
            .socket_timeout("30")
            .extract_audio(false)
            .output_template(&crate_template)
            .playlist_items(1u32)
            .run()
        {
            Ok(YoutubeDlOutput::Playlist(playlist)) => {
                if let Some(entries) = playlist.entries {
                    if let Some(video) = entries.first() {
                        // Get list of files before download
                        let files_before: std::collections::HashSet<_> = if !is_file_path {
                            std::fs::read_dir(output_path)
                                .ok()
                                .map(|entries| {
                                    entries
                                        .filter_map(|e| e.ok())
                                        .filter_map(|e| e.file_name().into_string().ok())
                                        .collect()
                                })
                                .unwrap_or_default()
                        } else {
                            std::collections::HashSet::new()
                        };

                        // Perform actual download for the first item
                        let mut cmd = Command::new("yt-dlp");
                        cmd.current_dir(working_dir);

                        // For Command::arg(), we don't need %% escaping since it doesn't go through shell
                        let cmd_template = if is_file_path {
                            output_path.to_string()
                        } else {
                            format!("{}/%(title)s.%(ext)s", output_path)
                        };

                        info!("Passing to yt-dlp command (playlist): {}", cmd_template);

                        cmd.arg(url)
                            .arg("-o")
                            .arg(&cmd_template)
                            .arg("--playlist-items")
                            .arg("1")
                            .arg("--newline"); // Force newlines for progress parsing

                        if overwrite {
                            cmd.arg("--force-overwrite");
                        }

                        cmd.stdout(Stdio::piped());

                        let mut child = cmd.spawn().map_err(|e| {
                            DownloadError::IoError(format!("Failed to execute yt-dlp: {}", e))
                        })?;

                        if let Some(stdout) = child.stdout.take() {
                            let reader = BufReader::new(stdout);
                            for line in reader.lines().map_while(std::result::Result::ok) {
                                // Print to terminal
                                println!("{}", line);

                                if line.starts_with("[download]") && line.contains("%") {
                                    if let Some(pct_str) = line.split_whitespace().nth(1) {
                                        if let Ok(pct_val) =
                                            pct_str.trim_end_matches('%').parse::<f32>()
                                        {
                                            on_progress(pct_val / 100.0);
                                        }
                                    }
                                }
                            }
                        }

                        let status = child.wait().map_err(|e| {
                            DownloadError::IoError(format!("Failed to wait for yt-dlp: {}", e))
                        })?;

                        if !status.success() {
                            return Err(DownloadError::DownloadFailed(
                                "Download process failed".to_string(),
                            ));
                        }

                        // Detect the actual downloaded file by comparing directory contents
                        if is_file_path {
                            Ok(output_path.to_string())
                        } else {
                            // Find newly created file
                            let files_after: std::collections::HashSet<_> =
                                std::fs::read_dir(output_path)
                                    .ok()
                                    .map(|entries| {
                                        entries
                                            .filter_map(|e| e.ok())
                                            .filter_map(|e| e.file_name().into_string().ok())
                                            .collect()
                                    })
                                    .unwrap_or_default();

                            let new_files: Vec<_> = files_after.difference(&files_before).collect();

                            if let Some(filename) = new_files.first() {
                                let full_path = format!("{}/{}", output_path, filename);
                                info!("Detected downloaded file from playlist: {}", full_path);
                                Ok(full_path)
                            } else {
                                // Fallback to constructed path
                                let title = video.title.clone().unwrap_or("video".to_string());
                                let ext = video.ext.clone().unwrap_or("mp4".to_string());
                                Ok(format!("{}/{}.{}", output_path, title, ext))
                            }
                        }
                    } else {
                        Err(DownloadError::DownloadFailed("Empty playlist".to_string()))
                    }
                } else {
                    Err(DownloadError::DownloadFailed(
                        "Failed to get playlist entries".to_string(),
                    ))
                }
            }
            Ok(YoutubeDlOutput::SingleVideo(video)) => {
                // Should not happen if playlist_items is used, but handle it just in case
                let video_title = video.title.clone().unwrap_or_else(|| "video".to_string());
                let ext = video.ext.clone().unwrap_or("mp4".to_string());

                let status = std::process::Command::new("yt-dlp")
                    .arg(url)
                    .arg("-o")
                    .arg(&output_template)
                    .status()
                    .map_err(|e| {
                        DownloadError::IoError(format!("Failed to execute yt-dlp: {}", e))
                    })?;

                if !status.success() {
                    return Err(DownloadError::DownloadFailed(
                        "Download process failed".to_string(),
                    ));
                }

                if is_file_path {
                    Ok(output_path.to_string())
                } else {
                    Ok(format!("{}/{}.{}", output_path, video_title, ext))
                }
            }
            Err(e) => Err(DownloadError::DownloadFailed(e.to_string())),
        }
    }

    #[allow(dead_code)]
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

    #[test]
    fn test_sanitize_url_vk() {
        let url = "https://vkvideo.ru/playlist/-220754053_3/video-220754053_456244420?linked=1";
        let sanitized = VideoDownloader::sanitize_url(url);
        assert_eq!(sanitized, "https://vk.com/video-220754053_456244420");

        let url_simple = "https://vk.com/video-220754053_456244420";
        let sanitized_simple = VideoDownloader::sanitize_url(url_simple);
        assert_eq!(sanitized_simple, "https://vk.com/video-220754053_456244420");

        let url_no_video = "https://vkvideo.ru/playlist/-220754053_3";
        let sanitized_no_video = VideoDownloader::sanitize_url(url_no_video);
        assert_eq!(sanitized_no_video, url_no_video);
    }

    #[test]
    fn test_sanitize_url_dzen() {
        let url =
            "https://dzen.ru/video/watch/634b04f2596d3e28c83c233e?rid=12345&referrer_clid=1400";
        let sanitized = VideoDownloader::sanitize_url(url);
        assert_eq!(
            sanitized,
            "https://dzen.ru/video/watch/634b04f2596d3e28c83c233e"
        );

        let url_clean = "https://dzen.ru/video/watch/634b04f2596d3e28c83c233e";
        let sanitized_clean = VideoDownloader::sanitize_url(url_clean);
        assert_eq!(sanitized_clean, url_clean);
    }
}
