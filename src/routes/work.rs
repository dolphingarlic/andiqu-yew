// TODO: Copy Sushain's website: https://www.skc.name/

use yew::prelude::*;

struct WorkItem {
    title: &'static str,
    date: &'static str,
    subtitle: &'static str,
    description: &'static str,
}

fn render_item(item: &WorkItem) -> Html {
    html! {
        <li>
            <span class="fa-li"><i class="fas fa-chevron-circle-right"></i></span>
            <h4>{item.title}</h4>
            <h5><strong>{item.date}{" // "}</strong>{item.subtitle}</h5>
            <p>{item.description}</p>
        </li>
    }
}

const EMPLOYMENT: [WorkItem; 1] = [WorkItem {
    title: "Junior Developer",
    date: "Jan 2021 - Present",
    subtitle: "BSC Global",
    description: "",
}];

const EDUCATION: [WorkItem; 1] = [WorkItem {
    title: "High School",
    date: "Jan 2016 - Dec 2020",
    subtitle: "St John's College",
    description: "",
}];

const AWARDS: [WorkItem; 10] = [
    WorkItem {
        title: "IOI 2020 Bronze Medal",
        date: "Sep 2020",
        subtitle: "Singapore",
        description: "Bronze medal (scored 306.1/600) at the 2020 International Olympiad in Informatics."
    },
    WorkItem {
        title: "IMO 2020 Bronze Medal",
        date: "Sep 2020",
        subtitle: "St Petersburg, Russia",
        description: "Bronze medal (scored 17/42) at the 2020 International Mathematics Olympiad."
    },
    WorkItem {
        title: "CMC 2020 Bronze Medal",
        date: "Jul 2020",
        subtitle: "Cyberspace Mathematics Competition",
        description: "Bronze medal (scored 33/56; top-scoring African contestant) at the 2020 international Cyberspace Mathematics Competition."
    },
    WorkItem {
        title: "SAPO 2020 Gold Medal",
        date: "Sep 2020",
        subtitle: "Johannesburg, South Africa",
        description: "Gold medal (scored 256/300; overall winner) at the 2020 South-African Programming Olympiad."
    },
    WorkItem {
        title: "SAMO 2020 Gold Medal",
        date: "Sep 2020",
        subtitle: "Johannesburg, South Africa",
        description: "Gold medal (overall senior winner) at the 2020 South-African Mathematics Olympiad."
    },
    WorkItem {
        title: "IMO 2019 Honourable Mention",
        date: "Sep 2019",
        subtitle: "Bath, United Kingdom",
        description: "Honourable mention (scored 15/42) at the 2019 International Mathematics Olympiad."
    },
    WorkItem {
        title: "PAMO 2019 Silver Medal",
        date: "Apr 2019",
        subtitle: "Cape Town, South Africa",
        description: "Silver medal (6th place overall) at the 2019 Pan-African Mathematics Olympiad."
    },
    WorkItem {
        title: "SAPO 2019 Gold Medal",
        date: "Sep 2019",
        subtitle: "Cape Town, South Africa",
        description: "Gold medal (scored 672/800; overall winner) at the 2019 South-African Programming Olympiad."
    },
    WorkItem {
        title: "Google Code-In 2018 Grand Prize Winner",
        date: "Dec 2018",
        subtitle: "Google LLC",
        description: "One of 54 grand-prize winners of the 2018 Google Code-In - an annual international competition where high-schoolers get to contribute to various open-source organizations by completing tasks."
    },
    WorkItem {
        title: "SAMO 2017 Gold Medal",
        date: "Sep 2017",
        subtitle: "Johannesburg, South Africa",
        description: "Gold medal (overall junior winner) at the 2017 South-African Mathematics Olympiad."
    }
];

const PROJECTS: [WorkItem; 1] = [WorkItem {
    title: "Git the lines",
    date: "May 2020",
    subtitle: "Python | Regex | Web scraping | Discord bot",
    description: "A Discord bot that prints out the lines referenced in a code snippet link.",
}];

const VOLUNTEERING: [WorkItem; 3] = [
    WorkItem {
        title: "Core Team Member",
        date: "Aug 2020 - Present",
        subtitle: "USACO Guide",
        description: "Write editorials for the practice problems presented in the guide and co-author learning modules for the Gold and Platinum divisions."
    },
    WorkItem {
        title: "OSS Developer",
        date: "Dec 2017 - Feb 2020",
        subtitle: "Apertium",
        description: "Helped develop language pairs using English, Afrikaans, and German. Upgraded the UI for the Apertium Global PairViewer."
    },
    WorkItem {
        title: "Google Code-In 2019 Mentor",
        date: "Dec 2019 - Feb 2020",
        subtitle: "Apertium",
        description: "Mentored students working with Apertium during the 2019 Google Code-In and reviewed over 150 tasks done by them."
    }
];

