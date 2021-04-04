use yew::prelude::*;

struct FunItem {
    title: &'static str,
    thumbnail_url: &'static str,
    description: &'static str,
    link: &'static str,
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

// TODO: Use serde.rs and std::fs for the fun items instead
const FEATURED: FunItem = FunItem {
    title: "Hex",
    thumbnail_url: "https://i.imgur.com/7asdDk5.png",
    description: "Hex is a two-player strategy game on a hexagonal grid. Play it on itch.io or get the game on Google Play!",
    link: "https://dolphingarlic.itch.io/hex"
};

const FUN_ITEMS: [FunItem; 6] = [
    FunItem {
        title: "Guess the Tune!",
        thumbnail_url: "https://i.imgur.com/EoWBYV8.png",
        description: "Can you identify random tunes by reading their melodies' notes? Find out on this fun quiz website!",
        link: "http://melody-guesser.herokuapp.com"
    },
    FunItem {
        title: "Tom StaglAIno",
        thumbnail_url: "https://i.imgur.com/dN4iEgP.png",
        description: "Can't get enough of Tom Stagliano? Enjoy GPT-2-generated Tom-wisdom in your own Discord server!",
        link: "https://github.com/dolphingarlic/tom-stagl-ai-no"
    },
    FunItem {
        title: "St0nks",
        thumbnail_url: "https://i.imgur.com/X3NEEBN.png",
        description: "Real News Headlines + Fake Financial Predictions = St0nks.",
        link: "https://st0nks.ml"
    },
    FunItem {
        title: "Joining Points",
        thumbnail_url: "https://i.imgur.com/J2qvm9G.png",
        description: "The game 'Joining Points' from IOI 2006, recreated in PyGame!",
        link: "https://github.com/dolphingarlic/joining_points"
    },
    FunItem {
        title: "Incredibowl",
        thumbnail_url: "https://i.imgur.com/hv2l32M.png",
        description: "Rate and share beautiful bowls on this fabowlous website!",
        link: "https://incredibowl.herokuapp.com"
    },
    FunItem {
        title: "Click for Cats",
        thumbnail_url: "https://i.imgur.com/j6QO3oQ.png",
        description: "My first website ever (made way back in 2017!) Don't judge pls (or do - I can't control you.)",
        link: "http://bits-and-bytes.me/click_for_cats"
    }
];

pub struct Fun {}

impl Component for Fun {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Fun {}
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
                    <h1 class="display-2">{"✨ FUN STUFF ✨"}</h1>

                    <h2>{"FEATURED"}</h2>
                    <div class="row">
                        { render_item(&FEATURED) }
                    </div>
                    <hr />

                    <h2>{"ALL"}</h2>
                    <div class="row">
                        { for FUN_ITEMS.iter().map(render_item) }
                    </div>
                </div>
            </section>
        }
    }
}
