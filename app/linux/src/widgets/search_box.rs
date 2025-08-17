use gtk::{glib::{self, subclass::types::ObjectSubclassIsExt}, prelude::{EditableExt, PopoverExt, WidgetExt}};

mod imp {
    use gtk::gdk::Key;
    use gtk::glib::subclass::InitializingObject;
    use gtk::glib::{self, Propagation};
    use gtk::{prelude::*, ListScrollFlags, ListView, ScrolledWindow, SingleSelection};
    use gtk::subclass::prelude::*;

    use std::cell::{Cell, RefCell};

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        <interface>
            <template class="SearchBox" parent="GtkWidget">
                <child>
                    <object class="GtkSearchEntry" id="entry">
                        <child>
                            <object class="GtkPopover" id="popover">
                                <property name="has-arrow">false</property>
                                <property name="autohide">false</property>
                                <property name="can-focus">false</property>
                            </object>
                        </child>
                    </object>
                </child> 
            </template>
        </interface>
    "#)]
    pub struct SearchBox {
        #[template_child]
        pub entry: TemplateChild<gtk::SearchEntry>, 
        #[template_child]
        pub popover: TemplateChild<gtk::Popover>,
        
        pub data: RefCell<Vec<String>>,
        pub expected_programmatic_change: RefCell<Option<String>>,
        pub signals_connected: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SearchBox {
        const NAME: &'static str = "SearchBox";
        type Type = super::SearchBox;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.set_layout_manager_type::<gtk::BinLayout>();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SearchBox {}
    impl WidgetImpl for SearchBox {}

    impl SearchBox {
        pub fn setup_signals(&self) {
            if self.signals_connected.get() {
                return;
            }

            let obj = self.obj();
            
            // Connect search changed signal
            self.entry.connect_search_changed(glib::clone!(
                #[weak] obj,
                move |entry| {
                    obj.imp().handle_entry_search_changed(entry);
                }
            ));

            // Connect activate signal
            self.entry.connect_activate(glib::clone!(
                #[weak] obj,
                move |entry| {
                    obj.imp().handle_entry_activate(entry);
                }
            ));

            // Setup key event controller
            let key_event_controller = gtk::EventControllerKey::new();
            
            key_event_controller.connect_key_pressed(glib::clone!(
                #[weak] obj,
                #[upgrade_or] Propagation::Proceed,
                move |_, key, _, _| {
                    obj.imp().handle_entry_key_pressed(key)
                }
            ));

            key_event_controller.connect_key_released(glib::clone!(
                #[weak] obj,
                move |_, key, _, _| {
                    obj.imp().handle_entry_key_released(key);
                }
            ));

            self.entry.add_controller(key_event_controller);
            self.signals_connected.set(true);
        }

        fn handle_entry_key_pressed(&self, key: Key) -> Propagation {
            if self.popover.is_visible() {
                match key {
                    Key::Up | Key::Down => return Propagation::Stop,
                    Key::Escape => self.popover.popdown(),
                    _ => {}
                }
            }
            Propagation::Proceed
        }

        fn handle_entry_key_released(&self, key: Key) {
            if self.popover.is_visible() {
                if let Some(scrollable_area) = self.popover.child() {
                    if let Some(first_child) = scrollable_area.first_child() {
                        if let Some(list_view) = first_child.downcast_ref::<ListView>() {
                            if let Some(model) = list_view.model() {
                                let selection = model.downcast_ref::<SingleSelection>().unwrap();
                                let position = selection.selected();
                                let total_items = model.n_items();
                                
                                match key {
                                    Key::Up => {
                                        let previous = (position + total_items - 1) % total_items;
                                        model.select_item(previous, true);
                                        list_view.scroll_to(previous, ListScrollFlags::FOCUS, None);
                                    }
                                    Key::Down => {
                                        let next = (position + 1) % total_items;
                                        model.select_item(next, true);
                                        list_view.scroll_to(next, ListScrollFlags::FOCUS, None);
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        fn handle_entry_activate(&self, _entry: &gtk::SearchEntry) {
            if let Some(scrollable_area) = self.popover.child() {
                if let Some(first_child) = scrollable_area.first_child() {
                    if let Some(list_view) = first_child.downcast_ref::<ListView>() {
                        let text = crate::widgets::utils::get_selected_item_text_from_list_view(list_view);
                        // drop(popover); // Release borrow before calling update_entry_text
                        self.obj().update_entry_text(&text);
                    }
                }
            }
        }

        fn handle_entry_search_changed(&self, object: &gtk::SearchEntry) {
            let current_text = object.text().to_string();
            
            // Check if this is a programmatic change we should ignore
            {
                let mut expected = self.expected_programmatic_change.borrow_mut();
                if let Some(expected_text) = expected.as_ref() {
                    if current_text == *expected_text {
                        *expected = None;
                        return;
                    }
                }
            }

            let query = current_text;
            if query.is_empty() {
                self.popover.popdown();
                return;
            }

            let data = self.data.borrow();
            let lower_query = query.to_lowercase();
            let mut matched_items: Vec<(String, i64)> = Vec::new();
            
            for item in data.iter() {
                if let Some(score) = hadocrx::ffi::utils::fuzzy_match(item, &lower_query) {
                    matched_items.push((item.clone(), score));
                }
            }
            
            matched_items.sort_by(|a, b| b.1.cmp(&a.1));
            
            if matched_items.is_empty() {
                self.popover.popdown();
                return;
            }

            let sorted_names: Vec<String> = matched_items.into_iter()
                .map(|(name, _)| name)
                .collect();

            let model = crate::widgets::utils::create_gio_liststore_model(sorted_names);
            let factory = crate::widgets::utils::create_signal_list_item_factory();
            let selection_model = SingleSelection::new(Some(model));
            let list_view = ListView::new(Some(selection_model), Some(factory));
            list_view.set_single_click_activate(true);

            // Connect the activate signal for the list view
            list_view.connect_activate(glib::clone!(
                #[weak(rename_to = entry)] self.entry,
                move |_, _| {
                    entry.emit_activate();
                }
            ));

            let scrollable_area = ScrolledWindow::builder()
                .child(&list_view)
                .min_content_width(object.width())
                .min_content_height(200)
                .build();

            let popover = &self.popover;
            popover.set_child(Some(&scrollable_area));
            popover.popup();
        }
    }
}

glib::wrapper! {
    pub struct SearchBox(ObjectSubclass<imp::SearchBox>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SearchBox {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn initialize(&self, data: Vec<String>) {
        let imp = self.imp();
        imp.data.replace(data);
        imp.setup_signals();
    }

    pub fn update_entry_text(&self, text: &str) {
        let imp = self.imp();
        imp.entry.set_text(text);
        imp.expected_programmatic_change.replace(Some(text.to_string()));
        
        if super::utils::is_focused(&*imp.entry) {
            imp.entry.set_position(-1);
        }
        
        let popover = &imp.popover;
        if popover.is_visible() {
            popover.popdown();
        }
    }

    pub fn entry(&self) -> &gtk::SearchEntry {
        &self.imp().entry
    }

    pub fn popover(&self) -> &gtk::Popover {
        &self.imp().popover
    }
}

impl Default for SearchBox {
    fn default() -> Self {
        Self::new()
    }
}

// #[allow(dead_code)]
// pub struct SearchBox {
//     pub entry: gtk::SearchEntry,
//     pub popover: gtk::Popover,
//     pub expected_programmatic_change: RefCell<Option<String>>
// }
//
// #[allow(dead_code)]
// impl SearchBox {
//     pub fn new() -> Rc<Self> {
//         let entry = gtk::SearchEntry::new();
//         let popover = gtk::Popover::builder()
//             .has_arrow(false)
//             .autohide(false)
//             .can_focus(false)
//             .build();
//         Rc::new(Self { 
//             entry, popover, 
//             expected_programmatic_change: RefCell::new(None)
//         })
//     }
//
//     pub fn initialize(self: &Rc<Self>, data: Vec<String>) {
//         self.popover.set_parent(&self.entry); 
//         
//         let self_clone = self.clone();
//         self.entry.connect_search_changed(move |object| { 
//             self_clone.handle_entry_search_changed(object, data.clone());
//         });
//
//         let self_clone = self.clone();
//         self.entry.connect_activate(move |object| {
//             self_clone.handle_entry_activate(object);
//         });
//
//         let self_clone = self.clone();
//         let key_event_controller = gtk::EventControllerKey::new();
//         key_event_controller.connect_key_released(move |_, key, _, _| {
//             self_clone.handle_entry_key_released(key); 
//         });
//         let self_clone = self.clone();
//         key_event_controller.connect_key_pressed(move |_, key, _, _| {
//             self_clone.handle_entry_key_pressed(key) 
//         });
//         self.entry.add_controller(key_event_controller);
//     }
//
//     pub fn update_entry_text(&self, text: String) {
//         self.entry.set_text(&text);
//         *self.expected_programmatic_change.borrow_mut() = Some(text.clone()); 
//         if super::utils::is_focused(&self.entry) { self.entry.set_position(-1); }
//         if self.popover.is_visible() {
//             self.popover.popdown(); 
//         }
//     }
//
//     fn handle_entry_key_pressed(&self, key: Key) -> Propagation {
//         if self.popover.is_visible() {
//             match key {
//                 Key::Up | Key::Down => { return Propagation::Stop; },
//                 Key::Escape => { self.popover.popdown(); },
//                 _ => {}
//             }
//         }         
//         Propagation::Proceed
//     }
//
//     fn handle_entry_key_released(&self, key: Key) {
//         if self.popover.is_visible() {
//             let scrollable_area = self.popover.child().unwrap();
//             let first_child = scrollable_area.first_child().unwrap();
//             let list_view = first_child.downcast_ref::<ListView>().unwrap();            
//             let model = list_view.model().unwrap();
//             let position = model.downcast_ref::<SingleSelection>().unwrap().selected();
//             let total_items = model.n_items();
//             let previous = (position + total_items - 1) % total_items;
//             let next = (position + 1) % total_items;
//             match key {
//                 Key::Up => { 
//                     model.select_item(previous, true); 
//                     list_view.scroll_to(previous, ListScrollFlags::FOCUS, None);
//                 },
//                 Key::Down => { 
//                     model.select_item(next, true); 
//                     list_view.scroll_to(next, ListScrollFlags::FOCUS, None);
//                 },
//                 _ => {}
//             }
//         }
//     }
//
//     fn handle_entry_activate(&self, _entry: &gtk::SearchEntry) {
//         if let Some(scrollable_area) = self.popover.child() {
//             let first_child = scrollable_area.first_child().unwrap();
//             let list_view = first_child.downcast_ref::<ListView>().unwrap();
//             let text = super::utils::get_selected_item_text_from_list_view(list_view);
//             self.update_entry_text(text);
//         }
//     }
//
//     fn handle_entry_search_changed(self: &Rc<Self>, object: &gtk::SearchEntry, data: Vec<String>) {
//         let current_text = object.text().to_string();
//         let mut mutable_ref = self.expected_programmatic_change.borrow_mut();
//         if let Some(expected) = mutable_ref.as_ref() {
//             if current_text == *expected {
//                 *mutable_ref = None;
//                 return; 
//             }
//         }
//         let query = object.text().to_string();
//         if query.is_empty() {
//             self.popover.popdown();
//             return;
//         }
//         let lower_query = query.to_lowercase();
//         let mut matched_items: Vec<(String, i64)> = Vec::new();
//         for item in data.clone() {
//             let item_text = item.to_string();
//             if let Some(score) = hadocrx::utils::fuzzy_match(item_text.as_str(), &lower_query) {
//                 matched_items.push((item_text, score));
//             }
//         } 
//         matched_items.sort_by(|a, b| b.1.cmp(&a.1));
//         if matched_items.is_empty() {
//             self.popover.popdown();
//             return;
//         }
//         let sorted_names = matched_items.iter().map(|s| s.0.clone()).collect::<Vec<String>>();
//         let model = super::utils::create_gio_liststore_model(sorted_names);
//         let factory = super::utils::create_signal_list_item_factory();
//         let selection_model = SingleSelection::new(Some(model));
//         let list_view = ListView::new(Some(selection_model), Some(factory));
//         list_view.set_single_click_activate(true);
//         let self_clone = self.clone();
//         list_view.connect_activate(move |_, _| {
//             self_clone.entry.emit_activate(); 
//         });
//         let scrollable_area = ScrolledWindow::builder()
//             .child(&list_view)
//             .min_content_width(object.width())
//             .min_content_height(200)
//             .build();
//
//         self.popover.set_child(Some(&scrollable_area));
//         self.popover.popup();
//     } 
// }
