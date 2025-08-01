use std::{cell::RefCell, rc::Rc};

use adw::prelude::ExpanderRowExt;
use gtk::prelude::{BoxExt, WidgetExt};

use super::medicine_row::MedicineRow;

pub struct MedicineBox {
    pub rows: RefCell<Vec<MedicineRow>>,
    pub container: gtk::Box
}

impl MedicineBox {
    pub fn new() -> Rc<Self> {
        let container = super::vbox();
        super::set_margins(container.as_ref(), 16);
        container.set_spacing(8);
        Rc::new(Self {
            rows: RefCell::new(Vec::new()),
            container
        })
    }

    pub fn append(&self, row: MedicineRow) {
        self.rows.borrow_mut().push(row);
        self.setup_ui();
    }

    fn setup_ui(&self) {
        while let Some(child) = self.container.first_child() { 
            self.container.remove(&child);
            drop(child);
        }
        
        for row in self.rows.borrow().iter() {
            let suffix = super::hbox();
            let prefix = super::vbox();
            let body = super::hbox();
            body.set_spacing(8);

            row.label_brand_name.add_css_class("heading");
            row.label_brand_name.set_halign(gtk::Align::Start);
            prefix.append(&row.label_brand_name);

            row.label_generic_name.add_css_class("caption");
            row.label_generic_name.set_halign(gtk::Align::Start);
            prefix.append(&row.label_generic_name);

            suffix.set_spacing(8);
            suffix.set_margin_end(16);
            suffix.append(&super::hspacer());
            suffix.append(&row.btn_up);
            suffix.append(&row.btn_down);
            suffix.append(&row.btn_delete);

            body.append(&row.label_formulation);
            body.append(&row.label_strength);
            
            let expander_row = adw::ExpanderRow::new();
            expander_row.add_prefix(&prefix);
            expander_row.add_suffix(&suffix);
            expander_row.add_row(&body);
            // expander_row.set_title(row.label_brand_name.label().as_str());
            // expander_row.set_subtitle(row.label_generic_name.label().as_str());
            self.container.append(&expander_row);
        }
    }
}
