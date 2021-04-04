use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::typist::Typist;
use crate::routes::AppRoute;

pub struct Navbar {}

impl Component for Navbar {
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
            <nav id="navbar" class="navbar navbar-expand-md navbar-dark bg-dark">
                <div class="container-fluid">
                    <RouterAnchor<AppRoute> classes="navbar-brand mb-0 h1" route=AppRoute::Home>
                        <img style="image-rendering: pixelated;" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAABY0lEQVRYR+2WzRWCMAzH2w28yAxsoEMgZwdhDAbxrEvIKnhxg/rSL9vYj9Dy5IIHfLyW5Jfkn7Scbfzj1n/Xifs42te+bb9rNEiBtrnfR9fUJu3cOsXveQDBTsjHJE3DI7UmN9QCWAfiwRm/OCAAocHEU0XBzzoaDegB4ECJZZAA4Px9vLLD6+ZDOEY9iBBAYQniGQiUDiBkFjyAfI1TO35rTLG3IgC4WwbxFagWIYU4vycLIUXaKC0ac7Rep7dpFMITaVMIANRSqJE5IWYmZIR4JsCHE2diZpABL+hlGRgGBtOyN//OtATntg0xhFNzXMllAE70Nhtun6sMGJuqHAnnnhiSGqNrAJuBsZgMkpaBfAcU79gB1rkPoLtESKCxGq1xHFcd5ztAVAOmZqRLSbUGiru4/sN9DtAyUH4WqBoFNGJmxX8AjFQC94gdgJaBQB1J8wGlHjct2KAD1Ld80MLmAB+2vg4wq5XFbgAAAABJRU5ErkJggg==" alt="" width="32" height="32" />
                    </RouterAnchor<AppRoute>>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#nav-content" aria-controls="nav-content" aria-expanded="false" aria-label="Toggle navigation">
                        <span class="navbar-toggler-icon"></span>
                    </button>
                    <div id="nav-content" class="collapse navbar-collapse mb-md-0 mb-2">
                        <ul class="navbar-nav me-auto">
                            <li class="nav-item">
                                <RouterAnchor<AppRoute> classes="animated link-grey nav-link" route=AppRoute::Home>
                                    {"Home"}
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li class="nav-item">
                                <RouterAnchor<AppRoute> classes="animated link-grey nav-link" route=AppRoute::Work>
                                    {"Work"}
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li class="nav-item">
                                <RouterAnchor<AppRoute> classes="animated link-grey nav-link" route=AppRoute::Fun>
                                    {"Fun"}
                                </RouterAnchor<AppRoute>>
                            </li>
                            <li class="nav-item">
                                <RouterAnchor<AppRoute> classes="animated link-grey nav-link" route=AppRoute::HowItsMade>
                                    {"How it's made"}
                                </RouterAnchor<AppRoute>>
                            </li>
                        </ul>

                        <hr class="text-white" />

                        <span class="navbar-text">
                            <Typist
                                word_list=greetings
                                type_time=1000
                                wait_time=2000
                            />
                        </span>
                    </div>
                </div>
            </nav>
        }
    }
}
