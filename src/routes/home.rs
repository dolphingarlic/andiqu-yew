use yew::prelude::*;

use crate::components::typist::Typist;

pub struct Home {
    link: ComponentLink<Self>,
    is_spicy: bool
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
                <section class="home centered jumbotron">
                    <div class="container">
                        <h1 class="display-1">{ "ANDI QU - THE WEBSITE" }</h1>
                        <h2>{ "Now (almost) truly a JavaScript-free zone!" }</h2>
                        <hr />
                        <p>{ "I like computer science, pigeons, and education, but I really don't like J*vaScr*pt." }</p>
                        <p>
                            { "That's why I decided to remake "}
                            <strong>{ "Andi Qu - The Website" }</strong>
                            { " with as little JS as possible!" }
                        </p>
                        <p>
                            { "I achieved this by mainly using Rust and " }
                            <a href="https://webassembly.org/">{ "WebAssembly" }</a>
                            { " (but unfortunately, one cannot completely escape JS in modern web development... for now.)" }
                        </p>
                    </div>
                </section>
                <section class="about jumbotron">
                    <div class="about-bg" />
                    <div class="about-bg about-bg2" />
                    <div class="about-bg about-bg3" />
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

                        <p>{ "TODO" }</p>
                    </div>
                </section>

                <div class="signature">
                    <div class="triangle" />
                    <Typist
                        word_list=greetings
                        type_time=1000
                        wait_time=1000
                    />
                </div>
            </>
        }
    }
}
