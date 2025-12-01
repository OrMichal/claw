use crate::models::HtmlElement;

pub struct StyleLink {
    pub src: String,
    pub classes: Option<String>,
    pub id: Option<String>
}

impl HtmlElement for StyleLink {
    fn get_tag(&self) -> &str {
        "link"
    }

    fn get_html(&self) -> String {
        format!("<{} rel=\"stylesheet\" href=\"{}\" >", self.get_tag(), self.src)
    }

    fn get_id(&self) -> Option<&String> {
        match &self.id {
            Some(id) => Some(id),
            None => None
        }
    }

    fn get_classes(&self) -> Option<&String> {
        match &self.classes {
            Some(classes) => Some(classes),
            None => None
        }
    }
}
