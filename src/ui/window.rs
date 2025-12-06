use gtk4::{
    prelude::*, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ProgressBar,
};
use log::info;

pub fn build_window(app: &Application) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("VDownloader")
        .default_width(800)
        .default_height(600)
        .build();

    let main_box = Box::new(Orientation::Vertical, 12);
    main_box.set_margin_top(24);
    main_box.set_margin_bottom(24);
    main_box.set_margin_start(24);
    main_box.set_margin_end(24);

    let header = Label::new(Some("VDownloader"));
    header.add_css_class("title-1");

    let subtitle = Label::new(Some(
        "Download videos from YouTube, TikTok, Twitter, Instagram, and Reddit",
    ));
    subtitle.add_css_class("dim-label");

    let url_box = Box::new(Orientation::Horizontal, 12);
    let url_entry = Entry::builder()
        .placeholder_text("Enter video URL here...")
        .hexpand(true)
        .build();

    let download_button = Button::with_label("Download");
    download_button.add_css_class("suggested-action");

    let url_entry_clone = url_entry.clone();
    download_button.connect_clicked(move |btn| {
        let url = url_entry_clone.text();
        if !url.is_empty() {
            info!("Download requested for URL: {}", url);
            btn.set_label("Downloading...");
            btn.set_sensitive(false);
        }
    });

    url_box.append(&url_entry);
    url_box.append(&download_button);

    let progress_section = Box::new(Orientation::Vertical, 6);
    let progress_label = Label::new(Some("Ready to download"));
    progress_label.set_halign(gtk4::Align::Start);

    let progress_bar = ProgressBar::new();
    progress_bar.set_show_text(true);
    progress_bar.set_text(Some("0%"));

    progress_section.append(&progress_label);
    progress_section.append(&progress_bar);

    let queue_section = super::components::download_queue::create_queue_placeholder();

    main_box.append(&header);
    main_box.append(&subtitle);
    main_box.append(&url_box);
    main_box.append(&progress_section);
    main_box.append(&queue_section);

    window.set_child(Some(&main_box));

    window
}
