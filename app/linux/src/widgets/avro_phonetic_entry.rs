#![allow(deprecated)]

use std::{cell::RefCell, ops::Not};
use gtk::{
    gdk::{Key, ModifierType},
    glib::{self, Object},
    prelude::*,
    subclass::prelude::*,
};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct AvroPhoneticEntry {
        pub is_bangla_mode: RefCell<bool>,
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
            obj.update_secondary_icon();
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

    fn update_secondary_icon(&self) {
        let imp = self.imp();
        let icon_name = if *imp.is_bangla_mode.borrow() { "bn" } else { "en" };
        self.set_secondary_icon_name(Some(&crate::widgets::utils::get_theme_aware_icon_name(icon_name)));
    }
    
    fn handle_key_press(&self, keyval: Key, state: ModifierType) -> glib::Propagation {
        let imp = self.imp();
        // Skip if modifier keys are pressed (except Shift for some cases)
        match state {
            ModifierType::CONTROL_MASK | ModifierType::ALT_MASK | ModifierType::SUPER_MASK => {
                imp.english_buffer.borrow_mut().clear(); 
                if state == ModifierType::CONTROL_MASK && keyval == Key::m {
                    let mut mutable_borrow = imp.is_bangla_mode.borrow_mut();
                    *mutable_borrow = mutable_borrow.not();
                    drop(mutable_borrow);
                    self.update_secondary_icon();
                }
                return glib::Propagation::Proceed;
            }
            ModifierType::SHIFT_MASK => {
                if keyval == Key::Left || keyval == Key::Right { return glib::Propagation::Proceed; }
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
        if *imp.is_bangla_mode.borrow() && !buffer.is_empty() {
            let mut entry_text = imp.processed_buffer.borrow().clone();
            entry_text.push_str(&hadocrx::ffi::avro_phonetic::convert(&buffer));
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
