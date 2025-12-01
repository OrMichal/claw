use crate::models::Component;

pub struct Route {
    pub path: String,
    pub children: Box<Route>,
    pub component: Box<dyn Component>
}

pub struct Router {
    current_route: String,
    routes: Vec<Route>,
}

impl Router {
    pub fn get_instance() -> Self {
        todo!()
    }
}
