use std::rc::Rc;

use gtk::{prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, GridExt, GtkWindowExt,  WidgetExt}, CssProvider};

use super::{ widgets, styles};

pub struct AppState {
    pub widgets: AppWidgets,
    pub window: gtk::ApplicationWindow,
    pub dialog: gtk::AlertDialog,
}

impl AppState {
    pub fn new(app: &gtk::Application) -> Rc<Self> {
        let widgets = AppWidgets::new();
        let window = Self::create_window(app);
        let dialog = gtk::AlertDialog::builder().build();     
        Rc::new(Self { widgets, window, dialog })
    }

    pub fn setup_ui(self: &Rc<Self>) {
        self.setup_styles();
        self.prepare_widgets();
        self.setup_layout();  
        self.window.present();
    }

    fn setup_styles(&self) {
        let css_provider = CssProvider::new();
        css_provider.load_from_string(styles::CSS);

        gtk::style_context_add_provider_for_display(
            &gtk::gdk::Display::default().unwrap(), 
            &css_provider, 
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION
        );
    }

    fn setup_layout(&self) { 
        let root = hadocrx_macros::gtk_box! (
            @orientation gtk::Orientation::Vertical,
            @margin_start 0, @margin_end 0, @margin_top 0, @margin_bottom 0,
            gtk::HeaderBar::new(),
            gtk::ScrolledWindow::builder().child(&self.widgets.container).build()
        );
        self.window.set_child(Some(&root));
        
        let grid = hadocrx_macros::gtk_grid!(
            @margin_top 16, @margin_bottom 16, @margin_start 16, @margin_end 16,
            @column_spacing 16, @row_spacing 8,
            @halign gtk::Align::Center,
            @width_request 800, [ 
                &Self::right_aligned_label("Brand Name"), 
                &self.widgets.brand_name_search_box,
                &Self::right_aligned_label("Manufacturer"),
                &self.widgets.manufacturer_dropdown_box.entry,
            ], [ 
                &Self::right_aligned_label("Generic Name"), 
                &self.widgets.generic_name_search_box,
                &Self::right_aligned_label("Dosing"),
                &self.widgets.dosing_box,
                &self.widgets.btn_add
            ], [
                &Self::right_aligned_label("Strength"), 
                &self.widgets.strength_dropdown_box.entry,
                &Self::right_aligned_label("Instructions"),
                &self.widgets.instructions_box
            ], [
                &Self::right_aligned_label("Formulation"), 
                &self.widgets.formulation_dropdown_box.entry,
                &Self::right_aligned_label("Duration"),
                &self.widgets.duration_box
            ]
        );
        
        self.widgets.container.append(&grid);
        self.widgets.container.append(&self.widgets.medicine_box.container);    
    }

    fn right_aligned_label(label: &str) -> gtk::Label {
        gtk::Label::builder().label(label).halign(gtk::Align::End).build()
    }