pub struct Work {}

impl Component for Work {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Work {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section id="work" class="jumbotron">
                <div class="container">
                    <h1 class="display-2">{"WORK & EXPERIENCE"}</h1>
                    <details class="mb-3">
                        <summary><h2>{"Employment & Education"}</h2></summary>
                        <div>
                            <ul class="fa-ul">
                                { for EMPLOYMENT.iter().map(render_item) }
                            </ul>
                            <hr />
                            <ul class="fa-ul">
                                { for EDUCATION.iter().map(render_item) }
                            </ul>
                        </div>
                    </details>
                    <details class="mb-3">
                        <summary><h2>{"Awards & Recognitions"}</h2></summary>
                        <div>
                            <p>{"Note that this is not an exhaustive list."}</p>
                            <ul class="fa-ul">
                                { for AWARDS.iter().map(render_item) }
                            </ul>
                        </div>
                    </details>
                    <details class="mb-3">
                        <summary><h2>{"Projects & Publications"}</h2></summary>
                        <div>
                            <p>
                                {"Note that this is not an exhaustive list. See my "}
                                <a class="animated link-green" href="https://github.com/dolphingarlic">{"GitHub profile"}</a>
                                {" for more projects."}
                            </p>
                            <ul class="fa-ul">
                                { for PROJECTS.iter().map(render_item) }
                            </ul>
                        </div>
                    </details>
                    <details class="mb-3">
                        <summary><h2>{"Volunteering"}</h2></summary>
                        <div>
                            <ul class="fa-ul">
                                { for VOLUNTEERING.iter().map(render_item) }
                            </ul>
                        </div>
                    </details>
                    <details class="mb-3">
                        <summary><h2>{"Skills & Languages"}</h2></summary>
                        <div>
                            <ul class="fa-ul">
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4><strong>{"Python // "}</strong>{"4 years"}</h4>
                                    <ul class="fa-ul">
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Django // "}</strong>{"2 years"}
                                            </h5>
                                        </li>
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Discord.py // "}</strong>{"1
                                                year"}
                                            </h5>
                                        </li>
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"PyGame // "}</strong>{"1 year"}
                                            </h5>
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4>
                                        <strong>{"HTML, CSS & JS // "}</strong>{"4 years"}
                                    </h4>
                                    <ul class="fa-ul">
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"React // "}</strong>{"2 years"}
                                            </h5>
                                        </li>
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Angular // "}</strong>{"1 year"}
                                            </h5>
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4><strong>{"C++ // "}</strong>{"3 years"}</h4>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4><strong>{"C# // "}</strong>{"2 years"}</h4>
                                    <ul class="fa-ul">
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Unity Game Engine // "}</strong
                                                >{"1 year"}
                                            </h5>
                                        </li>
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong
                                                    >{"Entity Framework Core // "}</strong
                                                >{"1 year"}
                                            </h5>
                                        </li>
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Azure Functions // "}</strong>{"1
                                                year"}
                                            </h5>
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4><strong>{"Dart // "}</strong>{"1 year"}</h4>
                                    <ul class="fa-ul">
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Angular Dart // "}</strong>{"1
                                                year"}
                                            </h5>
                                        </li>
                                    </ul>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4><strong>{"Rust // "}</strong>{"1 year"}</h4>
                                    <ul class="fa-ul">
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Rocket.rs // "}</strong>{"1 year"}
                                            </h5>
                                        </li>
                                        <li>
                                            <span class="fa-li"
                                                ><i class="fas fa-chevron-right"></i
                                            ></span>
                                            <h5>
                                                <strong>{"Yew.rs // "}</strong>{"1 year"}
                                            </h5>
                                        </li>
                                    </ul>
                                </li>
                            </ul>
                            <hr />
                            <ul class="fa-ul">
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4>
                                        <strong>{"English // "}</strong>{"Native Speaker"}
                                    </h4>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4>
                                        <strong>{"Mandarin // "}</strong>{"Native Speaker"}
                                    </h4>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4>
                                        <strong>{"Afrikaans // "}</strong>{"Limited Working
                                        Proficiency"}
                                    </h4>
                                </li>
                                <li>
                                    <span class="fa-li"
                                        ><i class="fas fa-chevron-circle-right"></i
                                    ></span>
                                    <h4>
                                        <strong>{"German // "}</strong>{"Elementary
                                        Proficiency"}
                                    </h4>
                                </li>
                            </ul>
                        </div>
                    </details>
                </div>
            </section>
        }
    }
}
