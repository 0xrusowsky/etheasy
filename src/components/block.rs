use std::cmp::max;

use super::super::{
    app::ScreenSize,
    parser::{
        self,
        types::ParseResult,
        utils::{count_chars, format_size},
    },
};
use gloo_console::log;
use web_sys::{Element, HtmlTextAreaElement};
use yew::prelude::*;
use yew::Component;

pub enum Msg {
    InputChanged(TextAreaInput),
    CheckForEnter(KeyboardEvent),
}

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub on_enter: Callback<()>,
    pub textarea_ref: NodeRef,
    // app state
    pub toggle: bool,
    pub size: ScreenSize,
}

#[derive(Debug, PartialEq)]
pub struct TextAreaInput {
    pub value: String,
    pub height: i32,
}

#[derive(Debug)]
pub struct BlockComponent {
    // block state
    input: TextAreaInput,
    min_height: i32,
    dec: String,
    hex: String,
}

impl BlockComponent {
    pub fn new() -> Self {
        Self {
            input: TextAreaInput {
                value: "".to_string(),
                height: 0,
            },
            min_height: 110,
            dec: String::from(""),
            hex: String::from(""),
        }
    }

    fn parse_input(&mut self, full: bool, size: ScreenSize) {
        let mut output_dec = "".to_string();
        let mut output_hex = "".to_string();
        let split = self.input.value.split('\n');

        for s in split {
            let p = parser::parse(s);
            let (dec, hex) = parser::utils::stringify(p, full, size);
            output_dec = format!("{}{}\n", output_dec, dec);
            output_hex = format!("{}{}\n", output_hex, hex);
        }

        let output_lb: i32 = max(
            count_chars(&output_dec, "\n"),
            count_chars(&output_hex, "\n"),
        )
        .try_into()
        .unwrap();

        if self.input.height > 136 {
            match size {
                ScreenSize::XS => {
                    let lb = self.input.height / 16;
                    self.min_height = max(110, lb * 16);
                }
                _ => {
                    // log!(
                    //     "self.input.height",
                    //     self.input.height,
                    //     "min_height",
                    //     self.input.height - 60
                    // );
                    // let adj = (self.input.height - 136) / 20 - 1;
                    // log!("adj", adj, "min_height", self.input.height - adj * 20);
                    // self.min_height = self.input.height - adj;
                    self.min_height = self.input.height;
                }
            }
        }
        self.dec = output_dec;
        self.hex = output_hex;
    }
}

impl Component for BlockComponent {
    type Message = Msg;
    type Properties = BlockProps;

    fn create(_ctx: &Context<Self>) -> Self {
        BlockComponent::new()
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(input) => {
                self.input = input;
                self.parse_input(ctx.props().toggle, ctx.props().size);
                // Manually resize textarea to avoid scrollbars
                if let Some(textarea) = ctx.props().textarea_ref.cast::<HtmlTextAreaElement>() {
                    match textarea.remove_attribute("style") {
                        Ok(_) => (),
                        Err(_) => log!("Failed to remove style attribute"),
                    }
                    log!(
                        "scroll_height",
                        textarea.scroll_height(),
                        "client_height",
                        textarea.client_height()
                    );
                    if textarea.scroll_height() > textarea.client_height() {
                        textarea
                            .set_attribute("style", &format!("height: {}px", self.min_height))
                            .expect("Failed to set style");
                    }
                    log!("textarea.style", textarea.get_attribute("style"));
                }
                true
            }
            Msg::CheckForEnter(e) => {
                let has_content = self.input.value.len() > 0
                    && self.input.value.len() != count_chars(&self.input.value, "\n");
                if e.key() == "Enter" && !e.shift_key() && has_content {
                    e.prevent_default();
                    ctx.props().on_enter.emit(());
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_text_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::InputChanged(TextAreaInput {
                value: input.value(),
                height: input.scroll_height(),
            })
        });
        let on_key_down = ctx.link().callback(Msg::CheckForEnter);
        html! {
            <div class="grid h-full grid-cols-3 p-4 pb-0 border-b-2 border-gray-100/25 dark:border-b-4 dark:border-dark-primary">
                <div class="peer/input col-span-1 pt-0 pr-2">
                    <p class="mt-0 text-gray-400">{ "input:" }</p>
                    <textarea ref={ctx.props().textarea_ref.clone()}
                        oninput={on_text_input}
                        onkeydown={on_key_down}
                        class="w-full h-full focus:min-h-[110px] font-mono focus-within:text-gray-50 placeholder-gray-600 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0"
                        data-gramm="false"
                        placeholder="\n1 ether to gwei\nnow - unix(2023,12,31)\naddress(0)\n0x1234 + 5678">
                    </textarea>
                </div>
            if ctx.props().toggle {
                <div class="col-span-2 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                    <p class="pt-0 text-gray-400">{ "hex: " }</p>
                    <div> {
                        for self.hex.split('\n').into_iter().map(|v| {
                            html!{
                                <div class="w-full ">{ v }</div>
                            } })
                        }
                    </div>
                </div>
            } else {
                    <div class="col-span-1 overflow-x-auto text-right peer-focus-within/input:text-amber-300 pl-2">
                        <p class="pt-0 text-gray-400">{ "dec: " }</p>
                        <div> {
                            for self.dec.split('\n').into_iter().map(|v| {
                                html!{
                                    <div class="w-full ">{ v }</div>
                                } })
                            }
                        </div>
                    </div>
                <div class="col-span-1 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                    <p class="pt-0 text-gray-400">{ "hex: " }</p>
                    <div> {
                        for self.hex.split('\n').into_iter().map(|v| {
                            html!{
                                <div class="w-full ">{ v }</div>
                            } })
                        }
                </div>
                </div>
            }
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        self.parse_input(ctx.props().toggle, ctx.props().size);
        true
    }
}
