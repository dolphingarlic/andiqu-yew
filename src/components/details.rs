use yew::prelude::*;

struct Details {
    summary: &'static str,
    contents: Vec<Html>,
    link: ComponentLink<Self>
}

impl Component for Details {}
