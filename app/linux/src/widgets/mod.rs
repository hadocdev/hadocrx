use gtk::{glib::{self, object::IsA}, prelude::{BoxExt, ButtonExt, WidgetExt}, Orientation};
pub mod search_box; 
pub mod dropdown_box; 
pub mod utils;
pub mod medicine_row;
pub mod medicine_box;
pub mod bangla_entry;

const DEFAULT_MARGIN: i32 = 4;

#[allow(dead_code)]
pub fn label(text: &str) -> gtk::Label {
    gtk::Label::builder().label(text).build()
}

#[allow(dead_code)]
pub fn button<F: Fn(&gtk::Button) + 'static>
(label: &str, on_click: F) -> (gtk::Button, glib::SignalHandlerId) {
    let button = gtk::Button::builder()
        .label(label)
        .margin_start(DEFAULT_MARGIN).margin_end(DEFAULT_MARGIN)
        .margin_top(DEFAULT_MARGIN).margin_bottom(DEFAULT_MARGIN)
        .build();
    let signal_handler_id = button.connect_clicked(on_click);
    (button, signal_handler_id)
}

#[allow(dead_code)]
pub fn combo_button_fn<T: IsA<gtk::Widget>>(orientation: gtk::Orientation, children: Vec<Box<T>>) -> gtk::Button {
    let container = gtk::Box::builder()
        .orientation(orientation)
        .margin_start(DEFAULT_MARGIN).margin_end(DEFAULT_MARGIN)
        .margin_top(DEFAULT_MARGIN).margin_bottom(DEFAULT_MARGIN)
        .spacing(8)
        .build();
    for child in children {
        container.append(&*child);
    }
    gtk::Button::builder().child(&container).build()
}

#[macro_export]
macro_rules! combo_button {
    ( $orientation:expr, $($val:expr),+ ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Box::new(Into::<gtk::Widget>::into($val)));
            )+
            crate::widgets::combo_button_fn($orientation, temp_vec)
        }  
    };
}
pub (super) use combo_button;

#[allow(dead_code)]
pub fn vertical_icon_button(text: &str, icon_name: &str) -> gtk::Button {
    let vbox = vbox();
    vbox.set_spacing(8);
    let label = label(text);
    let icon = gtk::Image::builder().icon_name(icon_name).build();
    vbox.append(&icon);
    vbox.append(&label);
    gtk::Button::builder().child(&vbox).build()
}

#[allow(dead_code)]
pub fn custom_icon_button(icon_name: &str) -> gtk::Button {
    gtk::Button::builder().icon_name(&utils::get_theme_aware_icon_name(icon_name)).build()
}

#[allow(dead_code)]
pub fn set_margins(widget: &gtk::Widget, margin: i32) {
    widget.set_margin_start(margin);
    widget.set_margin_end(margin);
    widget.set_margin_top(margin);
    widget.set_margin_bottom(margin);
}


#[allow(dead_code)]
pub fn hspacer() -> gtk::Box {
    gtk::Box::builder().hexpand(true).build()
}

#[allow(dead_code)]
pub fn vspacer() -> gtk::Box {
    gtk::Box::builder().vexpand(true).build()
}

#[allow(dead_code)]
pub fn vbox() -> gtk::Box {
    gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_start(DEFAULT_MARGIN).margin_end(DEFAULT_MARGIN)
        .margin_top(DEFAULT_MARGIN).margin_bottom(DEFAULT_MARGIN)
        .build()
}

#[allow(dead_code)]
pub fn hbox() -> gtk::Box {
    gtk::Box::builder()
        .orientation(Orientation::Horizontal)
        .margin_start(DEFAULT_MARGIN).margin_end(DEFAULT_MARGIN)
        .margin_top(DEFAULT_MARGIN).margin_bottom(DEFAULT_MARGIN)
        .build()
}
