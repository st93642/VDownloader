/*****************************************************************************/
/*                                                                           */
/*  main.rs                                              TTTTTTTT SSSSSSS II */
/*                                                          TT    SS      II */
/*  By: st93642@students.tsi.lv                             TT    SSSSSSS II */
/*                                                          TT         SS II */
/*  Created: Dec 08 2025 21:19 st93642                      TT    SSSSSSS II */
/*  Updated: Dec 08 2025 21:19 st93642                                       */
/*                                                                           */
/*   Transport and Telecommunication Institute - Riga, Latvia                */
/*                       https://tsi.lv                                      */
/*****************************************************************************/


mod core;
mod ui;

use gtk4::{glib, prelude::*, Application};
use log::info;

const APP_ID: &str = "com.vdownloader.VDownloader";

fn main() -> glib::ExitCode {
    env_logger::init();
    info!("Starting VDownloader application");

    // Initialize Tokio runtime
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = runtime.enter();

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    // Enable dark theme
    if let Some(settings) = gtk4::Settings::default() {
        settings.set_gtk_application_prefer_dark_theme(true);
    }

    let window = ui::window::build_window(app);
    window.present();
}
