use std::rc::Rc;

use adw::prelude::AdwApplicationWindowExt;
use gtk::{glib::object::Cast, prelude::{BoxExt, ButtonExt, EditableExt, EntryExt, GridExt, GtkWindowExt, PopoverExt, WidgetExt}, CssProvider, ListView};

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
        self.window.set_content(Some(&self.widgets.container));
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
        self.widgets.container.append(&header);
        
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
 
        self.widgets.grid.attach(&self.widgets.add_button, 2, 1, 1, 2);

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
            if !entry.text().is_empty() {
                let brand_name = entry.text().to_string();
                let generic_name = hadocrx::db::get_generic_name_by_brand_name(brand_name.clone());
                self_clone.widgets.generic_name_search_box.entry.set_text(&generic_name);
                *self_clone.widgets.generic_name_search_box.expected_programmatic_change.borrow_mut() = Some(generic_name.clone()); 
                
                let strengths = hadocrx::db::get_strengths_by_generic_name(generic_name.clone());
                self_clone.widgets.strength_dropdown_box.update(strengths); 
            }
        });

        let self_clone = self.clone();
        self.widgets.strength_dropdown_box.entry.connect_activate(move |_| {
            let scrollable_area = self_clone.widgets.strength_dropdown_box.popover.child().unwrap();
            let first_child = scrollable_area.first_child().unwrap();
            let list_view = first_child.downcast_ref::<ListView>().unwrap();
            let strength = widgets::utils::get_selected_item_text_from_list_view(list_view);
            
            let brand_name = hadocrx::db::get_brand_name_by_generic_name_and_strength(
                self_clone.widgets.brand_name_search_box.entry.text().to_string(),
                self_clone.widgets.generic_name_search_box.entry.text().to_string(),
                strength.clone()
            );
            self_clone.widgets.brand_name_search_box.entry.set_text(&brand_name);
            *self_clone.widgets.brand_name_search_box.expected_programmatic_change.borrow_mut() = Some(brand_name.clone());
            let formulations = hadocrx::db::get_formulations_by_brand_name_and_strength(brand_name, strength);
            self_clone.widgets.formulation_dropdown_box.update(formulations);
        });

        let self_clone = self.clone();
        self.widgets.add_button.connect_clicked(move |_| {
            let brand_name = self_clone
                .widgets.brand_name_search_box.entry.text()
                .split_whitespace().collect::<Vec<&str>>().join(" ");
            let generic_name = self_clone.widgets.generic_name_search_box.entry.text();
            let strength = self_clone.widgets.strength_dropdown_box.entry.text();
            let formulation = self_clone.widgets.formulation_dropdown_box.entry.text();
            if !brand_name.is_empty() && !generic_name.is_empty() && !formulation.is_empty() {
                let medicine_row = widgets::medicine_row::MedicineRow::new(
                    formulation.to_string(), brand_name.to_string(), 
                    generic_name.to_string(), strength.to_string()
                );
                self_clone.widgets.medicine_box.append(medicine_row);
                self_clone.widgets.brand_name_search_box.entry.set_text("");
                self_clone.widgets.generic_name_search_box.entry.set_text("");
                self_clone.widgets.strength_dropdown_box.entry.set_text("");
                self_clone.widgets.formulation_dropdown_box.entry.set_text("");
                self_clone.widgets.brand_name_search_box.entry.grab_focus();
            }
        });
        // let medicine_row = widgets::medicine_row::MedicineRow::new(
        //     "Cream".to_string(), "Lulider".to_string(), 
        //     "Luliconazole".to_string(), "10 mg/gm".to_string()
        // );
        // self.widgets.medicine_box.append(medicine_row);
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
    pub generic_name_search_box: Rc<widgets::search_box::SearchBox>,
    pub brand_name_search_box: Rc<widgets::search_box::SearchBox>,
    pub strength_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub formulation_dropdown_box: Rc<widgets::dropdown_box::DropdownBox>,
    pub add_button: gtk::Button,
    pub medicine_box: Rc<widgets::medicine_box::MedicineBox>
}

impl AppWidgets {
    pub fn new() -> Self {
        let container = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .margin_start(0).margin_end(0).margin_top(0).margin_bottom(0)
            .hexpand(true)
            .build();
        let grid = gtk::Grid::builder()
            .margin_top(16).margin_bottom(16)
            .margin_start(16).margin_end(16)
            .column_spacing(16)
            .row_spacing(8)
            .hexpand(true)
            .build();
        
        let generic_name_search_box = widgets::search_box::SearchBox::new();
        generic_name_search_box.entry.set_placeholder_text(Some("Generic Name"));
        
        let brand_name_search_box = widgets::search_box::SearchBox::new();
        brand_name_search_box.entry.set_placeholder_text(Some("Brand Name"));
        
        let strength_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        strength_dropdown_box.entry.set_placeholder_text(Some("Strength"));
        
        let formulation_dropdown_box = widgets::dropdown_box::DropdownBox::new();
        formulation_dropdown_box.entry.set_placeholder_text(Some("Dosage Formulation"));
        
        let add_button = widgets::horizontal_icon_button("Add", "list-add");
        let medicine_box = widgets::medicine_box::MedicineBox::new();
        
        Self { 
            container, grid, 
            generic_name_search_box, brand_name_search_box, 
            strength_dropdown_box, formulation_dropdown_box,
            add_button, medicine_box
        }
    }
}
