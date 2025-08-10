use std::cell::RefCell;
use gtk::{
    gdk::{Key, ModifierType},
    glib::{self, Object},
    prelude::*,
    subclass::prelude::*,
};
use crate::hadocrx;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct AvroPhoneticEntry {
        pub english_buffer: RefCell<String>,
        pub processed_buffer: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AvroPhoneticEntry {
        const NAME: &'static str = "AvroPhoneticEntry";
        type Type = super::AvroPhoneticEntry;
        type ParentType = gtk::Entry;
    }

    impl ObjectImpl for AvroPhoneticEntry {
        fn constructed(&self) {
            self.parent_constructed();
            
            let key_controller = gtk::EventControllerKey::new();
            key_controller.set_propagation_phase(gtk::PropagationPhase::Capture);
            
            let obj = self.obj();
            key_controller.connect_key_pressed(glib::clone!(
                #[weak] obj,
                #[upgrade_or] glib::Propagation::Proceed,
                move |_, keyval, _, state| {
                    obj.handle_key_press(keyval, state)
                }
            ));
            
            obj.add_controller(key_controller);
        }
    }

    impl WidgetImpl for AvroPhoneticEntry {}
    impl EntryImpl for AvroPhoneticEntry {} 
}

glib::wrapper! {
    pub struct AvroPhoneticEntry(ObjectSubclass<imp::AvroPhoneticEntry>)
        @extends gtk::Entry, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, 
                   gtk::Editable, gtk::CellEditable;
}

impl AvroPhoneticEntry {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn clear(&self) {
        self.set_text("");
        let imp = self.imp();
        imp.english_buffer.borrow_mut().clear();
        imp.processed_buffer.borrow_mut().clear();
    } 
    
    fn handle_key_press(&self, keyval: Key, state: ModifierType) -> glib::Propagation {
            let imp = self.imp();
            // Skip if modifier keys are pressed (except Shift for some cases)
            match state {
                ModifierType::CONTROL_MASK | ModifierType::ALT_MASK | ModifierType::SUPER_MASK => {
                    imp.english_buffer.borrow_mut().clear(); 
                    return glib::Propagation::Proceed;
                }
                ModifierType::SHIFT_MASK => {
                    match keyval {
                        Key::Left | Key::Right => return glib::Propagation::Proceed,
                        _ => {}
                    }
                }
                _ => {}
            }

            
            if keyval == Key::BackSpace {
                let mut buffer = imp.english_buffer.borrow_mut();
                if !buffer.is_empty() {
                    // Check if there's a selection
                    if let Some((start, end)) = self.selection_bounds() {
                        if start != end {
                            buffer.clear();
                            return glib::Propagation::Proceed;
                        }
                    }
                    buffer.pop();
                }
            } else if keyval == Key::Tab {
                return glib::Propagation::Proceed;
            } else if keyval == Key::Delete {
                // Handle delete key if needed
            } else if keyval == Key::space {
                imp.english_buffer.borrow_mut().clear();
            } else if let Some(ch) = keyval.to_unicode() {
                let mut buffer = imp.english_buffer.borrow_mut();
                if buffer.is_empty() {
                    *imp.processed_buffer.borrow_mut() = self.text().to_string();
                }
                buffer.push(ch);
            }

            let buffer = imp.english_buffer.borrow();
            if !buffer.is_empty() {
                let mut entry_text = imp.processed_buffer.borrow().clone();
                entry_text.push_str(&hadocrx::avro_phonetic::convert(&buffer));
                self.set_text(&entry_text);
                self.set_position(-1);
                return glib::Propagation::Stop;
            }

            glib::Propagation::Proceed
        }
}

impl Default for AvroPhoneticEntry {
    fn default() -> Self {
        Self::new()
    }
}
