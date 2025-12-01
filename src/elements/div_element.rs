use crate::models::HtmlElement;

pub struct Div {
    pub inner_html: Vec<Box<dyn HtmlElement>>,
    pub classes: Option<String>,
    pub id: Option<String>
}

impl Div {
    pub fn new<I: IntoIterator<Item = T>, T: HtmlElement + 'static>(inner_html: I) -> Self {
        Self {
            classes: None,
            id: None,
            inner_html: inner_html
                .into_iter()
                .map(|e| Box::new(e) as Box<dyn HtmlElement>)
                .collect()
        }
    }
}

impl HtmlElement for Div {
    fn get_tag(&self) -> &str {
        "div"
    }

    fn get_html(&self) -> String {
        format!("<{}>{}</{}>", 
            self.get_tag(), 
            self.inner_html.iter()
                .map(|x| x.get_html())
                .collect::<Vec<String>>()
                .join(""), 
            self.get_tag()
        )
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

pub fn div<I: IntoIterator<Item = T>, T: HtmlElement + 'static>(inner_html: I) -> Box<Div> {
    Box::new(Div::new(inner_html))
}
