use yew::prelude::*;

pub struct Work {
    link: ComponentLink<Self>
}

impl Component for Work {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Work {
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
            <p>{ "TODO: Work" }</p>
        }
    }
}
