use yew::prelude::*;

use crate::components::typist::Typist;

pub struct About {
    link: ComponentLink<Self>,
    is_spicy: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for About {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            is_spicy: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.is_spicy = !self.is_spicy;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let spicy_label = if self.is_spicy {
            "Spicy"
        } else {
            "Sweet"
        };

        let iama: Vec<&str> = if self.is_spicy {
            vec![
                " a level-42 pigeon enthusiast.",
                " a maker and a breaker.",
                " a non-competitive competitive programmer.",
                ".",
                " bullied by scary maths sometimes."
            ]
        } else {
            vec![
                " a programmer.",
                " a musician.",
                " a mathematician.",
                " an artist.",
                " an educator."
            ]
        };

        html! {
            <>
                <div class="about-bg" />
                <div class="about-bg about-bg2" />
                <div class="about-bg about-bg3" />
                <div class="about jumbotron">
                    <div class="container">
                        <h1 class="display-2">
                            { "ABOUT ME" }
                        </h1>
                        <input
                            type="checkbox"
                            class="checkbox"
                            id="about-toggle"
                            checked=self.is_spicy
                            onclick=self.link.callback(|_| Msg::Toggle)
                        />
                        <label class="label ms-2" for="about-toggle">
                            <span class="toggle-icon">{ "😈" }</span>
                            <span class="toggle-icon">{ "😇" }</span>
                            <div class="ball"></div>
                            <span class="about-toggle-label ps-3">{ spicy_label }</span>
                        </label>
                        <hr />

                        <p>
                            { "I'm Andi, and I am" }
                            <Typist
                                word_list=iama
                                type_time=750
                                wait_time=1500
                            />
                        </p>
                    </div>
                </div>
            </>
        }
    }
}
