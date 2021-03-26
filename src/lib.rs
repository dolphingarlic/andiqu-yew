#![recursion_limit = "1024"]

mod components;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::components::{
    about::About,
    footer::Footer,
    home::Home
};

struct Root {}

impl Component for Root {
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
            <>
                <main>
                    <Home />
                    <About />
                </main>
                <Footer />
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_to_body();
}
