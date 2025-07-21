mod subclass {
    use std::cell::RefCell;
    use gtk::glib;
    use gtk::prelude::ObjectExt;
    use gtk::glib::{subclass::{object::ObjectImpl, types::ObjectSubclass}, Properties};
    use gtk::subclass::prelude::DerivedObjectProperties;
    
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::TextObject)]
    pub struct TextObject {
        #[property(name = "text", get, set, type = String)]
        text: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TextObject {
        const NAME: &'static str = "TextObject";
        type Type = super::TextObject;
        type ParentType = gtk::glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for TextObject {}
}

gtk::glib::wrapper! {
    pub struct TextObject(ObjectSubclass<subclass::TextObject>);
}

impl TextObject {
    pub fn new(text: String) -> Self { // Takes String
        gtk::glib::Object::builder().property("text", text).build()
    }
}
