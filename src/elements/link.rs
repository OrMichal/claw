use crate::models::HtmlElement;

pub struct Link;

impl HtmlElement for Link {
    fn get_html(&self) -> String {
        format!("<{}>", self.get_tag())
    }

    fn get_tag(&self) -> &str {
        "link"
    }

    fn get_classes(&self) -> Option<String> {
        None
    }

    fn get_id(&self) -> Option<String> {
        None
    }
}
