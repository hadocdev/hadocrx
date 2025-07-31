use gtk::{glib, prelude::{ButtonExt, WidgetExt}, Orientation};
pub mod search_box; 

const DEFAULT_MARGIN: i32 = 4;

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
pub fn set_margins(widget: &gtk::Widget, margin: i32) {
    widget.set_margin_start(margin);
    widget.set_margin_end(margin);
    widget.set_margin_top(margin);
    widget.set_margin_bottom(margin);
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
