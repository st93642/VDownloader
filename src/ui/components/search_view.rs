/*****************************************************************************/
/*                                                                           */
/*  search_view.rs                                       TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 07 2025 19:00 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 07 2025 19:10 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/

use crate::core::search::{SearchError, SearchResult, SearchService};
use crate::ui::components::preview_window::PreviewWindow;
use gdk_pixbuf::{Pixbuf, PixbufLoader};
use gtk4::{
    prelude::*, ApplicationWindow, Button, Image, Label, ListBox, Orientation, ScrolledWindow,
    SearchEntry, Spinner,
};
use log::{debug, warn};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

const THUMBNAIL_WIDTH: i32 = 120;
const THUMBNAIL_HEIGHT: i32 = 90;

type DownloadCallback = std::boxed::Box<dyn Fn(SearchResult)>;
type ThumbnailCache = HashMap<String, Pixbuf>;

pub struct SearchView {
    pub container: gtk4::Box,
    search_service: SearchService,
    search_entry: SearchEntry,
    search_button: Button,
    spinner: Spinner,
    status_label: Label,
    results_list: ListBox,
    thumbnail_cache: Rc<RefCell<ThumbnailCache>>,
    download_callback: Rc<RefCell<Option<DownloadCallback>>>,
    window: Rc<RefCell<Option<ApplicationWindow>>>,
}

impl SearchView {
    pub fn new() -> Self {
        Self::new_with_service(SearchService::new(10))
    }

    pub fn new_with_service(search_service: SearchService) -> Self {
        let container = gtk4::Box::new(Orientation::Vertical, 12);
        container.set_margin_top(12);
        container.set_margin_bottom(12);
        container.set_margin_start(12);
        container.set_margin_end(12);

        let search_box = gtk4::Box::new(Orientation::Horizontal, 12);

        let search_entry = SearchEntry::builder()
            .placeholder_text("Search for videos...")
            .hexpand(true)
            .build();

        let search_button = Button::with_label("Search");
        search_button.add_css_class("suggested-action");

        search_box.append(&search_entry);
        search_box.append(&search_button);

        let spinner = Spinner::builder().margin_top(6).build();
        spinner.set_visible(false);

        let status_label = Label::new(Some("Enter a search query to begin"));
        status_label.set_halign(gtk4::Align::Start);
        status_label.set_margin_top(6);
        status_label.add_css_class("dim-label");
        status_label.set_selectable(true);

        let scrolled_window = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .vscrollbar_policy(gtk4::PolicyType::Automatic)
            .min_content_height(300)
            .margin_top(12)
            .build();

        let results_list = ListBox::builder()
            .selection_mode(gtk4::SelectionMode::None)
            .build();
        results_list.add_css_class("boxed-list");

        scrolled_window.set_child(Some(&results_list));

        container.append(&search_box);
        container.append(&spinner);
        container.append(&status_label);
        container.append(&scrolled_window);

        let thumbnail_cache = Rc::new(RefCell::new(HashMap::new()));
        let download_callback = Rc::new(RefCell::new(None));
        let window = Rc::new(RefCell::new(None));

        let mut view = Self {
            container,
            search_service,
            search_entry: search_entry.clone(),
            search_button: search_button.clone(),
            spinner,
            status_label: status_label.clone(),
            results_list: results_list.clone(),
            thumbnail_cache: thumbnail_cache.clone(),
            download_callback: download_callback.clone(),
            window: window.clone(),
        };

        view.connect_search_signals();
        view
    }

    pub fn set_download_callback<F>(&self, callback: F)
    where
        F: Fn(SearchResult) + 'static,
    {
        *self.download_callback.borrow_mut() = Some(std::boxed::Box::new(callback));
    }

    pub fn set_window(&self, window: ApplicationWindow) {
        *self.window.borrow_mut() = Some(window);
    }

    fn connect_search_signals(&mut self) {
        let search_entry = self.search_entry.clone();
        let search_button = self.search_button.clone();
        let spinner = self.spinner.clone();
        let status_label = self.status_label.clone();
        let results_list = self.results_list.clone();
        let search_service = self.search_service;
        let thumbnail_cache = self.thumbnail_cache.clone();
        let download_callback = self.download_callback.clone();
        let window = self.window.clone();

        let perform_search = Rc::new(move || {
            let query = search_entry.text();
            if query.trim().is_empty() {
                status_label.set_label("Error: Please enter a search query");
                status_label.remove_css_class("dim-label");
                status_label.add_css_class("error");
                return;
            }

            search_entry.set_sensitive(false);
            search_button.set_sensitive(false);
            spinner.set_visible(true);
            spinner.start();

            status_label.remove_css_class("error");
            status_label.add_css_class("dim-label");
            status_label.set_label("Searching...");

            while let Some(child) = results_list.first_child() {
                results_list.remove(&child);
            }

            let search_entry_clone = search_entry.clone();
            let search_button_clone = search_button.clone();
            let spinner_clone = spinner.clone();
            let status_label_clone = status_label.clone();
            let results_list_clone = results_list.clone();
            let search_service_clone = search_service;
            let thumbnail_cache_clone = thumbnail_cache.clone();
            let download_callback_clone = download_callback.clone();
            let window_clone = window.clone();
            let query_clone = query.to_string();

            gtk4::glib::spawn_future_local(async move {
                match search_service_clone.search(&query_clone, None).await {
                    Ok(results) => {
                        search_entry_clone.set_sensitive(true);
                        search_button_clone.set_sensitive(true);
                        spinner_clone.stop();
                        spinner_clone.set_visible(false);

                        if results.is_empty() {
                            status_label_clone.set_label("No results found");
                            status_label_clone.remove_css_class("dim-label");
                            status_label_clone.add_css_class("warning");
                        } else {
                            status_label_clone
                                .set_label(&format!("Found {} result(s)", results.len()));
                            status_label_clone.remove_css_class("warning");
                            status_label_clone.add_css_class("dim-label");

                            for result in results {
                                let card = Self::create_result_card(
                                    &result,
                                    &download_callback_clone,
                                    &window_clone,
                                );
                                results_list_clone.append(&card);

                                if let Some(ref thumbnail_url) = result.thumbnail {
                                    Self::load_thumbnail(
                                        thumbnail_url.clone(),
                                        card.clone(),
                                        thumbnail_cache_clone.clone(),
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        search_entry_clone.set_sensitive(true);
                        search_button_clone.set_sensitive(true);
                        spinner_clone.stop();
                        spinner_clone.set_visible(false);

                        let error_msg = Self::format_search_error(&e);
                        status_label_clone.set_label(&error_msg);
                        status_label_clone.remove_css_class("dim-label");
                        status_label_clone.add_css_class("error");
                    }
                }
            });
        });

        let perform_search_clone = perform_search.clone();
        self.search_button.connect_clicked(move |_| {
            perform_search_clone();
        });

        self.search_entry.connect_activate(move |_| {
            perform_search();
        });
    }

    fn create_result_card(
        result: &SearchResult,
        download_callback: &Rc<RefCell<Option<DownloadCallback>>>,
        window: &Rc<RefCell<Option<ApplicationWindow>>>,
    ) -> gtk4::Box {
        let card = gtk4::Box::new(Orientation::Horizontal, 12);
        card.set_margin_top(6);
        card.set_margin_bottom(6);
        card.set_margin_start(12);
        card.set_margin_end(12);

        let placeholder_pixbuf = Self::create_placeholder_pixbuf();
        let thumbnail = Image::from_pixbuf(Some(&placeholder_pixbuf));
        thumbnail.set_pixel_size(THUMBNAIL_WIDTH);

        card.append(&thumbnail);

        let info_box = gtk4::Box::new(Orientation::Vertical, 6);
        info_box.set_hexpand(true);

        let title_label = Label::new(Some(&result.title));
        title_label.set_halign(gtk4::Align::Start);
        title_label.set_wrap(true);
        title_label.set_wrap_mode(gtk4::pango::WrapMode::WordChar);
        title_label.set_max_width_chars(50);
        title_label.add_css_class("heading");

        info_box.append(&title_label);

        if let Some(ref uploader) = result.uploader {
            let uploader_label = Label::new(Some(uploader));
            uploader_label.set_halign(gtk4::Align::Start);
            uploader_label.add_css_class("dim-label");
            info_box.append(&uploader_label);
        }

        let mut metadata_parts = Vec::new();

        if let Some(duration) = result.duration {
            metadata_parts.push(Self::format_duration(duration));
        }

        if let Some(views) = result.view_count {
            metadata_parts.push(Self::format_views(views));
        }

        if !metadata_parts.is_empty() {
            let metadata_text = metadata_parts.join(" • ");
            let metadata_label = Label::new(Some(&metadata_text));
            metadata_label.set_halign(gtk4::Align::Start);
            metadata_label.add_css_class("dim-label");
            metadata_label.add_css_class("caption");
            info_box.append(&metadata_label);
        }

        let platform_label = Label::new(Some(&format!("{:?}", result.platform)));
        platform_label.set_halign(gtk4::Align::Start);
        platform_label.add_css_class("dim-label");
        platform_label.add_css_class("caption");
        info_box.append(&platform_label);

        card.append(&info_box);

        // Button box for Preview and Download
        let button_box = gtk4::Box::new(Orientation::Vertical, 6);
        button_box.set_valign(gtk4::Align::Center);

        // Preview button
        let preview_button = Button::with_label("Preview");

        let result_clone_preview = result.clone();
        let window_clone = window.clone();
        preview_button.connect_clicked(move |_| {
            if let Some(ref parent_window) = *window_clone.borrow() {
                let preview = PreviewWindow::new(parent_window, &result_clone_preview);
                preview.present();
            }
        });

        button_box.append(&preview_button);

        // Download button
        let download_button = Button::with_label("Download");
        download_button.add_css_class("suggested-action");

        let result_clone = result.clone();
        let download_callback_clone = download_callback.clone();
        download_button.connect_clicked(move |_| {
            if let Some(ref callback) = *download_callback_clone.borrow() {
                callback(result_clone.clone());
            }
        });

        button_box.append(&download_button);
        card.append(&button_box);

        card
    }

    fn create_placeholder_pixbuf() -> Pixbuf {
        Pixbuf::new(
            gdk_pixbuf::Colorspace::Rgb,
            false,
            8,
            THUMBNAIL_WIDTH,
            THUMBNAIL_HEIGHT,
        )
        .unwrap()
    }

    fn load_thumbnail(url: String, card: gtk4::Box, cache: Rc<RefCell<ThumbnailCache>>) {
        if let Some(cached_pixbuf) = cache.borrow().get(&url) {
            if let Some(thumbnail) = card.first_child().and_then(|w| w.downcast::<Image>().ok()) {
                thumbnail.set_from_pixbuf(Some(cached_pixbuf));
            }
            return;
        }

        let cache_clone = cache.clone();
        let url_clone = url.clone();

        gtk4::glib::spawn_future_local(async move {
            match Self::fetch_thumbnail(&url).await {
                Ok(pixbuf) => {
                    cache_clone.borrow_mut().insert(url_clone, pixbuf.clone());

                    if let Some(thumbnail) =
                        card.first_child().and_then(|w| w.downcast::<Image>().ok())
                    {
                        thumbnail.set_from_pixbuf(Some(&pixbuf));
                    }
                }
                Err(e) => {
                    warn!("Failed to load thumbnail from {}: {}", url, e);
                }
            }
        });
    }

    async fn fetch_thumbnail(url: &str) -> Result<Pixbuf, Box<dyn std::error::Error>> {
        debug!("Fetching thumbnail: {}", url);

        let response = reqwest::get(url).await?;

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err("Rate limited (HTTP 429)".into());
        }

        if !response.status().is_success() {
            return Err(format!("HTTP error: {}", response.status()).into());
        }

        let bytes = response.bytes().await?;

        let loader = PixbufLoader::new();
        loader.write(&bytes)?;
        loader.close()?;

        let pixbuf = loader
            .pixbuf()
            .ok_or("Failed to load pixbuf from image data")?;

        let scaled_pixbuf = pixbuf
            .scale_simple(
                THUMBNAIL_WIDTH,
                THUMBNAIL_HEIGHT,
                gdk_pixbuf::InterpType::Bilinear,
            )
            .ok_or("Failed to scale pixbuf")?;

        Ok(scaled_pixbuf)
    }

    fn format_duration(seconds: u64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, secs)
        } else {
            format!("{}:{:02}", minutes, secs)
        }
    }

    fn format_views(count: u64) -> String {
        if count >= 1_000_000 {
            format!("{:.1}M views", count as f64 / 1_000_000.0)
        } else if count >= 1_000 {
            format!("{:.1}K views", count as f64 / 1_000.0)
        } else {
            format!("{} views", count)
        }
    }

    fn format_search_error(error: &SearchError) -> String {
        match error {
            SearchError::InvalidQuery(msg) => format!("Error: Invalid query - {}", msg),
            SearchError::CommandFailed(msg) => {
                format!("Error: Search failed - {}", msg)
            }
            SearchError::JsonParseError(msg) => {
                format!("Error: Failed to parse results - {}", msg)
            }
            SearchError::MissingYtDlp => {
                "Error: yt-dlp not found. Please install yt-dlp to use search".to_string()
            }
            SearchError::RateLimited(msg) => {
                format!("Error: Rate limited - {}. Please try again later", msg)
            }
            SearchError::IoError(msg) => format!("Error: I/O error - {}", msg),
        }
    }
}

impl Default for SearchView {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_duration() {
        assert_eq!(SearchView::format_duration(45), "0:45");
        assert_eq!(SearchView::format_duration(150), "2:30");
        assert_eq!(SearchView::format_duration(3661), "1:01:01");
        assert_eq!(SearchView::format_duration(7325), "2:02:05");
    }

    #[test]
    fn test_format_views() {
        assert_eq!(SearchView::format_views(42), "42 views");
        assert_eq!(SearchView::format_views(999), "999 views");
        assert_eq!(SearchView::format_views(1500), "1.5K views");
        assert_eq!(SearchView::format_views(15000), "15.0K views");
        assert_eq!(SearchView::format_views(1500000), "1.5M views");
        assert_eq!(SearchView::format_views(42000000), "42.0M views");
    }

    #[test]
    fn test_format_search_error() {
        let error = SearchError::InvalidQuery("Empty query".to_string());
        let msg = SearchView::format_search_error(&error);
        assert!(msg.contains("Invalid query"));

        let error = SearchError::MissingYtDlp;
        let msg = SearchView::format_search_error(&error);
        assert!(msg.contains("yt-dlp not found"));

        let error = SearchError::RateLimited("Too many requests".to_string());
        let msg = SearchView::format_search_error(&error);
        assert!(msg.contains("Rate limited"));
    }
}
