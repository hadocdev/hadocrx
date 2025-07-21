use gtk::prelude::*;
use gtk::{
    glib, 
    Application, ApplicationWindow, 
    CssProvider, EntryIconPosition
};

mod components;
mod styles;
mod models;

const APP_ID: &str = "org.hadoc.rx";
const DATA: &[&str] = &[
        "Paracetamol",
        "Ibuprofen",
        "Amoxicillin",
        "Metformin",
        "Omeprazole",
        "Simvastatin",
        "Lisinopril",
        "Amlodipine",
        "Salbutamol",
        "Cetirizine",
    ];

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

#[allow(deprecated)]
fn build_ui(app: &Application) {
    let css_provider = CssProvider::new();
    css_provider.load_from_string(styles::CSS);

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(), 
        &css_provider, 
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
    ); 

    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(400)
        .default_height(300)
        .title("HadocRx")
        .build(); 

    window.maximize();

    let entry_rc = components::search_bar::new(DATA, move |object, icon_position| {
        match icon_position {
            EntryIconPosition::Primary => {
                search(object.text().as_str());
            },
            EntryIconPosition::Secondary => {
                object.set_text("");
            }
            _ => {}
        }
    }); 
    let vbox = components::vbox();
    vbox.append(&*entry_rc);
    window.set_child(Some(&vbox)); 

    window.present();
}

#[allow(dead_code)]
fn search(query: &str) {
    eprintln!("Search not implemented yet!");
    eprintln!("query: {}", query);
}
