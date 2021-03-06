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
        let spicy_label = if self.is_spicy { "Spicy" } else { "Sweet" };

        let about_colour = if self.is_spicy {
            "jumbotron orange"
        } else {
            "jumbotron yellow"
        };

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
                        play the Wii without the safety strap! I know - I'm a real daredevil. Sometimes
                        I'm even amazed myself that I've managed to survive this long in the first
                        place! But nothing quite beats the adrenaline rush I get from "}
                        <strong>{"submitting code without compiling it locally first"}</strong>{"."}
                    </p>
                    <p>
                        {"I am also motivated to create "}<strong>{"bleeding-edge quantum distributed
                        AI-powered blockchain cloud-automated solutions"}</strong>{" because of the "}
                        <strong>{"fourth industrial revolution"}</strong>{" (or so I'm told.) Although
                        some of my software may be a bit... unorthodox, I can assure you that I am
                        simply "}<strong>{"too ahead of my time"}</strong>{". Check out "}
                        <a class="animated link-orange" href="https://github.com/dolphingarlic/tom-stagl-ai-no">
                            {"Tom StaglAIno"}
                        </a>{" and "}
                        <a class="animated link-orange" href="https://st0nks.ml">
                            {"St0nks"}
                        </a>{" for some truly "}<strong>{"next-generation technology"}</strong>{"."}
                    </p>
                </>
            }
        } else {
            html! {
                <>
                    <p>
                        {"Since I first learnt to code in grade 9, computer science has become one
                        of my greatest passions. I love CS for two main reasons. Firstly, it allows
                        me to create almost anything my heart desires and share it with the world,
                        all from a text editor. Secondly, it involves a lot of challenging technical
                        problems to solve."}
                    </p>
                    <p>
                        {"I like problem-solving because of the elegant and surprising techniques that
                        I often employ to solve some of the problems. Because of this, I compete in many
                        mathematics and informatics Olympiads such as the IMO and IOI."}
                    </p>
                    <p>
                        {"When I'm not busy doing CS or maths, I also love listening to and playing music
                        (especially classical music.) I play the piano, clarinet, and flute (and I always
                        enjoy reading about some spicy music history too!)"}
                    </p>
                    <p>
                        {"I also love sharing my knowledge with others through creating learning resources
                        and tutoring. Because of this, I contribute to educational FOSS initiatives like
                        the USACO Guide."}
                    </p>
                </>
            }
        };

        html! {
            <>
                <section id="home" class="centered jumbotron">
                    <div class="container">
                        <h1 class="display-1">{"ANDI QU - THE WEBSITE"}</h1>
                        <h2>{"Now (almost) truly a JavaScript-free zone!"}</h2>
                        <hr />
                        <p>
                            {"I like a lot of things, but one thing
                            I really dislike is J*v*Scr*pt.
                            That's why I decided to remake "}
                            <strong>{"Andi Qu - The Website"}</strong>
                            {" with as little JS as possible!"}
                        </p>
                        <p>
                            {"I achieved this by mainly using Rust and "}
                            <a class="animated link-red" href="https://webassembly.org/">{"WebAssembly"}</a>
                            {" (but unfortunately, one cannot completely escape JS
                            in modern web development... for now.)"}
                        </p>
                    </div>
                </section>
                <section id="about" class={about_colour}>
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
                            <span class="toggle-icon">{"????"}</span>
                            <span class="toggle-icon">{"????"}</span>
                            <div class="ball"></div>
                            <span class="about-toggle-label ps-3">{ spicy_label }</span>
                        </label>
                        <hr />

                        { description }

                        <p>
                            {"TL;DR - I am Andi, and I am"}
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
            </>
        }
    }
}