    fn prepare_widgets(self: &Rc<Self>) {
        let generic_names = hadocrx::ffi::db::get_generic_names();
        let brand_names = hadocrx::ffi::db::get_brand_names();
        self.widgets.generic_name_search_box.initialize(generic_names);
        self.widgets.brand_name_search_box.initialize(brand_names);
        self.widgets.strength_dropdown_box.initialize(Vec::new()); 
        self.widgets.formulation_dropdown_box.initialize(Vec::new());
        self.widgets.manufacturer_dropdown_box.initialize(Vec::new());
        self.widgets.strength_dropdown_box.entry.set_secondary_icon_sensitive(false);
        self.widgets.formulation_dropdown_box.entry.set_secondary_icon_sensitive(false);
        self.widgets.manufacturer_dropdown_box.entry.set_secondary_icon_sensitive(false);
        
        let self_clone = self.clone();
        // self.widgets.brand_name_search_box.entry.connect_activate(move |entry| {
        self.widgets.brand_name_search_box.entry().connect_activate(move |entry| {
            if entry.text().is_empty() { return; }
            let brand_name = entry.text().to_string();
            
            // get generic_name from the db
            // update the generic_name box
            let generic_name = hadocrx::ffi::db::get_generic_name_by_brand_name(brand_name.clone());
            self_clone.widgets.generic_name_search_box.update_entry_text(&generic_name);
            self_clone.widgets.generic_name_search_box.entry().emit_activate();

            // get the manufacturer for this brand_name
            // get all available manufacturers for this generic_name
            // update the manufacturer box
            let manufacturer = hadocrx::ffi::db::get_manufacturer_by_brand_name(brand_name); 
            
            self_clone.widgets.manufacturer_dropdown_box.update_entry_text(manufacturer.clone()); 
            // enable the manufacturer dropdown
            if !self_clone.widgets.manufacturer_dropdown_box.entry.is_secondary_icon_sensitive() {
                self_clone.widgets.manufacturer_dropdown_box.entry.set_secondary_icon_sensitive(true);
            }
            
            // get available strengths for this generic_name
            let strengths = hadocrx::ffi::db::get_strengths_by_generic_name(generic_name);
            let count_strengths = strengths.len();
            self_clone.widgets.strength_dropdown_box.update(strengths);
            // enable the strength dropdown
            self_clone.widgets.strength_dropdown_box.entry.set_secondary_icon_sensitive(true);
            // if only one strength is available, set it on the strength box
            if count_strengths == 1 {
                self_clone.widgets.strength_dropdown_box.entry.emit_activate();
            } else {
                self_clone.widgets.formulation_dropdown_box.update(Vec::new());
            }
        });

        let self_clone = self.clone();
        // self.widgets.generic_name_search_box.entry.connect_activate(move |entry| {
        self.widgets.generic_name_search_box.entry().connect_activate(move |entry| {
            if entry.text().is_empty() { return; }
            let generic_name = entry.text().to_string();
            let manufacturers = hadocrx::ffi::db::get_manufacturers_by_generic_name(generic_name.clone());
            self_clone.widgets.manufacturer_dropdown_box.update(manufacturers);
        });

        let self_clone = self.clone();
        self.widgets.strength_dropdown_box.entry.connect_activate(move |entry| {
            if entry.text().is_empty() { return; }
            let strength = entry.text().to_string();
            let generic_name = self_clone.widgets.generic_name_search_box.entry().text().to_string();
            let manufacturer = self_clone.widgets.manufacturer_dropdown_box.entry.text().to_string();
            // get the correct brand_name for the generic_name, strength and manufacturer 
            let brand_name = hadocrx::ffi::db::get_brand_name_by_generic_name_manufacturer_and_strength(
                generic_name.clone(), manufacturer.clone(), strength.clone()
            );
            
            if let Some(name) = brand_name {
                self_clone.widgets.brand_name_search_box.update_entry_text(&name);
                // get available formulations for the brand_name and strength
                let formulations = hadocrx::ffi::db::get_formulations_by_brand_name_and_strength(name, strength);
                let count_formulations = formulations.len();
                self_clone.widgets.formulation_dropdown_box.update(formulations);
                // enable the formulation box
                self_clone.widgets.formulation_dropdown_box.entry.set_secondary_icon_sensitive(true);
                // if only one formulation is available, set it on the formulation box
                if count_formulations == 1 {
                    self_clone.widgets.formulation_dropdown_box.entry.emit_activate();
                }
            } else {
                self_clone.widgets.strength_dropdown_box.update_entry_text(String::new());
                self_clone.widgets.formulation_dropdown_box.update_entry_text(String::new());
                self_clone.dialog.set_message("Unavailable!");
                self_clone.dialog.set_detail(&format!("{} - {} is not available from {}", generic_name, strength, manufacturer));
                self_clone.dialog.show(Some(&self_clone.window));
            }
        });

        let self_clone = self.clone();
        self.widgets.manufacturer_dropdown_box.entry.connect_activate(move |entry| {
            if entry.text().is_empty() { return; }
            let manufacturer = entry.text().to_string();
            let generic_name = self_clone.widgets.generic_name_search_box.entry().text().to_string();
            // get brand_name for this manufacturer and generic_name
            let brand_name = hadocrx::ffi::db::get_brand_name_by_generic_name_and_manufacturer(generic_name, manufacturer);
            self_clone.widgets.brand_name_search_box.update_entry_text(&brand_name);
        });

        let self_clone = self.clone();
        self.widgets.btn_add.connect_clicked(move |_| {
            let brand_name = self_clone
                .widgets.brand_name_search_box.entry().text()
                .split_whitespace().collect::<Vec<&str>>().join(" ");
            let generic_name = self_clone.widgets.generic_name_search_box.entry().text().to_string();
            let strength = self_clone.widgets.strength_dropdown_box.entry.text().to_string();
            let formulation = self_clone.widgets.formulation_dropdown_box.entry.text().to_string();
            let manufacturer = self_clone.widgets.manufacturer_dropdown_box.entry.text().to_string();
            // let dosing = self_clone.widgets.dosing_box.entry.text().to_string();
            // let duration = self_clone.widgets.duration_box.entry.text().to_string();
            // let instructions = self_clone.widgets.instructions_box.entry.text().to_string();
            let dosing = self_clone.widgets.dosing_box.text().to_string();
            let duration = self_clone.widgets.duration_box.text().to_string();
            let instructions = self_clone.widgets.instructions_box.text().to_string();


            let errors = widgets::utils::validation_errors!(brand_name, strength, formulation, dosing);
            if let Some(message) = errors {
                self_clone.dialog.set_message("Required fields are empty!");
                self_clone.dialog.set_detail(&message);
                self_clone.dialog.show(Some(&self_clone.window));
            } else {
                let medicine_data = hadocrx::ffi::prescription::MedicineData::new(brand_name, generic_name, strength, formulation, manufacturer, dosing, instructions, duration);
                let medicine_row = widgets::medicine_row::MedicineRow::new(medicine_data);
                self_clone.widgets.medicine_box.append(medicine_row);
                
                // self_clone.widgets.brand_name_search_box.entry.set_text("");
                // self_clone.widgets.generic_name_search_box.entry.set_text("");
                self_clone.widgets.strength_dropdown_box.update(Vec::new());
                self_clone.widgets.formulation_dropdown_box.update(Vec::new());
                self_clone.widgets.manufacturer_dropdown_box.entry.set_text("");

                self_clone.widgets.dosing_box.clear();
                self_clone.widgets.duration_box.clear();
                self_clone.widgets.instructions_box.clear();
                
                // self_clone.widgets.brand_name_search_box.entry.grab_focus();
                self_clone.widgets.brand_name_search_box.grab_focus();
            }
        }); 
    }

