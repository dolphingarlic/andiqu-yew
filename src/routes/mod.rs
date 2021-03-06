pub mod fun;
pub mod home;
pub mod howitsmade;
pub mod work;

use yew_router::prelude::*;

// App routes
#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "#/work"]
    Work,
    #[to = "#/fun"]
    Fun,
    #[to = "#/how-its-made"]
    HowItsMade,
    #[to = "#/"]
    Home,
}

// Fix fragment handling problem for yew_router
pub fn fix_fragment_routes(route: &mut Route) {
    let r = route.route.as_str();
    if let Some(index) = r.find('#') {
        route.route = r[index..].to_string();
    } else {
        route.route = "#/".to_string();
    }
}
