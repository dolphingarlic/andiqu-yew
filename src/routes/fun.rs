use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FunItem {
    title: &'static str,
    thumbnail_url: &'static str,
    description: &'static str,
    link: &'static str,
}

pub struct Fun {
    fun_items: Vec<FunItem>,
    #[allow(unused)]
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub fun_items: Vec<FunItem>,
}

impl Component for Fun {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            fun_items: props.fun_items,
            link,
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

                <div class="container">
                    <h1 class="display-2">{"FUN STUFF :D"}</h1>

                    <h2>{"FEATURED"}</h2>
                    <p>{"TODO: Random item"}</p>
                    <hr />

                    <h2>{"ALL"}</h2>
                    <p>{"TODO: Card list"}</p>
                </div>
            </section>
        }
    }
}
