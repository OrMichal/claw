use crate::models::HtmlElement;

pub struct Div {
    pub inner_html: Vec<Box<dyn HtmlElement>>,
}

impl Div {
    pub fn new<I: IntoIterator<Item = T>, T: HtmlElement + 'static>(inner_html: I) -> Self {
        Self {
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
}

pub fn div<I: IntoIterator<Item = T>, T: HtmlElement + 'static>(inner_html: I) -> Box<Div> {
    Box::new(Div::new(inner_html))
}
