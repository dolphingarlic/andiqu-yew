use yew::prelude::*;

use crate::components::typist::Typist;

pub struct Home {
    link: ComponentLink<Self>,
    is_spicy: bool,
}

pub enum Msg {
    Toggle,
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
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

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let greetings: Vec<&str> = vec![
            "Hello! My name is Andi.",
            "你好！ 我叫安迪。",
            "Hallo! My naam is Andi.",
            "Tere! Minu nimi on Andi.",
            "Hallo! Ich heiße Andi.",
            "¡Hola! Mi nombre es Andi.",
            "Sawubona! Igama lami ngingu-Andi.",
            "สวัสดี! ฉันชื่อ Andi.",
            "Bonjour! Je m'appelle Andi.",
            "Привет! Меня зовут анди.",
            "हैलो! मेरा नाम एंडी है.",
        ];

        let spicy_label = if self.is_spicy { "Spicy" } else { "Sweet" };

        let about_colour = if self.is_spicy { "about jumbotron orange" } else { "about jumbotron yellow" };

        let iama: Vec<&str> = if self.is_spicy {
            vec![
                " a level-42 pigeon enthusiast.",
                " a maker and a breaker.",
                " a non-competitive competitive programmer.",
                ".",
                " bullied by scary maths sometimes.",
                " awesome.",
            ]
        } else {
            vec![
                " a programmer.",
                " a musician.",
                " a problem-solver.",
                " a mathematician.",
                " an artist.",
                " an educator.",
            ]
        };

        let description = if self.is_spicy {
            html! {
                <>
                    <p>
                        {"I am a "}<strong>{"dynamic figure"}</strong>{", often seen "}<strong>{"creating fake
                        emails"}</strong>{" to get more free trials and "}<strong>{"solving abstract
                        problems"}</strong>{" crafted by "}<strong>{"Polish mathematicians"}</strong>
                        {". When I am bored, I charm cats and pigeons with my transcendental clarinet squeaks."}
                    </p>
                    <p>
                        {"I am "}<strong>{"motivated to take risks"}</strong>{". Sometimes, because I like
                        to live dangerously, I play chess with "}<strong>{"one fewer pawn"}</strong>
                        {" and solve "}<strong>{"geometry in pen"}</strong>{". Sometimes I even
                        play the Wii without the safety strap! I know
                        - I'm a real daredevil. Sometimes I'm even amazed myself
                        that I've managed to survive this long in the first
                        place! But nothing quite beats the adrenaline rush I get from "}
                        <strong>{"submitting code without compiling it locally first"}</strong>{"."}
                    </p>
                    // TODO: Paragraph about creating software (mention some spicier projects like
                    //       Tom StaglAIno and St0nks)
                </>
            }
        } else {
            html! {
                <p>{"TODO: Sweet description"}</p>
            }
        };

        html! {
            <>
                <section class="home centered jumbotron">
                    <div class="container">
                        <h1 class="display-1">{"ANDI QU - THE WEBSITE"}</h1>
                        <h2>{"Now (almost) truly a JavaScript-free zone!"}</h2>
                        <hr />
                        <p>
                            {"I like computer science, pigeons, and education,
                            but I really don't like J*vaScr*pt.
                            That's why I decided to remake "}
                            <strong>{"Andi Qu - The Website"}</strong>
                            {" with as little JS as possible!"}
                        </p>
                        <p>
                            {"I achieved this by mainly using Rust and "}
                            <a href="https://webassembly.org/">{"WebAssembly"}</a>
                            {" (but unfortunately, one cannot completely escape JS
                            in modern web development... for now.)"}
                        </p>
                    </div>
                </section>
                <section class={about_colour}>
                    <div class="container">
                        <h1 class="display-2">
                            {"ABOUT ME"}
                        </h1>
                        <input
                            type="checkbox"
                            class="spicy-toggle"
                            id="about-toggle"
                            checked=self.is_spicy
                            onclick=self.link.callback(|_| Msg::Toggle)
                        />
                        <label class="label ms-2" for="about-toggle">
                            <span class="toggle-icon">{"😈"}</span>
                            <span class="toggle-icon">{"😇"}</span>
                            <div class="ball"></div>
                            <span class="about-toggle-label ps-3">{ spicy_label }</span>
                        </label>
                        <hr />

                        { description }

                        <p>
                            {"Tl;dr - I am Andi, and I am"}
                            <strong>
                                <Typist
                                    word_list=iama
                                    type_time=500
                                    wait_time=3000
                                />
                            </strong>
                        </p>
                    </div>
                </section>

                // Greetings signature at the top of the screen
                <div class="signature">
                    <div class="triangle" />
                    <Typist
                        word_list=greetings
                        type_time=1000
                        wait_time=2000
                    />
                </div>
            </>
        }
    }
}
