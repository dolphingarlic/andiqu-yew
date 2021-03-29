use yew::prelude::*;

pub struct Fun {
    link: ComponentLink<Self>
}

impl Component for Fun {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="fun jumbotron">
                // Sliding background effect :D
                <div class="fun-bg" />
                <div class="fun-bg fun-bg2" />
                <div class="fun-bg fun-bg3" />

                <h1 class="display-2">{ "FUN STUFF :D" }</h1>

                <h2>{ "(RANDOMLY) FEATURED" }</h2>
                <p>{ "TODO: Random item" }</p>
                <hr />

                <h2>{ "ALL" }</h2>
                <p>{ "TODO: Card list" }</p>
            </section>
        }
    }
}
