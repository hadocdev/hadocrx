use std::env;

use gtk::prelude::ButtonExt;

pub struct MedicineRow {
    pub label_formulation: gtk::Label,
    pub label_brand_name: gtk::Label,
    pub label_generic_name: gtk::Label,
    pub label_strength: gtk::Label,
    pub btn_up: gtk::Button,
    pub btn_down: gtk::Button,
    pub btn_delete: gtk::Button,
}

impl MedicineRow {
    pub fn new(formulation: String, brand_name: String, generic_name: String, strength: String) -> Self {
        let label_brand_name = gtk::Label::builder().label(brand_name).css_classes(["heading"]).build();
        let label_generic_name = gtk::Label::builder().label(generic_name).css_classes(["caption"]).build();
       
        let settings = gtk::Settings::for_display(&gtk::gdk::Display::default().unwrap());
        let should_be_dark = settings.is_gtk_application_prefer_dark_theme() || env::var("GTK_THEME").unwrap().contains(":dark"); 

        let btn_up = gtk::Button::new();
        let btn_down = gtk::Button::new();
        let btn_delete = gtk::Button::new();
        if should_be_dark {
            btn_up.set_icon_name("fa-arrow-up-dark");
            btn_down.set_icon_name("fa-arrow-down-dark");
            btn_delete.set_icon_name("fa-arrow-delete-dark");
        } else {
            btn_up.set_icon_name("fa-arrow-up");
            btn_down.set_icon_name("fa-arrow-down");
            btn_delete.set_icon_name("fa-arrow-delete");
        }

        Self { 
            label_formulation: gtk::Label::builder().label(formulation).build(), 
            label_brand_name, 
            label_generic_name, 
            label_strength: gtk::Label::builder().label(strength).build(), 
            btn_up, 
            btn_down,
            btn_delete 
        }
    }
}
