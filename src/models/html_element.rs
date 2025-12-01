pub trait HtmlElement {
    fn get_tag(&self) -> &str;
    fn get_html(&self) -> String;
    fn get_classes(&self) -> Option<&String>;
    fn get_id(&self) -> Option<&String>;
}

pub type Html = Box<dyn HtmlElement>;

pub fn view(element: &dyn HtmlElement) -> String {
    element.get_html()
}
