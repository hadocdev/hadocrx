use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::{gio, Label, ListItem, ListScrollFlags, ListView, ScrolledWindow, SignalListItemFactory, SingleSelection};
use gtk::prelude::*;

use crate::hadocrx;

#[allow(dead_code)]
pub struct SearchBox {
    pub entry: gtk::SearchEntry,
    pub popover: gtk::Popover,
    pub expected_programmatic_change: RefCell<Option<String>>
}

#[allow(dead_code)]
impl SearchBox {
    pub fn new() -> Rc<Self> {
        let entry = gtk::SearchEntry::new();
        let popover = gtk::Popover::builder()
            .has_arrow(false)
            .autohide(false)
            .can_focus(false)
            .build();
        Rc::new(Self { 
            entry, popover, 
            expected_programmatic_change: RefCell::new(None)
        })
    }

    pub fn initialize(self: &Rc<Self>, data: Vec<String>) {
        self.popover.set_parent(&self.entry); 
        
        let self_clone = self.clone();
        self.entry.connect_search_changed(move |object| { 
            self_clone.handle_search_changed(object, data.clone());
        });

        let self_clone = self.clone();
        self.entry.connect_activate(move |object| {
            self_clone.handle_activate(object);
        });

        let self_clone = self.clone();
        let key_event_controller = gtk::EventControllerKey::new();
        key_event_controller.connect_key_released(move |_, key, _, _| {
            self_clone.handle_key_released(key); 
        });
        let self_clone = self.clone();
        key_event_controller.connect_key_pressed(move |_, key, _, _| {
            self_clone.handle_key_pressed(key) 
        });
        self.entry.add_controller(key_event_controller);
    }

    fn handle_key_pressed(&self, key: Key) -> Propagation {
        if self.popover.is_visible() {
            match key {
               Key::Up | Key::Down => { return Propagation::Stop; },
               _ => { return Propagation::Proceed; }
            }
        }
        gtk::glib::Propagation::Proceed
    }

    fn handle_key_released(&self, key: Key) {
        if self.popover.is_visible() {
            let scrollable_area = self.popover.child().unwrap();
            let first_child = scrollable_area.first_child().unwrap();
            let list_view = first_child.downcast_ref::<ListView>().unwrap();            
            let model = list_view.model().unwrap();
            let position = model.downcast_ref::<SingleSelection>().unwrap().selected();
            let total_items = model.n_items();
            let previous = (position + total_items - 1) % total_items;
            let next = (position + 1) % total_items;
            match key {
                Key::Up => { 
                    model.select_item(previous, true); 
                    list_view.scroll_to(previous, ListScrollFlags::FOCUS, None);
                },
                Key::Down => { 
                    model.select_item(next, true); 
                    list_view.scroll_to(next, ListScrollFlags::FOCUS, None);
                },
                _ => {}
            }
        }
    }

    fn handle_activate(&self, object: &gtk::SearchEntry) {
        if self.popover.is_visible() {
            let scrollable_area = self.popover.child().unwrap();
            let first_child = scrollable_area.first_child().unwrap();
            let list_view = first_child.downcast_ref::<ListView>().unwrap();
            let text = Self::get_selected_item_text(list_view);

            *self.expected_programmatic_change.borrow_mut() = Some(text.clone()); 

            object.set_text(text.as_str()); 
            object.set_position(-1);
            self.popover.popdown(); 
        }
    }

    fn handle_search_changed(self: &Rc<Self>, object: &gtk::SearchEntry, data: Vec<String>) {
        let current_text = object.text().to_string();
        let mut mutable_ref = self.expected_programmatic_change.borrow_mut();
        if let Some(expected) = mutable_ref.as_ref() {
            if current_text == *expected {
                *mutable_ref = None;
                return; 
            }
        }
        let query = object.text().to_string();
        if query.is_empty() {
            self.popover.popdown();
            return;
        }
        let lower_query = query.to_lowercase();
        let mut matched_items: Vec<(String, i64)> = Vec::new();
        for item in data.clone() {
            let item_text = item.to_string();
            if let Some(score) = hadocrx::utils::fuzzy_match(item_text.as_str(), &lower_query) {
                matched_items.push((item_text, score));
            }
        } 
        matched_items.sort_by(|a, b| b.1.cmp(&a.1));
        if matched_items.is_empty() {
            self.popover.popdown();
            return;
        }
        let sorted_names = matched_items.iter().map(|s| s.0.clone()).collect::<Vec<String>>();
        let model = Self::create_model_gio_liststore(sorted_names);
        let factory = Self::create_factory();
        let selection_model = SingleSelection::new(Some(model));
        let list_view = ListView::new(Some(selection_model), Some(factory));
        list_view.set_single_click_activate(true);
        let self_clone = self.clone();
        list_view.connect_activate(move |_, _| {
            self_clone.entry.emit_activate(); 
        });
        let scrollable_area = ScrolledWindow::builder()
            .child(&list_view)
            .min_content_width(object.width())
            .min_content_height(200)
            .build();

        self.popover.set_child(Some(&scrollable_area));
        self.popover.popup();
    }
    
    fn get_selected_item_text(list_view: &ListView) -> String {
        let selection_model = list_view.model().unwrap();
        let selected_item = selection_model.downcast_ref::<SingleSelection>().unwrap().selected_item().unwrap();
        selected_item.property("text")
    }

    fn create_model_gio_liststore(data: Vec<String>) -> gio::ListStore {
        let model = gio::ListStore::new::<crate::models::text_object::TextObject>();
        for item in data {
            let text_object = crate::models::text_object::TextObject::new(item.to_string());
            model.append(&text_object);
        }
        model
    }

    fn create_factory() -> SignalListItemFactory {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, item| {
            let hbox = super::hbox();
            hbox.set_margin_start(12);
            item.downcast_ref::<ListItem>().unwrap().set_child(Some(&hbox));
        });
        factory.connect_bind(move |_, item| {
            let text_object = item.downcast_ref::<ListItem>().unwrap()
                .item().and_downcast::<crate::models::text_object::TextObject>().unwrap();
            let hbox = item.downcast_ref::<ListItem>().unwrap().child().and_downcast::<gtk::Box>().unwrap();
            while let Some(child) = hbox.first_child() {
                hbox.remove(&child); 
            }
            let label = Label::builder()
                .label(text_object.text().as_str())
                .css_classes(["list_view_item"])
                .build();
            hbox.append(&label);
        });
        factory
    }
}
