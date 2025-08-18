use gtk::prelude::*;
use gtk::glib;
use gtk::gio;

mod widgets;
mod styles;
mod models;
mod app;

const APP_ID: &str = "org.hadoc.rx";

fn main() -> glib::ExitCode {
    gio::resources_register_include!("compiled.gresource").expect("Failed to register resource");
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

#[allow(deprecated)]
fn build_ui(app: &gtk::Application) {
    let app_state = app::AppState::new(app);
    app_state.setup_ui(); 
}
