use std::{cell::RefCell, rc::Rc};

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
            let row_hbox = super::hbox();
            row_hbox.add_css_class("medicine_box_row");
            row_hbox.set_spacing(8);
            
            row_hbox.append(&row.label_formulation);
            row_hbox.append(&row.label_brand_name);
            row_hbox.append(&row.label_generic_name);
            row_hbox.append(&row.label_strength);
            
            row_hbox.append(&super::hspacer());
            row_hbox.append(&row.btn_up);
            row_hbox.append(&row.btn_down);
            row_hbox.append(&row.btn_delete);

            
            // expander_row.set_title(row.label_brand_name.label().as_str());
            // expander_row.set_subtitle(row.label_generic_name.label().as_str());
            self.container.append(&row_hbox);
        }
    }
}
