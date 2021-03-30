// TODO: Copy Sushain's website: https://www.skc.name/

use yew::prelude::*;

pub struct Work {
    link: ComponentLink<Self>,
}

impl Component for Work {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Work { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="work">
                <div class="jumbotron">
                    <div class="container">
                        <h1 class="display-2">{"MY WORK & EXPERIENCE"}</h1>
                    </div>
                </div>
            </section>
        }
    }
}
