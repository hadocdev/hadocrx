use gtk::{glib::{self, object::IsA}, prelude::{BoxExt, ButtonExt, FrameExt, WidgetExt}, Orientation};
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
pub fn label_left_aligned(text: &str) -> gtk::Label {
    gtk::Label::builder().label(text).halign(gtk::Align::Start).build()
}

#[allow(dead_code)]
pub fn label_with_class(text: &str, css_class: Option<&str>) -> gtk::Label {
    let label = gtk::Label::builder().label(text).valign(gtk::Align::Baseline).build();
    if let Some(class) = css_class {
        label.add_css_class(class);
    }
    label
}

#[allow(dead_code)]
pub fn alert_dialog<F: Fn(&gtk::Button) + 'static>
(title_text: &str, body_text: &str, btn_text: &str, on_btn_click: F) -> gtk::Frame {
    let frame = gtk::Frame::builder()
        .hexpand(true)
        .vexpand(true)
        .build();
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_start(16).margin_end(16).margin_top(16).margin_bottom(16)
        .build();
    frame.set_child(Some(&vbox));
    vbox.set_spacing(8);
    let title = label_with_class(title_text, Some("title-3"));
    let body = label(body_text);
    body.set_wrap(true);
    let btn_ok = gtk::Button::with_label(btn_text); 
    btn_ok.connect_clicked(on_btn_click);
    vbox.append(&title);
    vbox.append(&body);
    vbox.append(&vspacer());
    vbox.append(&btn_ok);
    frame
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
