use std::{error::Error, io::Error};

use web_sys::window;

use crate::core::utils::WebDocument;

pub trait HtmlElement {
    fn get_tag(&self) -> &str;
    fn get_html(&self) -> String;
    fn get_classes(&self) -> Option<String>;
    fn get_id(&self) -> Option<String>;
}

pub type Html = Box<dyn HtmlElement>;

pub fn view(element: &dyn HtmlElement) -> String {
    element.get_html()
}

pub fn web_component(element: &dyn HtmlElement) -> Html {
    let doc = WebDocument::get();
}
