use std::io::Error;
use web_sys::{Document, window};
use crate::models::{Component, view};

pub struct ClawApp;

impl ClawApp {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn attach(&self, component: &dyn Component) -> Result<(), Error> {
        let window = window().expect("Could not find global `window`");
        let document: Document = window.document().expect("Could not find global `document`");

        let html = view(component.template().as_ref());

        document.get_element_by_id("app")
            .expect("could not find wasm placeholder")
            .set_inner_html(&html);

        Ok(())
    }
}

impl Default for ClawApp {
    fn default() -> Self {
        Self::new()
    }
}
