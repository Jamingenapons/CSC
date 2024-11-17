

#[derive(Debug, Clone, PartialEq)]
pub enum Route {
    Home,
    Settings(SettingsRoute),
    Profile,
    NotFound,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsRoute {
    General,
    Security,
    Notifications,
}


#[derive(Debug, Clone)]
pub struct Router {
    current_route: Route,
}

impl Router {
    pub fn new() -> Self {
        Router {
            current_route: Route::Home,
        }
    }

    pub fn navigate(&mut self, route: Route) {
        self.current_route = route;
    }

    pub fn current(&self) -> &Route {
        &self.current_route
    }

    pub fn is_current(&self, route: &Route) -> bool {
        self.current_route == *route
    }

    pub fn set_current(&mut self, route: Route) {
        self.current_route = route;
    }
}
