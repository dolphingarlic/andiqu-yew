use yew::prelude::*;

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
            <footer id="footer">
                <div class="container">
                    <div class="row">
                        <div id="social" class="my-3 col-md-6 col-12">
                            <div class="row">
                                <a class="col-lg-2 col-4" href="https://www.facebook.com/profile.php?id=100008200278280">
                                    <i class="fab fa-facebook my-1 me-2"></i>
                                </a>
                                <a class="col-lg-2 col-4" href="https://github.com/dolphingarlic">
                                    <i class="fab fa-github my-1 me-2"></i>
                                </a>
                                <a class="col-lg-2 col-4" href="https://open.spotify.com/user/gidc6eskpzj8pv3mo9z7os0hr?si=jVTeli1iRL6jr_wEwmZPlQ">
                                    <i class="fab fa-spotify my-1 me-2"></i>
                                </a>
                                <a class="col-lg-2 col-4" href="https://www.instagram.com/andi._.rainbows/">
                                    <i class="fab fa-instagram my-1 me-2"></i>
                                </a>
                                <a class="col-lg-2 col-4" href="https://www.youtube.com/channel/UCu2gsiVBpirUMppSSwJz2LA">
                                    <i class="fab fa-youtube my-1 me-2"></i>
                                </a>
                                <a class="col-lg-2 col-4" href="https://www.linkedin.com/in/andi-qu-48313a154/">
                                    <i class="fab fa-linkedin my-1 me-2"></i>
                                </a>
                            </div>
                        </div>
                        <div id="copyright" class="my-3 col-md-6 col-12">
                            <p>
                                {"© Andi Qu 2021 | Built with "}
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
