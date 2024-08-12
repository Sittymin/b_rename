mod window;

use gdk::Display;
use adw::prelude::*;
use adw::{gio, glib, Application };
use window::Window;

const APP_ID: &str = "io.github.sittymin";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("b_rename.gresource").expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("resources/style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
}
