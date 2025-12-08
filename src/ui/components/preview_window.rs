/*****************************************************************************/
/*                                                                           */
/*  preview_window.rs                                    TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 07 2025 20:30 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 08 2025 18:08 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/

use crate::core::search::SearchResult;
use gtk4::{prelude::*, ApplicationWindow, Box, Button, Label, Orientation, Spinner, Video};
use log::{debug, info, warn};
use std::process::Command as StdCommand;

pub struct PreviewWindow {
    window: ApplicationWindow,
}

impl PreviewWindow {
    pub fn new(parent: &ApplicationWindow, result: &SearchResult) -> Self {
        info!("Creating preview window for: {}", result.title);

        let window = ApplicationWindow::builder()
            .transient_for(parent)
            .modal(true)
            .title(format!("Preview: {}", result.title))
            .default_width(800)
            .default_height(600)
            .build();

        let container = Box::new(Orientation::Vertical, 12);
        container.set_margin_top(12);
        container.set_margin_bottom(12);
        container.set_margin_start(12);
        container.set_margin_end(12);

        // Create loading spinner
        let spinner = Spinner::builder()
            .hexpand(true)
            .vexpand(true)
            .spinning(true)
            .build();

        let loading_box = Box::new(Orientation::Vertical, 12);
        loading_box.set_vexpand(true);
        loading_box.set_valign(gtk4::Align::Center);

        let loading_label = Label::new(Some("Loading video..."));
        loading_label.add_css_class("dim-label");

        loading_box.append(&spinner);
        loading_box.append(&loading_label);

        container.append(&loading_box);

        // Create video player widget (initially hidden)
        let video = Video::builder()
            .autoplay(true)
            .hexpand(true)
            .vexpand(true)
            .visible(false)
            .build();

        container.append(&video);

        // Fetch actual video URL asynchronously
        let video_clone = video.clone();
        let loading_box_clone = loading_box.clone();
        let url = result.url.clone();
        let platform = result.platform.clone();

        gtk4::glib::spawn_future_local(async move {
            debug!(
                "Fetching video stream URL for: {} (platform: {:?})",
                url, platform
            );

            match Self::get_video_url(&url, &platform).await {
                Ok(stream_url) => {
                    info!("Got stream URL, setting up playback");
                    let file = gtk4::gio::File::for_uri(&stream_url);
                    video_clone.set_file(Some(&file));
                    loading_box_clone.set_visible(false);
                    video_clone.set_visible(true);
                }
                Err(e) => {
                    warn!("Failed to get video URL for {:?}: {}", platform, e);
                    loading_box_clone.set_visible(false);

                    // Show error message
                    let error_box = Box::new(Orientation::Vertical, 12);
                    error_box.set_vexpand(true);
                    error_box.set_valign(gtk4::Align::Center);

                    let error_label = Label::new(Some(&format!("Failed to load video: {}", e)));
                    error_label.add_css_class("error");
                    error_label.set_wrap(true);

                    error_box.append(&error_label);

                    if let Some(parent) = loading_box_clone.parent() {
                        if let Ok(container) = parent.downcast::<Box>() {
                            container.append(&error_box);
                        }
                    }
                }
            }
        });

        // Create info section
        let info_box = Box::new(Orientation::Vertical, 6);
        info_box.set_margin_top(12);

        let title_label = Label::new(Some(&result.title));
        title_label.set_halign(gtk4::Align::Start);
        title_label.set_wrap(true);
        title_label.add_css_class("title-2");
        info_box.append(&title_label);

        if let Some(ref uploader) = result.uploader {
            let uploader_label = Label::new(Some(&format!("By: {}", uploader)));
            uploader_label.set_halign(gtk4::Align::Start);
            uploader_label.add_css_class("dim-label");
            info_box.append(&uploader_label);
        }

        container.append(&info_box);

        // Create control buttons
        let button_box = Box::new(Orientation::Horizontal, 12);
        button_box.set_halign(gtk4::Align::End);
        button_box.set_margin_top(12);

        let close_button = Button::with_label("Close");
        let window_clone = window.clone();
        close_button.connect_clicked(move |_| {
            window_clone.close();
        });

        button_box.append(&close_button);
        container.append(&button_box);

        window.set_child(Some(&container));

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }

    async fn get_video_url(
        url: &str,
        platform: &crate::core::downloader::Platform,
    ) -> Result<String, String> {
        debug!(
            "Extracting video URL from: {} (platform: {:?})",
            url, platform
        );

        // Try multiple format strategies for better compatibility
        // Key: Use formats that return a SINGLE URL that can be played directly
        let formats = match platform {
            crate::core::downloader::Platform::Dzen => {
                // Dzen needs formats that provide single-stream URLs
                vec![
                    "best[ext=mp4]",     // Best MP4 (usually has muxed audio)
                    "best",              // Best any format
                    "best[height<=720]", // 720p or lower
                    "worst",             // Fallback to worst
                ]
            }
            crate::core::downloader::Platform::Rutube => {
                // Rutube typically has good format support
                vec![
                    "best[ext=mp4]",      // Best MP4
                    "best[height<=1080]", // Up to 1080p
                    "best",               // Any best format
                    "best[height<=720]",  // 720p fallback
                ]
            }
            _ => {
                // For other platforms (YouTube, etc.)
                vec![
                    "best[ext=mp4]",      // Prefer MP4
                    "best[height<=1080]", // Up to 1080p
                    "best",               // Any format
                    "worst",              // Fallback
                ]
            }
        };

        let mut last_error = String::new();

        for (i, format) in formats.iter().enumerate() {
            info!(
                "Attempt {} - Using format: {} for URL: {}",
                i + 1,
                format,
                url
            );

            // Use yt-dlp to get the direct video stream URL
            let output = tokio::task::spawn_blocking({
                let url = url.to_string();
                let format = format.to_string();
                move || {
                    StdCommand::new("yt-dlp")
                        .arg("--get-url")
                        .arg("--format")
                        .arg(&format)
                        .arg("--no-warnings")
                        .arg(&url)
                        .output()
                }
            })
            .await
            .map_err(|e| format!("Task join error: {}", e))?
            .map_err(|e| format!("Failed to execute yt-dlp: {}", e))?;

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.trim().lines().collect();

                debug!(
                    "yt-dlp returned {} line(s) for format '{}'",
                    lines.len(),
                    format
                );

                // Check if yt-dlp returned multiple URLs (video+audio that need merging)
                if lines.len() > 1 {
                    warn!(
                        "Format '{}' returned {} URLs (requires merging, not supported)",
                        format,
                        lines.len()
                    );
                    warn!("URLs: {:?}", lines);
                    last_error = "Format returned multiple streams that need merging".to_string();
                    continue;
                }

                let video_url = lines
                    .first()
                    .ok_or("No URL returned from yt-dlp")?
                    .to_string();

                if !video_url.is_empty() {
                    info!("Successfully extracted video URL with format: {}", format);
                    debug!("Video URL: {}", video_url);
                    return Ok(video_url);
                } else {
                    warn!("Empty URL returned for format '{}'", format);
                    last_error = "Empty URL returned".to_string();
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                warn!(
                    "Format '{}' failed with exit code: {:?}",
                    format,
                    output.status.code()
                );
                warn!("Error output: {}", stderr);
                last_error = stderr.to_string();
            }
        }

        // All formats failed, return helpful error
        warn!("All format attempts failed. Last error: {}", last_error);

        if last_error.contains("Unsupported URL") {
            Err("This video URL is not supported for preview".to_string())
        } else if last_error.contains("Video unavailable") || last_error.contains("Private video") {
            Err("Video is unavailable or private".to_string())
        } else if last_error.contains("dzen")
            && (last_error.contains("KeyError") || last_error.contains("exportResponse"))
        {
            Err("Dzen video preview not available. Try downloading instead.".to_string())
        } else if last_error.contains("ERROR") {
            let error_line = last_error
                .lines()
                .find(|line| line.contains("ERROR"))
                .unwrap_or("Unknown error");
            Err(format!("Preview failed: {}", error_line))
        } else {
            Err(
                "Could not extract video URL. This video may not be supported for preview."
                    .to_string(),
            )
        }
    }
}