    fn create_window(app: &gtk::Application) -> gtk::ApplicationWindow {
        gtk::ApplicationWindow::builder()
            .application(app)
            .title("HadocRx")
            .build()
    }
}

pub struct AppWidgets {
    pub container: gtk::Box,
    pub brand_name_search_box: widgets::search_box::SearchBox,
    pub generic_name_search_box: widgets::search_box::SearchBox,
    pub strength_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub formulation_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub manufacturer_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub dosing_box: widgets::avro_phonetic_entry::AvroPhoneticEntry,
    pub instructions_box: widgets::avro_phonetic_entry::AvroPhoneticEntry,
    pub duration_box: widgets::avro_phonetic_entry::AvroPhoneticEntry,
    pub btn_add: gtk::Button,
    pub medicine_box: Rc<widgets::medicine_box::MedicineBox>
}

impl AppWidgets {
    pub fn new() -> Self {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(0).margin_end(0).margin_top(0).margin_bottom(0)
            .hexpand(true)
            .vexpand(true)
            .build();
        container.set_size_request(800, 600); 
        
        let brand_name_search_box = widgets::search_box::SearchBox::new();
        brand_name_search_box.entry().set_placeholder_text(Some("Brand Name"));
        brand_name_search_box.entry().set_size_request(250, -1);
        
        let generic_name_search_box = widgets::search_box::SearchBox::new();
        generic_name_search_box.entry().set_placeholder_text(Some("Generic Name"));
        
        let strength_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        strength_dropdown_box.entry.set_placeholder_text(Some("Strength"));
        
        let formulation_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        formulation_dropdown_box.entry.set_placeholder_text(Some("Dosage Formulation"));

        let manufacturer_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        manufacturer_dropdown_box.entry.set_placeholder_text(Some("Manufacturer"));
        manufacturer_dropdown_box.entry.set_size_request(250, -1);

        let dosing_box = widgets::avro_phonetic_entry::AvroPhoneticEntry::new();
        let instructions_box = widgets::avro_phonetic_entry::AvroPhoneticEntry::new();
        let duration_box = widgets::avro_phonetic_entry::AvroPhoneticEntry::new();
        
        let btn_add = widgets::combo_button!(
            gtk::Orientation::Horizontal,
            widgets::label("Add Drug"),
            gtk::Image::from_icon_name("list-add")
        );
        
        let medicine_box = widgets::medicine_box::MedicineBox::new();
        
        Self { 
            container,
            brand_name_search_box, generic_name_search_box, 
            manufacturer_dropdown_box,
            strength_dropdown_box, formulation_dropdown_box,
            dosing_box, instructions_box, duration_box,
            btn_add, medicine_box
        }
    }
}


