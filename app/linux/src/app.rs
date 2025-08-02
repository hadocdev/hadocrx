use std::rc::Rc;

use adw::prelude::AdwApplicationWindowExt;
use gtk::{prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, GridExt, GtkWindowExt, WidgetExt}, CssProvider};

use super::{ widgets, styles, hadocrx };

pub struct AppState {
    pub widgets: AppWidgets,
    pub window: adw::ApplicationWindow
}

impl AppState {
    pub fn new(app: &adw::Application) -> Rc<Self> {
        let widgets = AppWidgets::new();
        let window = Self::create_window(app);
        Rc::new(Self { widgets, window })
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
        let header = adw::HeaderBar::new();
        let root = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(0).margin_end(0).margin_top(0).margin_bottom(0)
            .build();
        let scrolled_window = gtk::ScrolledWindow::builder().child(&self.widgets.container).build();
        root.append(&header);
        root.append(&scrolled_window);
        self.window.set_content(Some(&root));
        
        let label = widgets::label("Brand Name");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 0, 0, 1, 1);
        self.widgets.grid.attach(&self.widgets.brand_name_search_box.entry, 1, 0, 1, 1);
        
        let label = widgets::label("Generic Name");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 0, 1, 1, 1);
        self.widgets.grid.attach(&self.widgets.generic_name_search_box.entry, 1, 1, 1, 1);

        let label = widgets::label("Strength");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 0, 2, 1, 1);
        self.widgets.grid.attach(&self.widgets.strength_dropdown_box.entry, 1, 2, 1, 1);

        let label = widgets::label("Formulation");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 0, 3, 1, 1);
        self.widgets.grid.attach(&self.widgets.formulation_dropdown_box.entry, 1, 3, 1, 1);
 
        let label = widgets::label("Manufacturer");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 2, 0, 1, 1);
        self.widgets.grid.attach(&self.widgets.manufacturer_dropdown_box.entry, 3, 0, 1, 1); 

        let label = widgets::label("Dosing");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 2, 1, 1, 1);
        self.widgets.grid.attach(&self.widgets.dosing_box.entry, 3, 1, 1, 1);

        let label = widgets::label("Instructions");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 2, 2, 1, 1);
        self.widgets.grid.attach(&self.widgets.instructions_box.entry, 3, 2, 1, 1);

        let label = widgets::label("Duration");
        label.set_halign(gtk::Align::End);
        self.widgets.grid.attach(&label, 2, 3, 1, 1);
        self.widgets.grid.attach(&self.widgets.duration_box.entry, 3, 3, 1, 1);
        self.widgets.grid.attach(&self.widgets.btn_add, 4, 1, 1, 1);
        
        self.widgets.container.append(&self.widgets.grid);
        self.widgets.container.append(&self.widgets.medicine_box.container);    
    }

    fn prepare_widgets(self: &Rc<Self>) {
        let generic_names = hadocrx::db::get_generic_names();
        let brand_names = hadocrx::db::get_brand_names();
        self.widgets.generic_name_search_box.initialize(generic_names);
        self.widgets.brand_name_search_box.initialize(brand_names); 
        self.widgets.strength_dropdown_box.initialize(Vec::new()); 
        self.widgets.formulation_dropdown_box.initialize(Vec::new()); 
        
        let self_clone = self.clone();
        self.widgets.brand_name_search_box.entry.connect_activate(move |entry| {
            // on enter -> 
            if !entry.text().is_empty() {
                // if brand_name != empty, get generic_name from the db
                let brand_name = entry.text().to_string();
                let generic_name = hadocrx::db::get_generic_name_by_brand_name(brand_name.clone());
                // update the generic_name box
                self_clone.widgets.generic_name_search_box.update_entry_text(generic_name.clone());
                
                // get available strengths for this generic_name
                let strengths = hadocrx::db::get_strengths_by_generic_name(generic_name);
                let count_strengths = strengths.len();
                self_clone.widgets.strength_dropdown_box.update(strengths); 
                // if only one strength is available, set it on the strength box
                if count_strengths == 1 {
                    self_clone.widgets.strength_dropdown_box.entry.emit_activate();
                }
            }
        });

        let self_clone = self.clone();
        self.widgets.strength_dropdown_box.entry.connect_activate(move |entry| {
            let strength = entry.text().to_string();
            // get the correct brand_name for the generic_name, strength and manufacturer 
            let brand_name = hadocrx::db::get_brand_name_by_generic_name_and_strength(
                self_clone.widgets.brand_name_search_box.entry.text().to_string(),
                self_clone.widgets.generic_name_search_box.entry.text().to_string(),
                strength.clone()
            );
            
            // set the brand_name in the brand_name box
            self_clone.widgets.brand_name_search_box.update_entry_text(brand_name.clone());
            
            // get available formulations for the brand_name and strength
            let formulations = hadocrx::db::get_formulations_by_brand_name_and_strength(brand_name, strength);
            let count_formulations = formulations.len();
            self_clone.widgets.formulation_dropdown_box.update(formulations);
            // if only one formulation is available, set it on the formulation box
            if count_formulations == 1 {
                self_clone.widgets.formulation_dropdown_box.entry.emit_activate();
            }
        });

        let self_clone = self.clone();
        self.widgets.btn_add.connect_clicked(move |_| {
            let brand_name = self_clone
                .widgets.brand_name_search_box.entry.text()
                .split_whitespace().collect::<Vec<&str>>().join(" ");
            let generic_name = self_clone.widgets.generic_name_search_box.entry.text();
            let strength = self_clone.widgets.strength_dropdown_box.entry.text();
            let formulation = self_clone.widgets.formulation_dropdown_box.entry.text();
            if !brand_name.is_empty() && !generic_name.is_empty() && !formulation.is_empty() {
                let medicine_row = widgets::medicine_row::MedicineRow::new(
                    &formulation, &brand_name, 
                    &generic_name, &strength
                );
                self_clone.widgets.medicine_box.append(medicine_row);
                self_clone.widgets.brand_name_search_box.entry.set_text("");
                self_clone.widgets.generic_name_search_box.entry.set_text("");
                self_clone.widgets.strength_dropdown_box.entry.set_text("");
                self_clone.widgets.formulation_dropdown_box.entry.set_text("");
                self_clone.widgets.brand_name_search_box.entry.grab_focus();
            }
        }); 
    }

    fn create_window(app: &adw::Application) -> adw::ApplicationWindow {
        adw::ApplicationWindow::builder()
            .application(app)
            .title("HadocRx")
            .build()
    }
}

