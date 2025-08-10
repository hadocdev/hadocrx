use std::{cell::RefCell, rc::Rc};

use gtk::{gdk::{Key, ModifierType}, glib, prelude::{EditableExt, EventControllerExt, WidgetExt}};

use crate::hadocrx;

#[allow(dead_code)]
pub struct AvroPhoneticEntry {
    pub entry: gtk::Entry,
    pub english_buffer: RefCell<String>,
    pub processed_buffer: RefCell<String>
}

impl AvroPhoneticEntry {
    pub fn new() -> Rc<Self> {
        let self_rc = Rc::new(Self { 
            entry: gtk::Entry::new(),
            english_buffer: RefCell::new(String::new()),
            processed_buffer: RefCell::new(String::new())
        });
        let key_controller = gtk::EventControllerKey::new();
        key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
        let self_clone = self_rc.clone();
        key_controller.connect_key_pressed(move |_, keyval, _, state| {
            self_clone.handle_key_press(keyval, state)
        });
        self_rc.entry.add_controller(key_controller);
        self_rc
    }

    fn handle_key_press(self: &Rc<Self>, keyval: Key, state: ModifierType) -> glib::Propagation {
        match state {
            ModifierType::CONTROL_MASK | ModifierType::ALT_MASK | ModifierType::SUPER_MASK => {
                return glib::Propagation::Proceed;
            },
            ModifierType::SHIFT_MASK => {
                match keyval {
                    Key::Left | Key::Right => { return glib::Propagation::Proceed; },
                    _ => {}
                }
            }
            _ => {}
        } 
        if keyval == Key::BackSpace {
            let mut buffer = self.english_buffer.borrow_mut();
            if !buffer.is_empty() {
                if let Some((_, _)) = self.entry.selection_bounds() {
                    buffer.clear();
                    return glib::Propagation::Proceed;
                }
                buffer.pop();
            }
        } else if keyval == Key::Tab {
            return glib::Propagation::Proceed;
        } else if keyval == Key::Delete {

        } else if keyval == Key::space {
            self.english_buffer.borrow_mut().clear();
        } else if let Some(ch) = keyval.to_unicode() {
            let mut buffer = self.english_buffer.borrow_mut();
            if buffer.is_empty() { *self.processed_buffer.borrow_mut() = self.entry.text().to_string(); }
            buffer.push(ch);
        }
        let buffer = &*self.english_buffer.borrow();
        if !buffer.is_empty() {
            let mut entry_text = self.processed_buffer.borrow().clone();
            entry_text.push_str(&hadocrx::avro_phonetic::convert(buffer));
            self.entry.set_text(&entry_text);
            self.entry.set_position(-1);
            return glib::Propagation::Stop;
        }
        return glib::Propagation::Proceed;
    }
}
