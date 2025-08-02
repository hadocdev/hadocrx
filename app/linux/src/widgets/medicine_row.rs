use gtk::prelude::WidgetExt;

use super::custom_icon_button;

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
    pub fn new(formulation: &str, brand_name: &str, generic_name: &str, strength: &str) -> Self {
        let label_formulation = Self::label_with_class(formulation, None);
        let label_brand_name = Self::label_with_class(brand_name, Some("title-2"));
        let label_generic_name = Self::label_with_class(generic_name, Some("caption"));
        let label_strength = Self::label_with_class(strength, None);
       
        let btn_up = custom_icon_button("fa-arrow-up"); 
        btn_up.add_css_class("flat");
        let btn_down = custom_icon_button("fa-arrow-down");
        btn_down.add_css_class("flat");
        let btn_delete = custom_icon_button("fa-arrow-delete");
        btn_delete.add_css_class("flat");

        Self { 
            label_formulation, 
            label_brand_name, 
            label_generic_name, 
            label_strength, 
            btn_up, 
            btn_down,
            btn_delete 
        }
    }

    fn label_with_class(text: &str, css_class: Option<&str>) -> gtk::Label {
        let label = gtk::Label::builder().label(text).valign(gtk::Align::Baseline).build();
        if let Some(class) = css_class {
            label.add_css_class(class);
        }
        label
    }
}
