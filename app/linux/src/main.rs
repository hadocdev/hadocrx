use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::*;
use gtk::{
    glib, 
    CssProvider 
};

mod components;
mod styles;
mod models;
mod hadocrx;

const APP_ID: &str = "org.hadoc.rx";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

#[allow(deprecated)]
fn build_ui(app: &adw::Application) {
    let css_provider = CssProvider::new();
    css_provider.load_from_string(styles::CSS);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(), 
        &css_provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    ); 

    let window = adw::ApplicationWindow::builder()
        .application(app)
        .title("HadocRx")
        .build(); 

    let generics = hadocrx::db::get_generics();
    let entry_rc = components::search_bar::new(generics); 
    let vbox = components::vbox();
    vbox.append(&*entry_rc);
    window.set_content(Some(&vbox));


    window.present();
}
