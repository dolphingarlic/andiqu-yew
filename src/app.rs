use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::routes::{
    home::Home,
    work::Work,
    fix_fragment_routes,
    AppRoute
};

pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>
}

pub enum Msg {
    Route(Route)
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let mut route = route_service.get_route();
        fix_fragment_routes(&mut route);
        App {
            current_route: AppRoute::switch(route),
            router_agent,
            link
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(mut route) => {
                fix_fragment_routes(&mut route);
                self.current_route = AppRoute::switch(route)
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        html! {
            <>
                <main>
                {
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Home => html!{ <Home /> },
                            AppRoute::Work => html!{ <Work /> }
                        }
                    } else {
                        // 404 when route matches no component
                        html! { "404 error beep boop" }
                    }
                }
                </main>
                <Footer />
            </>
        }
    }
}
