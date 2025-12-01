use crate::prelude::*;

pub trait Component {
    fn name(&self) -> String;
    fn template(&self) -> Html;
    fn render(&self);
}
