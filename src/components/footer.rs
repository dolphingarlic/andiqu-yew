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
            <footer class="footer">
                <div class="container">
                    <div class="row">
                        <div class="navlinks my-3 col-lg-6 col-md-12">
                            <nav class="row">
                                <a class="col" href="/"><h2>{ "HOME" }</h2></a>
                                <a class="col" href="/work"><h2>{ "WORK" }</h2></a>
                                <a class="col" href="#"><h2>{ "BLOG" }</h2></a>
                                <a class="col" href="/fun"><h2>{ "FUN" }</h2></a>
                            </nav>
                        </div>
                        <div class="social my-3 col-lg-4 col-md-8 col-sm-12">
                            <div class="row">
                                <a class="col" href="#">{ "FACEBOOK" }</a>
                                <a class="col" href="#">{ "GITHUB" }</a>
                                <a class="col" href="#">{ "SPOTIFY" }</a>
                            </div>
                            <div class="row">
                                <a class="col" href="#">{ "INSTAGRAM" }</a>
                                <a class="col" href="#">{ "YOUTUBE" }</a>
                                <a class="col" href="#">{ "EMAIL" }</a>
                            </div>
                        </div>
                        <div class="copyright my-3 col-lg-2 col-md-4 col-sm-12">
                            <div>{ "© Andi Qu 2021" }</div>
                            <div>
                                { "Built with " }
                                <a href="https://yew.rs/docs/en/">{ "Yew" }</a>
                                { " & " }
                                <a href="https://www.buymeacoffee.com/andiqu">{ "💖" }</a>
                            </div>
                        </div>
                    </div>
                </div>
            </footer>
        }
    }
}
