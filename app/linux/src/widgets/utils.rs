use gtk::{gio, glib::object::{Cast, CastNone, ObjectExt}, prelude::{BoxExt, ListItemExt, WidgetExt}, Label, ListItem, ListView, SignalListItemFactory, SingleSelection};

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
