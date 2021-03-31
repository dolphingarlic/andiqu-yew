// TODO: Copy Sushain's website: https://www.skc.name/

use yew::prelude::*;

pub struct Work {
    link: ComponentLink<Self>,
}

impl Component for Work {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Work { link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="work">
                <div class="jumbotron">
                    <div class="container">
                        <h1 class="display-2">{"MY EXPERIENCE"}</h1>
                        <ul>
                            <li>
                                <details>
                                    <summary>{"Work & Education"}</summary>
                                    <h5>{"Work"}</h5>
                                    <ul>
                                        <li>
                                            <p>
                                                <strong>{"Jan 2021 - Present:"}</strong>
                                                {" Junior Developer - BSC Global"}
                                            </p>
                                        </li>
                                    </ul>
                                </details>
                            </li>
                            <li>
                                <details>
                                    <summary>{"Awards & Recognitions"}</summary>
                                </details>
                            </li>
                            <li>
                                <details>
                                    <summary>{"Projects & Publications"}</summary>
                                </details>
                            </li>
                            <li>
                                <details>
                                    <summary>{"Volunteering"}</summary>
                                </details>
                            </li>
                            <li>
                                <details>
                                    <summary>{"Skills & Languages"}</summary>
                                </details>
                            </li>
                        </ul>
                    </div>
                </div>
            </section>
        }
    }
}
