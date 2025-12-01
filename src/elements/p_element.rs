use crate::models::HtmlElement;

pub struct P {
    pub content: String,
    pub classes: Option<String>,
    pub id: Option<String>
}

impl P {
    pub fn new(content: &str) -> Self {
        Self { 
            classes: None,
            id: None,
            content: content.to_string() 
        }
    }
}

impl HtmlElement for P {
    fn get_tag(&self) -> &str {
        "p"
    }

    fn get_html(&self) -> String {
        format!("<{}>{}</{}>", self.get_tag(), self.content, self.get_tag())
    }

    fn get_classes(&self) -> Option<&String> {
        match &self.classes {
            Some(classes) => Some(&classes),
            None => None
        }
    }

    fn get_id(&self) -> Option<&String> {
        match &self.id {
            Some(id) => Some(&id),
            None => None
        }
    }
}

pub fn p(content: &str) -> P {
    P::new(content)
}
