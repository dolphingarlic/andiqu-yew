use yew::prelude::*;

use crate::components::typist::Typist;

pub struct Home {}

impl Component for Home {
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

        html! {
            <>
                <div class="home centered jumbotron">
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
                </div>
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
