use gtk::{gio, glib::object::{Cast, CastNone, ObjectExt}, prelude::{BoxExt, ListItemExt, RootExt, WidgetExt}, Label, ListItem, ListView, SignalListItemFactory, SingleSelection};

#[allow(dead_code)]
pub fn create_gio_liststore_model(data: Vec<String>) -> gio::ListStore {
    let model = gio::ListStore::new::<crate::models::text_object::TextObject>();
    for item in data {
        let text_object = crate::models::text_object::TextObject::new(item.to_string());
        model.append(&text_object);
    }
    model
}

#[allow(dead_code)]
pub fn create_signal_list_item_factory() -> SignalListItemFactory {
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

#[allow(dead_code)]
pub fn get_selected_item_text_from_list_view(list_view: &ListView) -> String {
    let selection_model = list_view.model().unwrap();
    let selected_item = selection_model.downcast_ref::<SingleSelection>().unwrap().selected_item().unwrap();
    selected_item.property("text")
}

#[allow(dead_code)]
pub fn get_theme_aware_icon_name(icon_name: &str) -> String {
    let mut output = icon_name.to_string();
    let settings = gtk::Settings::for_display(&gtk::gdk::Display::default().unwrap());
    let should_be_dark = settings.is_gtk_application_prefer_dark_theme() || std::env::var("GTK_THEME").unwrap().contains(":dark");
    if should_be_dark {
        output.push_str("-dark");
    } 
    output
}

#[allow(dead_code)]
pub fn is_focused<W: WidgetExt>(widget: &W) -> bool {
    if let Some(root) = widget.root() {
        if let Some(focused) = root.focus() {
            return focused.is_ancestor(widget);
        }
        return false;
    }
    false
}

#[macro_export]
macro_rules! validation_errors {
    ( $($item:ident),+ ) => {
        {
            let mut empty_items = Vec::new();
            $(
                let item_name = stringify!($item);
                if $item.is_empty() {
                    empty_items.push(
                        item_name.to_string().split("_").map(|s| s.to_uppercase()).collect::<Vec<String>>().join(" ")
                    );
                }
            )*
            if empty_items.is_empty() {
                None
            } else if empty_items.len() == 1{
                Some(format!("{} is required", empty_items[0]))
            } else {
                let mut error_message = empty_items[0].clone();
                for i in 1..empty_items.len()-1 {
                    error_message.push_str(", ");
                    error_message.push_str(&empty_items[i]);
                }
                error_message.push_str(" and ");
                error_message.push_str(&empty_items.last().unwrap());
                error_message.push_str(" are required");
                Some(error_message)
            }
        }  
    };
}
pub (crate) use validation_errors;
