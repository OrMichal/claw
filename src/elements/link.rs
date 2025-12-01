use crate::models::HtmlElement;

pub struct Link {
    pub classes: Option<String>,
    pub id: Option<String>
}

impl HtmlElement for Link {
    fn get_html(&self) -> String {
        format!("<{}>", self.get_tag())
    }

    fn get_tag(&self) -> &str {
        "link"
    }

    fn get_classes(&self) -> Option<&String> {
        match &self.classes {
            Some(classes) => Some(classes),
            None => None
        }
    }

    fn get_id(&self) -> Option<&String> {
        match &self.id {
            Some(id) => Some(id),
            None => None
        }
    }
}
