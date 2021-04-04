use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct HowItsMade {}

impl Component for HowItsMade {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        HowItsMade {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="how-its-made">
                <div class="jumbotron">
                    <div class="container">
                        <h1 class="display-2">{"HOW IT'S MADE"}</h1>

                        <p>
                            {"This website is mainly powered by "}
                            <a class="animated link-purple" href="https://yew.rs"
                                >{"Yew"}</a
                            >
                            {" - a "}
                            <a class="animated link-purple" href="https://rustlang.org"
                                >{"Rust"}</a
                            >
                            {" framework for creating WebAssembly apps."}
                        </p>
                        <p>
                            {"I first heard about Yew after someone pointed out that
                            although the original "}
                            <strong>{"Andi Qu - The Website"}</strong>
                            {" didn't involve my writing any JS, it still compiled to JS and
                            was thus not really JS-free at all."}
                        </p>
                        <p>
                            {"All interactive elements on this website (e.g. the greetings
                            typer in the navbar) were made with Rust - no JS at all! Even
                            the routing was done using Yew's "}
                            <a
                                class="animated link-purple"
                                href="https://docs.rs/yew-router/0.14.0/yew_router"
                            >
                                <code>{"yew_router"}</code>
                            </a>
                            {" extension."}
                        </p>
                        <p>
                            {"I used "}
                            <a class="animated link-purple" href="https://getbootstrap.com"
                                >{"Bootstrap 5"}</a
                            >
                            {" to create the responsive UI, and the cool backgrounds are
                            mainly from "}
                            <a
                                class="animated link-purple"
                                href="https://www.svgbackgrounds.com"
                                >{"SVGBackgrounds.com"}</a
                            >
                            {". I think these are great free resources, and I encourage you
                            to try them out if you want to create your own website too! (I
                            made the animated background on the "}
                            <RouterAnchor<AppRoute>
                                classes="animated link-purple" route=AppRoute::Fun> {"Fun
                                page"} </RouterAnchor<AppRoute
                            >> {" manually, so you'll have to look through my code to figure
                            out how it works.)"}
                        </p>
                        <p>
                            {"I host this website on "}
                            <a class="animated link-purple" href="https://netlify.com"
                                >{"Netlify"}</a
                            >
                            {" - a nice free static-website hosting service."}
                        </p>
                        <p>
                            {"If you want to check out the actual Rust code powering this
                            website, feel free to check out its "}
                            <a
                                class="animated link-purple"
                                href="https://github.com/dolphingarlic/andiqu-yew"
                                >{"GitHub repository"}</a
                            >
                            {" (and maybe follow me on GitHub while you're at it 😉.)"}
                        </p>
                    </div>
                </div>
            </section>
        }
    }
}
