use std::rc::Rc;
use std::sync::RwLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use gtk::{gio, Label, ListItem, ListView, Popover, ScrolledWindow, SearchEntry, SignalListItemFactory, SingleSelection};
use gtk::prelude::*;

use crate::hadocrx;

static LAST_UPDATED: RwLock<Duration> = RwLock::new(Duration::ZERO);

#[allow(dead_code)]
pub fn new<T>(data: T) -> Rc<SearchEntry> 
where T: IntoIterator + Clone + 'static, T::Item: ToString {
    let entry = SearchEntry::builder()
        .hexpand(true)
        .placeholder_text("Search by Brand or Generic Name")
        .build();

    let popover = Popover::builder()
        .has_arrow(false)
        .autohide(false)
        .can_focus(false)
        .build();
    
    popover.set_parent(&entry);

    let entry_rc = Rc::new(entry);
    let popover_rc = Rc::new(popover);
    let popover_clone = popover_rc.clone();
    let popover_clone_for_unparent = popover_rc.clone(); 

    {
        let mut last_updated = LAST_UPDATED.write().unwrap();
        *last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    }

    entry_rc.connect_search_changed(move |object| { 
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let elapsed;
        {
            let last_updated = LAST_UPDATED.read().unwrap();
            elapsed = now - *last_updated;
        }
        if elapsed < Duration::from_millis(object.search_delay() as u64 + 1) { return; }
        let query = object.text().to_string();
        if query.is_empty() {
            popover_rc.popdown();
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
            popover_rc.popdown();
            return;
        }
        let sorted_names = matched_items.iter().map(|s| s.0.clone()).collect::<Vec<String>>();
        let model = create_model_gio_liststore(sorted_names);
        let factory = create_factory();
        let selection_model = SingleSelection::new(Some(model));
        let list_view = ListView::new(Some(selection_model), Some(factory));
        let toplevels = gtk::Window::toplevels();
        let mut height = 0;
        for i in 0..toplevels.n_items() {
            let item = toplevels.item(i).unwrap();
            let window = item.downcast_ref::<gtk::Window>().unwrap();
            if window.is_active() {
                height = window.size(gtk::Orientation::Vertical);
                break;
            }
        }
        let min_height = if height<250 { height/2 } else { 150 };
        let scrollable_area = ScrolledWindow::builder()
            .child(&list_view)
            .min_content_width(object.width())
            .min_content_height(min_height)
            .build();

        popover_rc.set_child(Some(&scrollable_area));
        popover_rc.popup();
    });

    entry_rc.connect_activate(move |object| {
        if popover_clone.is_visible() {
            let scrollable_area = popover_clone.child().unwrap();
            let first_child = scrollable_area.first_child().unwrap();
            let list_view = first_child.downcast_ref::<ListView>().unwrap();
            let text = get_selected_item_text(list_view);

            {
                let mut last_updated = LAST_UPDATED.write().unwrap();
                *last_updated = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            }
            object.set_text(text.as_str()); 
            object.set_position(-1);
            popover_clone.popdown();
        }
    });

    entry_rc.clone().connect_destroy(move |_| {
        popover_clone_for_unparent.unparent();
    });
    
    entry_rc.clone()
}

fn get_selected_item_text(list_view: &ListView) -> String {
    let selection_model = list_view.model().unwrap();
    let selected_item = selection_model.downcast_ref::<SingleSelection>().unwrap().selected_item().unwrap();
    selected_item.property("text")
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
