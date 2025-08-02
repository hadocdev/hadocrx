use std::cell::RefCell;
use std::rc::Rc;

use gtk::gdk::Key;
use gtk::glib::Propagation;
use gtk::{EntryIconPosition, ListScrollFlags, ListView, ScrolledWindow, SingleSelection};
use gtk::prelude::*;

use crate::hadocrx;

#[allow(dead_code)]
pub struct DropdownBox {
    pub entry: gtk::Entry,
    pub popover: gtk::Popover,
    pub expected_programmatic_change: RefCell<Option<String>>
}

#[allow(dead_code)]
impl DropdownBox {
    pub fn new() -> Rc<Self> {
        let entry = gtk::Entry::builder()
            .secondary_icon_name("pan-down-symbolic")
            .secondary_icon_sensitive(false)
            .build();
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
        
        if let Some(scrollable_area) = self.create_scollable_area(String::new(), data.clone()) {
            self.popover.set_child(Some(&scrollable_area));
        }
        
        let self_clone = self.clone();
        self.entry.connect_changed(move |entry| { 
            self_clone.handle_entry_changed(entry, data.clone());
        });

        let self_clone = self.clone();
        self.entry.connect_activate(move |entry| {
            self_clone.handle_entry_activate(entry);
        });

        let self_clone = self.clone();
        self.entry.set_secondary_icon_sensitive(true);
        self.entry.connect_icon_release(move |entry, position| {
            self_clone.handle_entry_icon_release(entry, position);
        });

        let self_clone = self.clone();
        let key_event_controller = gtk::EventControllerKey::new();
        key_event_controller.connect_key_released(move |_, key, _, _| {
            self_clone.handle_entry_key_released(key); 
        });
        let self_clone = self.clone();
        key_event_controller.connect_key_pressed(move |_, key, _, _| {
            self_clone.handle_entry_key_pressed(key) 
        });
        self.entry.add_controller(key_event_controller);
    }

    pub fn update(self: &Rc<Self>, data: Vec<String>) {
        if self.popover.parent() == None { self.popover.set_parent(&self.entry); }
        if let Some(child) = self.popover.child() { drop(child); }
        if let Some(scrollable_area) = self.create_scollable_area(String::new(), data.clone()) {
            self.popover.set_child(Some(&scrollable_area));
        }
    }

    pub fn update_entry_text(&self, text: String) {
        self.entry.set_text(&text);
        *self.expected_programmatic_change.borrow_mut() = Some(text.clone()); 
        self.entry.set_position(-1);
        self.entry.set_secondary_icon_name(Some("pan-down-symbolic"));
        if self.popover.is_visible() {
            self.popover.popdown(); 
        }
    }

    fn handle_entry_icon_release(&self, entry: &gtk::Entry, _position: EntryIconPosition) {
        if self.popover.is_visible() { 
            entry.set_secondary_icon_name(Some("pan-down-symbolic"));
            self.popover.popdown(); 
        } 
        else {
            entry.set_secondary_icon_name(Some("pan-up-symbolic"));
            self.popover.popup(); 
        }
    }

    fn handle_entry_key_pressed(&self, key: Key) -> Propagation {
        if self.popover.is_visible() {
            match key {
               Key::Up | Key::Down => { return Propagation::Stop; },
               _ => { return Propagation::Proceed; }
            }
        }
        gtk::glib::Propagation::Proceed
    }

    fn handle_entry_key_released(&self, key: Key) {
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

    fn handle_entry_activate(&self, _entry: &gtk::Entry) {
        let scrollable_area = self.popover.child().unwrap();
        let first_child = scrollable_area.first_child().unwrap();
        let list_view = first_child.downcast_ref::<ListView>().unwrap();
        let text = super::utils::get_selected_item_text_from_list_view(list_view);
        self.update_entry_text(text);
    }

    fn handle_entry_changed(self: &Rc<Self>, entry: &gtk::Entry, data: Vec<String>) {
        let current_text = entry.text().to_string();
        let mut mutable_ref = self.expected_programmatic_change.borrow_mut();
        if let Some(expected) = mutable_ref.as_ref() {
            if current_text == *expected {
                *mutable_ref = None;
                return; 
            }
        }
        let query = entry.text().to_string();
        if query.is_empty() {
            self.popover.popdown();
            return;
        }
        if let Some(scrollable_area) = self.create_scollable_area(query, data) {
            self.popover.set_child(Some(&scrollable_area));
            self.popover.popup();
        }
    }

    fn create_scollable_area(self: &Rc<Self>, query: String, data: Vec<String>) -> Option<ScrolledWindow>{
        let lower_query = query.to_lowercase();
        let mut matched_items: Vec<(String, i64)> = Vec::new();
        for item in data.clone() {
            let item_text = item.to_string();
            let score = hadocrx::utils::fuzzy_match(item_text.as_str(), &lower_query).unwrap_or_default();
            matched_items.push((item_text, score));
        } 
        matched_items.sort_by(|a, b| b.1.cmp(&a.1));
        if matched_items.is_empty() {
            return None;
        }
        let sorted_names = matched_items.iter().map(|s| s.0.clone()).collect::<Vec<String>>();
        let model = super::utils::create_gio_liststore_model(sorted_names);
        let factory = super::utils::create_signal_list_item_factory();
        let selection_model = SingleSelection::new(Some(model));
        let list_view = ListView::new(Some(selection_model), Some(factory));
        list_view.set_single_click_activate(true);
        let self_clone = self.clone();
        list_view.connect_activate(move |_, _| {
            self_clone.entry.emit_activate(); 
        });
        let scrollable_area = ScrolledWindow::builder()
            .child(&list_view)
            .min_content_width(self.entry.width())
            .min_content_height(200)
            .build();
        Some(scrollable_area)
    } 
}
