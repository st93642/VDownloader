mod core;
mod ui;

use gtk4::{glib, prelude::*, Application};
use log::info;

const APP_ID: &str = "com.vdownloader.VDownloader";

fn main() -> glib::ExitCode {
    env_logger::init();
    info!("Starting VDownloader application");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = ui::window::build_window(app);
    window.present();
}
