use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="container">
                    <div class="row">
                        <div class="navlinks my-3 col-6">
                            <nav class="row">
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Home
                                    classes="col-lg-3 col-md-6 col-sm-12"
                                >
                                    <h2>{"HOME"}</h2>
                                </RouterAnchor<AppRoute>>
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Work
                                    classes="col-lg-3 col-md-6 col-sm-12"
                                >
                                    <h2>{"WORK"}</h2>
                                </RouterAnchor<AppRoute>>
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Fun
                                    classes="col-lg-3 col-md-6 col-sm-12"
                                >
                                    <h2>{"FUN"}</h2>
                                </RouterAnchor<AppRoute>>
                                <a class="col-lg-3 col-md-6 col-sm-12" href="#">
                                    <h2>{"BLOG"}</h2>
                                </a>
                            </nav>
                        </div>
                        <div class="social my-3 col-lg-4 col-6">
                            <div class="row">
                                // TODO: Replace with icons
                                <a class="col-md-4 col-sm-6" href="#">{"FACEBOOK"}</a>
                                <a class="col-md-4 col-sm-6" href="#">{"GITHUB"}</a>
                                <a class="col-md-4 col-sm-6" href="#">{"SPOTIFY"}</a>
                                <a class="col-md-4 col-sm-6" href="#">{"INSTAGRAM"}</a>
                                <a class="col-md-4 col-sm-6" href="#">{"YOUTUBE"}</a>
                                <a class="col-md-4 col-sm-6" href="#">{"LINKEDIN"}</a>
                            </div>
                        </div>
                        <div class="copyright my-3 col-lg-2 col-12">
                            <div>{"© Andi Qu 2021"}</div>
                            <div>
                                {"Built with "}
                                <a href="https://yew.rs/">{"Yew"}</a>
                                {" & "}
                                <a href="https://www.buymeacoffee.com/andiqu">{"💖"}</a>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
