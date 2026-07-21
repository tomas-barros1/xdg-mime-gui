use adw::prelude::*;

use crate::window::build_window;

pub fn run() {
    let app = adw::Application::builder()
        .application_id("io.github.tomas_barros1.DefaultApps")
        .build();

    app.connect_activate(|app| {
        let window = build_window(app);
        window.present();
    });

    app.run();
}
