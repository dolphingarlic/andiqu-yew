use yew::prelude::*;
use yew::services::timeout::{TimeoutService, TimeoutTask};

use std::str::Chars;
use std::time::Duration;

pub struct Typist {
    word_list: Vec<&'static str>,
    type_time: u64,
    wait_time: u64,
    link: ComponentLink<Self>,
    word_idx: usize,
    curr_chars: Chars<'static>,
    curr_speed: u64,
    type_fwd: bool,
    curr_string: String,
    timeout_handle: Option<TimeoutTask>,
}

pub enum Msg {
    Type,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub word_list: Vec<&'static str>,
    pub type_time: u64,
    pub wait_time: u64,
}

impl Component for Typist {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            word_list: props.word_list.clone(),
            type_time: props.type_time,
            wait_time: props.wait_time,
            link,
            word_idx: 0,
            curr_chars: props.word_list[0].chars(),
            curr_speed: props.type_time / props.word_list[0].len() as u64,
            type_fwd: true,
            curr_string: String::new(),
            timeout_handle: None,
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        if !(self.timeout_handle.is_some()) {
            self.timeout_handle = Some(TimeoutService::spawn(
                Duration::from_millis(self.type_time),
                self.link.callback(|_| Msg::Type),
            ));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Type => {
                self.timeout_handle = Some(if self.type_fwd {
                    match self.curr_chars.next() {
                        // Types the next character in the string if it exists
                        Some(c) => {
                            self.curr_string.push(c);
                            TimeoutService::spawn(
                                Duration::from_millis(self.curr_speed),
                                self.link.callback(|_| Msg::Type),
                            )
                        }
                        // Otherwise, signals to wait and then delete the string
                        None => {
                            self.type_fwd = false;
                            self.word_idx += 1;
                            if self.word_idx == self.word_list.len() {
                                self.word_idx = 0;
                            }
                            self.curr_chars = self.word_list[self.word_idx].chars();
                            TimeoutService::spawn(
                                Duration::from_millis(self.wait_time),
                                self.link.callback(|_| Msg::Type),
                            )
                        }
                    }
                } else {
                    self.curr_string.pop();
                    // After deleting the string, type the next string
                    if self.curr_string.len() == 0 {
                        self.type_fwd = true;
                        // Adjust the speed when moving on to the next word
                        self.curr_speed =
                            self.type_time / self.word_list[self.word_idx].len() as u64;
                    }
                    TimeoutService::spawn(
                        Duration::from_millis(self.curr_speed),
                        self.link.callback(|_| Msg::Type),
                    )
                })
            }
        };
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        // Set prop values
        self.word_list = props.word_list;
        self.type_time = props.type_time;
        self.wait_time = props.wait_time;
        // Reset other variables
        self.word_idx = 0;
        self.curr_chars = self.word_list[0].chars();
        self.curr_speed = self.type_time / self.word_list[0].len() as u64;
        self.type_fwd = true;
        self.curr_string = String::new();
        // Clear the timeout
        drop(&mut self.timeout_handle);
        self.timeout_handle = None;

        true
    }

    fn view(&self) -> Html {
        html! {
            <span class="typist">
                <span class="typing">{ &self.curr_string[..] }</span>
                <span class="blinking">{ "|" }</span>
            </span>
        }
    }
}
