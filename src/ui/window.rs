use crate::core::downloader::{DownloadRequest, VideoDownloader};
use crate::core::error::DownloadError;
use gtk4::{
    prelude::*, Application, ApplicationWindow, Box, Button, Entry, FileDialog, Label, Orientation,
    ProgressBar,
};
use log::info;
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("VDownloader")
        .default_width(600)
        .default_height(300)
        .build();

    let main_box = Box::new(Orientation::Vertical, 12);
    main_box.set_margin_top(24);
    main_box.set_margin_bottom(24);
    main_box.set_margin_start(24);
    main_box.set_margin_end(24);

    // Header
    let header = Label::new(Some("VDownloader"));
    header.add_css_class("title-1");

    let subtitle = Label::new(Some("Download videos from multiple platforms"));
    subtitle.add_css_class("dim-label");

    // URL input section
    let url_label = Label::new(Some("Video URL:"));
    url_label.set_halign(gtk4::Align::Start);
    url_label.set_margin_top(12);

    let url_entry = Entry::builder()
        .placeholder_text("Enter video URL here...")
        .hexpand(true)
        .build();

    // Download directory section
    let dir_label = Label::new(Some("Download Directory:"));
    dir_label.set_halign(gtk4::Align::Start);
    dir_label.set_margin_top(12);

    let dir_box = Box::new(Orientation::Horizontal, 12);

    let selected_path = Rc::new(RefCell::new(
        std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| String::from(".")),
    ));

    let path_label = Label::new(Some(&*selected_path.borrow()));
    path_label.set_halign(gtk4::Align::Start);
    path_label.set_hexpand(true);
    path_label.set_ellipsize(gtk4::pango::EllipsizeMode::Middle);
    path_label.add_css_class("monospace");

    let browse_button = Button::with_label("Browse...");

    let window_clone = window.clone();
    let path_label_clone = path_label.clone();
    let selected_path_clone = selected_path.clone();
    browse_button.connect_clicked(move |_| {
        let dialog = FileDialog::builder()
            .title("Select Download Directory")
            .modal(true)
            .build();

        let path_label_clone2 = path_label_clone.clone();
        let selected_path_clone2 = selected_path_clone.clone();

        dialog.select_folder(
            Some(&window_clone),
            None::<&gtk4::gio::Cancellable>,
            move |result| {
                if let Ok(file) = result {
                    if let Some(path) = file.path() {
                        let path_str = path.display().to_string();
                        info!("Selected download directory: {}", path_str);
                        *selected_path_clone2.borrow_mut() = path_str.clone();
                        path_label_clone2.set_label(&path_str);
                    }
                }
            },
        );
    });

    dir_box.append(&path_label);
    dir_box.append(&browse_button);

    // Progress bar
    let progress_bar = ProgressBar::builder()
        .margin_top(12)
        .show_text(true)
        .build();
    progress_bar.set_visible(false);

    // Status label
    let status_label = Label::new(Some("Ready to download"));
    status_label.set_halign(gtk4::Align::Start);
    status_label.set_margin_top(12);
    status_label.add_css_class("dim-label");

    // Download button
    let download_button = Button::with_label("Download");
    download_button.add_css_class("suggested-action");
    download_button.set_margin_top(12);

    let url_entry_clone = url_entry.clone();
    let selected_path_clone = selected_path.clone();
    let status_label_clone = status_label.clone();
    let progress_bar_clone = progress_bar.clone();
    download_button.connect_clicked(move |btn| {
        let url = url_entry_clone.text();
        let path = selected_path_clone.borrow().clone();

        if url.is_empty() {
            status_label_clone.set_label("Error: Please enter a video URL");
            status_label_clone.remove_css_class("dim-label");
            status_label_clone.add_css_class("error");
            return;
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            status_label_clone
                .set_label("Error: Invalid URL (must start with http:// or https://)");
            status_label_clone.remove_css_class("dim-label");
            status_label_clone.add_css_class("error");
            return;
        }

        info!("Download requested for URL: {} to path: {}", url, path);

        status_label_clone.remove_css_class("error");
        status_label_clone.add_css_class("dim-label");
        status_label_clone.set_label("Validating and starting download...");

        btn.set_label("Downloading...");
        btn.set_sensitive(false);
        progress_bar_clone.set_visible(true);
        progress_bar_clone.set_fraction(0.0);

        let btn_clone = btn.clone();
        let status_label_clone2 = status_label_clone.clone();
        let progress_bar_clone2 = progress_bar_clone.clone();
        let url_clone = url.to_string();

        gtk4::glib::spawn_future_local(async move {
            let downloader = VideoDownloader::new(path.clone());
            let platform = VideoDownloader::detect_platform(&url_clone);
            let request = DownloadRequest {
                url: url_clone.clone(),
                platform,
                output_path: Some(path.clone()),
            };

            progress_bar_clone2.set_fraction(0.5);
            progress_bar_clone2.set_text(Some("50%"));

            match downloader.download(request).await {
                Ok(file_path) => {
                    info!("Download successful: {}", file_path);
                    status_label_clone2.remove_css_class("dim-label");
                    status_label_clone2.add_css_class("success");
                    status_label_clone2
                        .set_label("Download completed! File saved to download directory");
                    progress_bar_clone2.set_fraction(1.0);
                    progress_bar_clone2.set_text(Some("100%"));
                    btn_clone.set_label("Download");
                    btn_clone.set_sensitive(true);

                    gtk4::glib::timeout_add_seconds_local_once(5, move || {
                        progress_bar_clone2.set_visible(false);
                    });
                }
                Err(e) => {
                    let error_msg = format_error(&e);
                    info!("Download failed: {}", error_msg);
                    status_label_clone2.remove_css_class("dim-label");
                    status_label_clone2.add_css_class("error");
                    status_label_clone2.set_label(&error_msg);
                    progress_bar_clone2.set_visible(false);
                    btn_clone.set_label("Download");
                    btn_clone.set_sensitive(true);
                }
            }
        });
    });

    // Assemble the UI
    main_box.append(&header);
    main_box.append(&subtitle);
    main_box.append(&url_label);
    main_box.append(&url_entry);
    main_box.append(&dir_label);
    main_box.append(&dir_box);
    main_box.append(&download_button);
    main_box.append(&progress_bar);
    main_box.append(&status_label);

    window.set_child(Some(&main_box));

    window
}

fn format_error(error: &DownloadError) -> String {
    match error {
        DownloadError::InvalidUrl(msg) => format!("Error: Invalid URL - {}", msg),
        DownloadError::InvalidOutputDirectory => {
            "Error: Download directory is invalid or not writable".to_string()
        }
        DownloadError::NetworkError(msg) => {
            format!("Error: Network issue - {}. Check your connection", msg)
        }
        DownloadError::DownloadFailed(msg) => format!("Error: Download failed - {}", msg),
        DownloadError::ExtractionError(msg) => format!("Error: Extraction failed - {}", msg),
        DownloadError::UnsupportedPlatform(platform) => {
            format!("Error: Platform not supported - {}", platform)
        }
        DownloadError::IoError(msg) => format!("Error: File system error - {}", msg),
        DownloadError::Cancelled => "Download was cancelled".to_string(),
        DownloadError::VideoNotFound => {
            "Error: Video not found or unavailable. Check URL and try again".to_string()
        }
    }
}
