use std::{cell::RefCell, rc::Rc};

use gtk::{gdk::{Key, ModifierType}, glib, prelude::WidgetExt};

#[allow(dead_code)]
pub struct AvroPhoneticEntry {
    pub entry: gtk::Entry,
    pub english_buffer: RefCell<String>,
    pub cursor_position: RefCell<usize>,
}

impl AvroPhoneticEntry {
    pub fn new() -> Rc<Self> {
        Rc::new(Self { 
            entry: gtk::Entry::new(),
            english_buffer: RefCell::new(String::new()),
            cursor_position: RefCell::new(0)
        })
    }

    pub fn initialize(self: &Rc<Self>) {
        let key_controller = gtk::EventControllerKey::new();
        let self_clone = self.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            self_clone.handle_key_press(keyval, state)
        });
        let self_clone = self.clone();
        key_controller.connect_key_released(move |_, keyval, _, state| {
            self_clone.handle_key_release(keyval, state)
        });
        self.entry.add_controller(key_controller);
    }

    fn handle_key_press(self: &Rc<Self>, keyval: Key, _state: ModifierType) -> glib::Propagation {
        if keyval.to_unicode().is_some() {
            return glib::Propagation::Stop;
        }
        glib::Propagation::Proceed
    }

    fn handle_key_release(self: &Rc<Self>, keyval: Key, _state: ModifierType) {
        if keyval == Key::BackSpace {
            self.english_buffer.borrow_mut().pop();
        } else if let Some(ch) = keyval.to_unicode() {
            let mut buffer = self.english_buffer.borrow_mut();
            buffer.push(ch);
        }
    }

}
