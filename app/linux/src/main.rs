use gtk::prelude::*;
use gtk::glib;
use gtk::gio;

mod widgets;
mod styles;
mod models;
mod hadocrx;
mod app;

const APP_ID: &str = "org.hadoc.rx";

fn main() -> glib::ExitCode {
    let resource_data = include_bytes!("../compiled.gresource");
    let resource = gio::Resource::from_data(&glib::Bytes::from(resource_data)).unwrap();
    gio::resources_register(&resource);
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

#[allow(deprecated)]
fn build_ui(app: &adw::Application) {
    let app_state = app::AppState::new(app);
    app_state.setup_ui();
}
