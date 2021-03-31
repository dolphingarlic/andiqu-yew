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
                                    <h1>{"HOME"}</h1>
                                </RouterAnchor<AppRoute>>
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Work
                                    classes="col-lg-3 col-md-6 col-sm-12"
                                >
                                    <h1>{"WORK"}</h1>
                                </RouterAnchor<AppRoute>>
                                <RouterAnchor<AppRoute>
                                    route=AppRoute::Fun
                                    classes="col-lg-3 col-md-6 col-sm-12"
                                >
                                    <h1>{"FUN"}</h1>
                                </RouterAnchor<AppRoute>>
                                <a class="col-lg-3 col-md-6 col-sm-12" href="#">
                                    <h1>{"BLOG"}</h1>
                                </a>
                            </nav>
                        </div>
                        <div class="social my-3 col-lg-3 col-6">
                            <div class="row">
                                <a class="col-md-4 col-6" href="https://www.facebook.com/profile.php?id=100008200278280">
                                    <i class="fab fa-facebook"></i>
                                </a>
                                <a class="col-md-4 col-6" href="https://github.com/dolphingarlic">
                                    <i class="fab fa-github"></i>
                                </a>
                                <a class="col-md-4 col-6" href="https://open.spotify.com/user/gidc6eskpzj8pv3mo9z7os0hr?si=jVTeli1iRL6jr_wEwmZPlQ">
                                    <i class="fab fa-spotify"></i>
                                </a>
                                <a class="col-md-4 col-6" href="https://www.instagram.com/andi._.rainbows/">
                                    <i class="fab fa-instagram"></i>
                                </a>
                                <a class="col-md-4 col-6" href="https://www.youtube.com/channel/UCu2gsiVBpirUMppSSwJz2LA">
                                    <i class="fab fa-youtube"></i>
                                </a>
                                <a class="col-md-4 col-6" href="https://www.linkedin.com/in/andi-qu-48313a154/">
                                    <i class="fab fa-linkedin"></i>
                                </a>
                            </div>
                        </div>
                        <div class="copyright my-3 col-lg-3 col-12">
                            <p>{"© Andi Qu 2021"}</p>
                            <p>
                                {"Built with "}
                                <a href="https://yew.rs/">{"Yew"}</a>
                                {" & "}
                                <a href="https://www.buymeacoffee.com/andiqu">{"💖"}</a>
                            </p>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
