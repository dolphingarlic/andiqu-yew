use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct FunItem {
    pub title: &'static str,
    pub thumbnail_url: &'static str,
    pub description: &'static str,
    pub link: &'static str,
}

pub struct Fun {
    featured: FunItem,
    fun_items: Vec<FunItem>,
    #[allow(unused)]
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub featured: FunItem,
    #[prop_or_default]
    pub fun_items: Vec<FunItem>,
}

fn render_item(item: &FunItem) -> Html {
    html! {
        <div class="col-lg-6 col-12 mb-3">
            <div class="card">
                <img src={item.thumbnail_url} height="100px" width="100px" class="my-3 ms-3" />
                <div class="card-body">
                    <h4 class="card-title">
                        <a class="animated link-blue" href={item.link}>
                            {item.title}
                        </a>
                    </h4>
                    <p class="card-text">{item.description}</p>
                </div>
            </div>
        </div>
    }
}

impl Component for Fun {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // TODO: Use serde.rs and std::fs for the fun items instead
        Self {
            featured: props.featured,
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
            <section id="fun" class="jumbotron">
                // Sliding background effect :D
                <div class="fun-bg" />
                <div class="fun-bg fun-bg2" />
                <div class="fun-bg fun-bg3" />

                <div class="container">
                    <h1 class="display-2">{"FUN STUFF :D"}</h1>

                    <h2>{"FEATURED"}</h2>
                    <div class="row">
                        { render_item(&self.featured) }
                    </div>
                    <hr />

                    <h2>{"ALL"}</h2>
                    <div class="row">
                        { for self.fun_items.iter().map(render_item) }
                    </div>
                </div>
            </section>
        }
    }
}
