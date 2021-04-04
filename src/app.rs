use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::Footer,
    navbar::Navbar,
};
use crate::routes::{
    fix_fragment_routes,
    fun::{Fun, FunItem},
    home::Home,
    work::Work,
    AppRoute,
};

pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
    #[allow(unused)]
    link: ComponentLink<Self>,
}

pub enum Msg {
    Route(Route),
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
            link,
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
        let featured = FunItem {
            title: "Hex",
            thumbnail_url: "https://i.imgur.com/7asdDk5.png",
            description: "Hex is a two-player strategy game on a hexagonal grid. Play it on itch.io or get the game on Google Play!",
            link: "https://dolphingarlic.itch.io/hex"
        };
        let fun_items = vec![
            FunItem {
                title: "Guess the Tune!",
                thumbnail_url: "https://i.imgur.com/EoWBYV8.png",
                description: "Can you identify random tunes by reading the notes in their melody? Find out on this fun little quiz website!",
                link: "http://melody-guesser.herokuapp.com"
            },
            FunItem {
                title: "Tom StaglAIno",
                thumbnail_url: "https://i.imgur.com/dN4iEgP.png",
                description: "Can't get enough of Tom Stagliano? Well, now you can enjoy AI-generated Tom-wisdom, all from your own Discord server!",
                link: "https://github.com/dolphingarlic/tom-stagl-ai-no"
            },
            FunItem {
                title: "St0nks",
                thumbnail_url: "https://i.imgur.com/X3NEEBN.png",
                description: "Real News Headlines + Fake Financial Predictions = St0nks.",
                link: "https://st0nks.ml"
            },
            FunItem {
                title: "Joining Points",
                thumbnail_url: "https://i.imgur.com/J2qvm9G.png",
                description: "The game 'Joining Points' from IOI 2006, recreated in PyGame!",
                link: "https://github.com/dolphingarlic/joining_points"
            },
            FunItem {
                title: "Incredibowl",
                thumbnail_url: "https://i.imgur.com/hv2l32M.png",
                description: "Rate and share bowl on this fabowlous website! (May or may not be down at random times as Heroku pauses my free projects.)",
                link: "https://incredibowl.herokuapp.com"
            },
            FunItem {
                title: "Click for Cats",
                thumbnail_url: "https://i.imgur.com/j6QO3oQ.png",
                description: "My first website ever (made way back in 2017!) Don't judge pls (or do - I can't control you.)",
                link: "http://bits-and-bytes.me/click_for_cats"
            }
        ];

        html! {
            <>
                <Navbar />
                <main>
                {
                    if let Some(route) = &self.current_route {
                        match route {
                            AppRoute::Home => html!{ <Home /> },
                            AppRoute::Work => html!{ <Work /> },
                            AppRoute::Fun => html! {
                                <Fun featured=featured fun_items=fun_items />
                            },
                            AppRoute::HowItsMade => html! {"Not implemented yet :("}
                        }
                    } else {
                        // 404 when route matches no component
                        html! {
                            <div class="jumbotron centered">
                                <h1 class="display-1">{"404: Andi made a boo-boo"}</h1>
                            </div>
                        }
                    }
                }
                </main>
                <Footer />
            </>
        }
    }
}
