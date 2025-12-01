use web_sys::{Document, Window, window};

pub struct WebWindow;

impl WebWindow {
    pub fn get() -> Window {
        window().expect("Could not get global `window`")
    }
}

pub struct WebDocument;

impl WebDocument {
    pub fn get() -> Document {
        WebWindow::get().document().expect("Could not get global `document`")
    }
}