pub struct AppWidgets {
    pub container: gtk::Box,
    pub grid: gtk::Grid,
    pub brand_name_search_box: Rc<widgets::search_box::SearchBox>,
    pub generic_name_search_box: Rc<widgets::search_box::SearchBox>,
    pub strength_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub formulation_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub manufacturer_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub dosing_box: Rc<widgets::bangla_entry::BanglaEntry>,
    pub instructions_box: Rc<widgets::bangla_entry::BanglaEntry>,
    pub duration_box: Rc<widgets::bangla_entry::BanglaEntry>,
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
        let grid = gtk::Grid::builder()
            .margin_top(16).margin_bottom(16)
            .margin_start(16).margin_end(16)
            .column_spacing(16)
            .row_spacing(8)
            .halign(gtk::Align::Center)
            .build();
        grid.set_size_request(800, -1);
        
        let brand_name_search_box = widgets::search_box::SearchBox::new();
        brand_name_search_box.entry.set_placeholder_text(Some("Brand Name"));
        brand_name_search_box.entry.set_size_request(250, -1);
        
        let generic_name_search_box = widgets::search_box::SearchBox::new();
        generic_name_search_box.entry.set_placeholder_text(Some("Generic Name"));
        
        let strength_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        strength_dropdown_box.entry.set_placeholder_text(Some("Strength"));
        
        let formulation_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        formulation_dropdown_box.entry.set_placeholder_text(Some("Dosage Formulation"));

        let manufacturer_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        manufacturer_dropdown_box.entry.set_placeholder_text(Some("Manufacturer"));
        manufacturer_dropdown_box.entry.set_size_request(250, -1);

        let dosing_box = widgets::bangla_entry::BanglaEntry::new();
        let instructions_box = widgets::bangla_entry::BanglaEntry::new();
        let duration_box = widgets::bangla_entry::BanglaEntry::new();
        
        let btn_add = widgets::combo_button!(
            gtk::Orientation::Horizontal,
            widgets::label("Add Drug"),
            gtk::Image::from_icon_name("list-add")
        );
        
        let medicine_box = widgets::medicine_box::MedicineBox::new();
        
        Self { 
            container, grid, 
            brand_name_search_box, generic_name_search_box, 
            manufacturer_dropdown_box,
            strength_dropdown_box, formulation_dropdown_box,
            dosing_box, instructions_box, duration_box,
            btn_add, medicine_box
        }
    }
}


