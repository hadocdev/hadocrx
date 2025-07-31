use adw::prelude::AdwApplicationWindowExt;
use gtk::prelude::*;
use gtk::{
    glib, 
    CssProvider 
};

mod widgets;
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

    let generic_names = hadocrx::db::get_generic_names();
    let brand_names = hadocrx::db::get_brand_names();
    let vbox = widgets::vbox();
    vbox.set_spacing(8);
    
    let generic_name_search_box = widgets::search_box::SearchBox::new();
    generic_name_search_box.entry.set_placeholder_text(Some("Generic Name"));
    generic_name_search_box.initialize(generic_names);
    
    let brand_name_search_box = widgets::search_box::SearchBox::new();
    brand_name_search_box.entry.set_placeholder_text(Some("Brand Name"));
    brand_name_search_box.initialize(brand_names);

    let generic_name_search_box_clone = generic_name_search_box.clone();
    brand_name_search_box.entry.connect_activate(move |object| {
        let generic_name = hadocrx::db::get_generic_name_by_brand_name(object.text().to_string());
        generic_name_search_box_clone.entry.set_text(&generic_name);
        *generic_name_search_box_clone.expected_programmatic_change.borrow_mut() = Some(generic_name.clone()); 
    });
    
    vbox.append(&generic_name_search_box.entry);
    vbox.append(&brand_name_search_box.entry);
 
    window.set_content(Some(&vbox));
    window.present();
}
