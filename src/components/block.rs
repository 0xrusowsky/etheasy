use crate::components::json::JsonComponent;
use crate::parser::types::result::ParseResult;
use crate::parser::{self, utils};

use gloo_console::log;
use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, Component};

pub enum Msg {
    Blur,
    InputChanged(TextAreaInput),
    CheckForEnter(KeyboardEvent),
}

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub on_enter: Callback<()>,
    pub textarea_ref: NodeRef,
    // app state
    pub toggle: bool,
    pub block_count: usize,
    pub block_id: usize,
}

#[derive(Debug, PartialEq)]
pub struct TextAreaInput {
    pub value: String,
    pub height: i32,
}

#[derive(Debug)]
pub struct BlockComponent {
    min_height: i32,
    initialized: bool,
    input: TextAreaInput,
    output: ParseResult,
}

impl BlockComponent {
    pub fn new() -> Self {
        Self {
            min_height: 110,
            initialized: false,
            input: TextAreaInput {
                value: "".to_string(),
                height: 0,
            },
            output: ParseResult::NAN,
        }
    }

    fn parse_input(&mut self) {
        let s = self.input.value.replace("\n", "");
        self.output = parser::parse(&s);

        if self.input.height > 136 {
            self.min_height = self.input.height;
        }
    }

    fn is_json(&self) -> bool {
        self.output.is_json()
    }

    fn is_str(&self) -> bool {
        self.output.is_str()
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
                self.initialized = true;
                self.input = input;
                self.parse_input();
                // Manually resize textarea to avoid scrollbars
                if let Some(textarea) = ctx.props().textarea_ref.cast::<HtmlTextAreaElement>() {
                    match textarea.remove_attribute("style") {
                        Ok(_) => (),
                        Err(_) => log!("Failed to remove style attribute"),
                    }
                    if textarea.scroll_height() > textarea.client_height() {
                        textarea
                            .set_attribute("style", &format!("height: {}px", self.min_height))
                            .expect("Failed to set style");
                    }
                }
                true
            }
            Msg::CheckForEnter(e) => {
                let has_content = self.input.value.len() > 0
                    && self.input.value.len() != utils::count_chars(&self.input.value, "\n");
                if e.key() == "Enter" && !e.shift_key() && has_content {
                    e.prevent_default();
                    ctx.props().on_enter.emit(());
                }
                false
            }
            Msg::Blur => {
                if let Some(textarea) = ctx.props().textarea_ref.cast::<HtmlTextAreaElement>() {
                    // Ensure first block is always expanded despite not focused
                    if self.input.value == "" && ctx.props().block_id == ctx.props().block_count {
                        match textarea.remove_attribute("style") {
                            Ok(_) => (),
                            Err(_) => log!("Failed to remove style attribute"),
                        }
                        textarea
                            .set_attribute("style", "height: 110px")
                            .expect("Failed to set style");
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_key_down = ctx.link().callback(Msg::CheckForEnter);
        let on_blur = ctx.link().callback(move |_: FocusEvent| Msg::Blur);
        let on_text_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::InputChanged(TextAreaInput {
                value: input.value(),
                height: input.scroll_height(),
            })
        });

        html! {
            <div class="grid h-full grid-cols-3 p-4 pb-0 border-b-2 border-gray-100/25 dark:border-b-4 dark:border-dark-primary">
                <div class="peer/input col-span-1 pt-0 pr-2">
                    <p class="mt-0 text-gray-400">{ "input:" }</p>
                    <textarea ref={ctx.props().textarea_ref.clone()}
                        oninput={on_text_input}
                        onkeydown={on_key_down}
                        onblur={on_blur}
                        class="w-full h-full focus:min-h-[110px] font-mono focus-within:text-gray-50 placeholder-gray-600 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0"
                        data-gramm="false"
                        placeholder={
                            // Only first block should have a placeholder
                            if ctx.props().block_id == ctx.props().block_count {
                                "1 ether to gwei\nnow - unix(2023,12,31)\naddress(0)\n0x1234 + 5678"
                            } else {
                                ""
                            }
                        }>
                    </textarea>
                </div>
                if self.is_json() {
                <div class="col-span-2 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                    <p class="pt-0 text-gray-400">{ "json: " }</p>
                    <div class="w-full text-left"><JsonComponent
                         value={self.output.get_json().unwrap()}/></div>
                </div>
                }
                else if self.is_str() {
                <div class="col-span-2 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                    <p class="pt-0 text-gray-400">{ "text: " }</p>
                    <div class="whitespace-normal break-all"> {
                        for self.output.get_string().unwrap().split('\n').into_iter().map(|v| {
                            html!{
                                <div class="w-full">{ v }</div>
                            } })
                        }
                    </div>
                </div>
                } else if ctx.props().toggle {
                    <div class="col-span-2 resize-none overflow-y-auto text-right peer-focus-within/input:text-emerald-400">
                        <p class="pt-0 text-gray-400">{ "hex: " }</p>
                        <div class="whitespace-normal break-all"> {
                            for self.output.to_hex_string(true).split('\n').into_iter().map(|v| {
                                html!{
                                    <div class="w-full">{ v }</div>
                                } })
                            }
                        </div>
                    </div>
                } else {
                    <div class="col-span-1 overflow-x-auto text-right peer-focus-within/input:text-amber-300 pl-2">
                        <p class="pt-0 text-gray-400">{ "dec: " }</p>
                        <div class="whitespace-normal break-all"> {
                            for self.output.to_string().split('\n').into_iter().map(|v| {
                                html!{
                                    <div class="w-full ">{ v }</div>
                                } })
                            }
                        </div>
                    </div>
                    <div class="col-span-1 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                        <p class="pt-0 text-gray-400">{ "hex: " }</p>
                        <div class="whitespace-normal break-all pl-1"> {
                            for self.output.to_hex_string(false).split('\n').into_iter().map(|v| {
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

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }
}
