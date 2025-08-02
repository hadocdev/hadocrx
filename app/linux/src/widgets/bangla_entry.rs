use std::rc::Rc;

pub struct BanglaEntry {
    pub entry: gtk::Entry
}

impl BanglaEntry {
    pub fn new() -> Rc<Self> {
        let entry = gtk::Entry::new();
        Rc::new(Self { entry })
    }
}
