use std::{cell::RefCell, rc::Rc};

use gtk::prelude::{ButtonExt, GridExt, WidgetExt};

use super::medicine_row::MedicineRow;

pub struct MedicineBox {
    pub container: gtk::Grid,
    pub rows: RefCell<Vec<MedicineRow>>,
}

impl MedicineBox {
    pub fn new() -> Rc<Self> {
        let container = gtk::Grid::builder()
            .margin_top(16).margin_bottom(16)
            .margin_start(16).margin_end(16)
            .column_spacing(16)
            .row_spacing(8)
            .halign(gtk::Align::Center)
            .build();
        Rc::new(Self {
            container,
            rows: RefCell::new(Vec::new()),
        })
    }

    pub fn append(self: &Rc<Self>, medicine_row: MedicineRow) {
        let index = self.rows.borrow().len();
        
        self.rows.borrow_mut().push(medicine_row);
        self.setup_row(index); 
        self.connect_btn_signals(index);
    }
    
    fn setup_header_row(&self) {
        self.container.attach(&Self::custom_label("Formulation"), 0, 0, 1, 1);
        self.container.attach(&Self::custom_label("Brand Name"), 1, 0, 1, 1);
        self.container.attach(&Self::custom_label("Generic Name"), 2, 0, 1, 1);
        self.container.attach(&Self::custom_label("Strength"), 3, 0, 1, 1);
        self.container.attach(&Self::custom_label("Dosing"), 4, 0, 1, 1);
        self.container.attach(&Self::custom_label("Instructions"), 5, 0, 1, 1);
        self.container.attach(&Self::custom_label("Duration"), 6, 0, 1, 1);
    }

    fn setup_row(self: &Rc<Self>, index: usize) {
        if index == 0 {
            self.setup_header_row();
        }
        let row = index as i32;
        let rows_borrowed = self.rows.borrow();
        let item = rows_borrowed.get(index).unwrap();
        self.container.attach(&item.label_formulation, 0, row+1, 1, 1);
        self.container.attach(&item.label_brand_name, 1, row+1, 1, 1);
        self.container.attach(&item.label_generic_name, 2, row+1, 1, 1);
        self.container.attach(&item.label_strength, 3, row+1, 1, 1);
        self.container.attach(&item.label_dosing, 4, row+1, 1, 1);
        self.container.attach(&item.label_instructions, 5, row+1, 1, 1);
        self.container.attach(&item.label_duration, 6, row+1, 1, 1);
      
        self.container.attach(&item.btn_up, 7, row+1, 1, 1);
        self.container.attach(&item.btn_down, 8, row+1, 1, 1);
        self.container.attach(&item.btn_delete, 9, row+1, 1, 1);
    }

    fn connect_btn_signals(self: &Rc<Self>, index: usize) {
        let rows_borrowed = self.rows.borrow();
        let item = rows_borrowed.get(index).unwrap();
        let id = item.data.id;
        
        let self_clone = self.clone(); 
        item.btn_delete.connect_clicked(move |btn| {
            btn.set_sensitive(false);
            {
                let mut mutable_borrow = self_clone.rows.borrow_mut();
                if let Some(index) = mutable_borrow.iter().position(|row| row.data.id == id) {
                    mutable_borrow.remove(index);
                }
            }
            self_clone.refresh_ui();
        });

        let self_clone = self.clone(); 
        item.btn_up.connect_clicked(move |_| {
            {
                let mut mutable_borrow = self_clone.rows.borrow_mut();
                if let Some(index) = mutable_borrow.iter().position(|row| row.data.id == id) {
                    if index > 0 {
                        mutable_borrow.swap(index, index-1);
                    }
                }
            }
            self_clone.refresh_ui();
        });
        
        let self_clone = self.clone(); 
        item.btn_down.connect_clicked(move |_| {
            {
                let mut mutable_borrow = self_clone.rows.borrow_mut();
                if let Some(index) = mutable_borrow.iter().position(|row| row.data.id == id) {
                    if index+1 < mutable_borrow.len() {
                        mutable_borrow.swap(index, index+1);
                    }
                }
            }
            self_clone.refresh_ui();
        });
    }

    fn refresh_ui(self: &Rc<Self>) {
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
            drop(child);
        }
        for index in 0..self.rows.borrow().len() {
            self.setup_row(index); 
        }
    }

    fn custom_label(text: &str) -> gtk::Label {
        gtk::Label::builder().label(text).halign(gtk::Align::Start).css_classes(["underline"]).build()
    }
}
