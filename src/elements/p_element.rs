use crate::models::HtmlElement;

pub struct P {
    pub content: String
}

impl P {
    pub fn new(content: &str) -> Self {
        Self { 
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
}

pub fn p(content: &str) -> P {
    P {
        content: content.to_string()
    }
}
