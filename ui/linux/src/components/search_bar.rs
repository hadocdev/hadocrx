use std::rc::Rc;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use gtk::{gio, Entry, EntryIconPosition, Label, ListItem, ListView, Popover, ScrolledWindow, SignalListItemFactory, SingleSelection};
use gtk::prelude::*;

#[allow(dead_code)]
pub fn new<T>
(data: T, icon_release: impl Fn(&Entry, EntryIconPosition) + 'static) -> Rc<Entry> 
where T: IntoIterator + Clone + Copy + 'static, T::Item: ToString {
    let entry = Entry::builder()
        .hexpand(true)
        .placeholder_text("Search by Brand or Generic Name")
        // https://specifications.freedesktop.org/icon-naming-spec/latest/#names
        .primary_icon_name("system-search")
        .secondary_icon_name("edit-clear")
        .build();

    let popover = Popover::builder()
        .has_arrow(false)
        .autohide(false)
        .build();
    
    popover.set_parent(&entry);

    let entry_rc = Rc::new(entry);
    let popover_rc = Rc::new(popover);
    let popover_clone = popover_rc.clone();
    let popover_clone_for_unparent = popover_rc.clone();

    entry_rc.connect_icon_release(icon_release); 

    entry_rc.connect_changed(move |object| {
        let query = object.text().to_string();
        if query.is_empty() {
            popover_rc.popdown();
            return;
        }
        let matcher = SkimMatcherV2::default();
        let lower_query = query.to_lowercase();
        let mut matched_items: Vec<(String, i64)> = Vec::new();
        for item in data {
            let item_text = item.to_string();
            if let Some(score) = matcher.fuzzy_match(item_text.as_str(), &lower_query) {
                matched_items.push((item_text, score));
            }
        } 
        matched_items.sort_by(|a, b| b.1.cmp(&a.1));
        if matched_items.is_empty() {
            popover_rc.popdown();
            return;
        }
        let sorted_names = matched_items.iter().map(|s| s.0.clone()).collect::<Vec<String>>();
        let model = create_model_gio_liststore(sorted_names);
        let factory = create_factory();
        let selection_model = SingleSelection::new(Some(model));
        let list_view = ListView::new(Some(selection_model), Some(factory));
        let scrollable_area = ScrolledWindow::builder()
            .child(&list_view)
            .min_content_width(object.width())
            .min_content_height(150)
            .build();

        popover_rc.set_child(Some(&scrollable_area));
        popover_rc.popup();
    });

    entry_rc.connect_activate(move |object| {
        let selection_model = popover_clone
            .child().unwrap()
            .first_child().unwrap().downcast_ref::<ListView>().unwrap()
            .model().unwrap();
        let selected_item = selection_model.downcast_ref::<SingleSelection>().unwrap().selected_item().unwrap();
        let text: String = selected_item.property("text");
        
        object.set_text(text.as_str());
        popover_clone.popdown();
    });

    entry_rc.clone().connect_destroy(move |_| {
        popover_clone_for_unparent.unparent();
    });
    
    entry_rc.clone()
}

#[allow(dead_code)]
fn create_model_gio_liststore<T>(data: T) -> gio::ListStore 
where T: IntoIterator, T::Item: ToString {
    let model = gio::ListStore::new::<crate::models::text_object::TextObject>();
    for item in data {
        let text_object = crate::models::text_object::TextObject::new(item.to_string());
        model.append(&text_object);
    }
    model
}

#[allow(dead_code)]
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
        let label = Label::builder()
            .label(text_object.text().as_str())
            .css_classes(["list_view_item"])
            .build();
        hbox.append(&label);
    });
    factory
}
