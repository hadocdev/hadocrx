// use gtk::prelude::WidgetExt;

use super::custom_icon_button;

pub struct MedicineRow {
    pub label_formulation: gtk::Label,
    pub label_brand_name: gtk::Label,
    pub label_generic_name: gtk::Label,
    pub label_strength: gtk::Label,
    pub label_dosing: gtk::Label,
    pub label_duration: gtk::Label,
    pub label_instructions: gtk::Label,
    
    pub btn_up: gtk::Button,
    pub btn_down: gtk::Button,
    pub btn_delete: gtk::Button,
}

impl MedicineRow {
    pub fn new(formulation: &str, brand_name: &str, generic_name: &str, strength: &str, dosing: &str, duration: &str, instructions: &str) -> Self {
        let label_formulation = gtk::Label::builder().label(formulation).halign(gtk::Align::Start).build();
        let label_brand_name = gtk::Label::builder().label(brand_name).css_classes(["title-2"]).halign(gtk::Align::Start).build();
        let label_generic_name = gtk::Label::builder().label(generic_name).css_classes(["caption"]).halign(gtk::Align::Start).build();
        let label_strength = gtk::Label::builder().label(strength).halign(gtk::Align::Start).build();
        let label_dosing = gtk::Label::builder().label(dosing).selectable(true).halign(gtk::Align::Start).build();
        let label_duration = gtk::Label::builder().label(duration).selectable(true).halign(gtk::Align::Start).build();
        let label_instructions = gtk::Label::builder().label(instructions).selectable(true).halign(gtk::Align::Start).build();
       
        let btn_up = custom_icon_button("fa-arrow-up"); 
        let btn_down = custom_icon_button("fa-arrow-down");
        let btn_delete = custom_icon_button("fa-arrow-delete");
        // btn_up.add_css_class("flat");
        // btn_down.add_css_class("flat");
        // btn_delete.add_css_class("flat");

        Self { 
            label_formulation, 
            label_brand_name, 
            label_generic_name, 
            label_strength,
            label_dosing,
            label_duration,
            label_instructions,
            btn_up, 
            btn_down,
            btn_delete 
        }
    }
}
