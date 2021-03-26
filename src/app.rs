use yew::prelude::*;

use crate::components::{
    about::About,
    footer::Footer,
    home::Home
};

pub struct App {}

impl Component for App {
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
