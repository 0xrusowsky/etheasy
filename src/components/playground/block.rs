use super::{
    clipboard::ClipboardComponent,
    types::{BlockInput, BlockState},
};
use crate::parser::types::result::ParseResult;
use crate::parser::{self, utils};
use crate::{components::json::JsonComponent, parser::utils::count_chars};

use gloo_console::log;
use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, Component};

pub enum Msg {
    Blur,
    InputChanged(BlockInput),
    CheckForEnter(KeyboardEvent),
}

#[derive(Properties, PartialEq)]
pub struct BlockProps {
    pub on_enter: Callback<()>,
    pub on_result: Callback<ParseResult>,
    pub on_export: Callback<BlockInput>,
    pub on_import: Callback<()>,
    pub textarea_ref: NodeRef,
    // app state
    pub toggle: bool,
    pub export: bool,
    pub blocks: Vec<BlockState>,
    pub import: Option<BlockInput>,
    pub block_index: usize,
    pub label_change: bool,
}

#[derive(Debug)]
pub struct BlockComponent {
    min_height: i32,
    initialized: bool,
    input: BlockInput,
    output: ParseResult,
}

impl BlockComponent {
    fn parse_input(&mut self, blocks: &Vec<BlockState>) {
        let s = self.input.get_value().replace("\n", "");
        self.output = parser::parse(&s, blocks);
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

    fn create(ctx: &Context<Self>) -> Self {
        match &ctx.props().import {
            Some(input) => Self {
                min_height: input.height(),
                initialized: true,
                input: input.clone(),
                output: ctx
                    .props()
                    .blocks
                    .get(ctx.props().block_index)
                    .unwrap()
                    .get_result(),
            },
            None => Self {
                min_height: 110,
                initialized: false,
                input: BlockInput::default(),
                output: ParseResult::NAN,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(input) => {
                self.initialized = true;
                self.input = input;
                let lb = count_chars(&self.input.get_value(), "\n");
                self.parse_input(&ctx.props().blocks);
                ctx.props().on_result.emit(self.output.clone());
                // Manually resize textarea to avoid scrollbars
                if let Some(textarea) = ctx.props().textarea_ref.cast::<HtmlTextAreaElement>() {
                    match textarea.remove_attribute("style") {
                        Ok(_) => self.min_height = textarea.client_height(),
                        Err(_) => log!("Failed to remove style attribute"),
                    }
                    if textarea.scroll_height() > textarea.client_height() {
                        textarea
                            .set_attribute(
                                "style",
                                &format!(
                                    "height: {}px",
                                    std::cmp::max(110, textarea.scroll_height())
                                ),
                            )
                            .expect("Failed to set style");
                    } else if lb > 0 {
                        textarea
                            .set_attribute(
                                "style",
                                &format!("height: {}px", textarea.scroll_height()),
                            )
                            .expect("Failed to set style");
                    }
                }
                true
            }
            Msg::CheckForEnter(e) => {
                let has_content = self.input.len() > 0
                    && self.input.len() != utils::count_chars(self.input.get_value(), "\n");
                if e.key() == "Enter" && !e.shift_key() && has_content {
                    e.prevent_default();
                    ctx.props().on_enter.emit(());
                }
                false
            }
            Msg::Blur => {
                if let Some(textarea) = ctx.props().textarea_ref.cast::<HtmlTextAreaElement>() {
                    // Ensure first block is always expanded despite not focused
                    if self.input.get_value() == ""
                        && ctx.props().block_index == ctx.props().blocks.len()
                    {
                        match textarea.remove_attribute("style") {
                            Ok(_) => (),
                            Err(_) => log!("Failed to remove style attribute"),
                        }
                        textarea
                            .set_attribute("style", "height: 110px")
                            .expect("Failed to set style");
                    }
                }
                ctx.props().on_result.emit(self.output.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_key_down = ctx.link().callback(Msg::CheckForEnter);
        let on_blur = ctx.link().callback(move |_: FocusEvent| Msg::Blur);
        let on_text_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::InputChanged(BlockInput::new(input.value(), input.scroll_height() + 10))
        });

        html! {
            <div class="w-full text-gray-500 grid h-full grid-cols-3 p-4 pb-0 border-b-2 border-gray-100/25 dark:border-b-4 dark:border-dark-primary">
                <div class="peer/input col-span-1 pr-2">
                    <p class="mt-0 text-gray-400 pb-1">{ "input:" }</p>
                    <textarea ref={ctx.props().textarea_ref.clone()}
                        class="w-full h-full font-mono focus-within:text-gray-50 placeholder-gray-600 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0"
                        oninput={on_text_input}
                        onkeydown={on_key_down}
                        onblur={on_blur}
                        data-gramm="false"
                        placeholder={
                            // Only first block should have a placeholder
                            if ctx.props().block_index == ctx.props().blocks.len() - 1 {
                                "address(0)\nunchecked(max_uint + 1)\nnow - unix(2023,12,31)\n1 ether to gwei"
                            } else {
                                ""
                            }
                        }>
                    </textarea>
                </div>
                if self.is_json() {
                <div class="col-span-2 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                    <p class="pt-0 text-gray-400">{ "json:" }</p>
                    <div class="w-full text-left"><JsonComponent
                         value={self.output.get_json().unwrap()}/></div>
                </div>
                }
                else if self.is_str() {
                <div class="col-span-2 overflow-x-auto peer-focus-within/input:text-emerald-400">
                    <div class="flex text-gray-400 justify-end">
                        <p class="pt-0 pr-2">{ "text:" }</p>
                        <ClipboardComponent text={self.output.to_string()} text_style={"text-gray-400 hover:text-gray-50"}/>
                    </div>
                    <div class="text-right whitespace-normal break-all"> {
                        for self.output.to_string().split('\n').into_iter().map(|v| {
                            html!{
                                <div class="w-full">{ v }</div>
                            } })
                        }
                    </div>
                </div>
                } else if ctx.props().toggle {
                    <div class="col-span-2 resize-none overflow-y-auto text-right peer-focus-within/input:text-emerald-400">
                        <div class="flex text-gray-400 justify-end">
                            <p class="pt-0 pr-2">{ "hex:" }</p>
                            <ClipboardComponent text={self.output.to_hex_string(true)} text_style={"text-gray-400 hover:text-gray-50"}/>
                        </div>
                        <div class="whitespace-normal break-all pr-2"> {
                            for self.output.to_hex_string(true).split('\n').into_iter().map(|v| {
                                html!{
                                    <div class="w-full">{ v }</div>
                                } })
                            }
                        </div>
                    </div>
                } else {
                    <div class="col-span-1 overflow-x-auto text-right peer-focus-within/input:text-amber-300 pl-2">
                        <div class="flex text-gray-400 justify-end">
                            <p class="pt-0 pr-2">{ "dec:" }</p>
                            <ClipboardComponent text={self.output.to_string()} text_style={"text-gray-400 hover:text-gray-50"}/>
                        </div>
                        <div class="whitespace-normal break-all pr-2"> {
                            for self.output.to_string().split('\n').into_iter().map(|v| {
                                html!{
                                    <div class="w-full ">{ v }</div>
                                } })
                            }
                        </div>
                    </div>
                    <div class="col-span-1 overflow-x-auto text-right peer-focus-within/input:text-emerald-400">
                        <div class="flex text-gray-400 justify-end">
                            <p class="pt-0 pr-2">{ "hex:" }</p>
                            <ClipboardComponent text={self.output.to_hex_string(false)} text_style={"text-gray-400 hover:text-gray-50"}/>
                        </div>
                        <div class="whitespace-normal break-all px-2"> {
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

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().label_change != old_props.label_change {
            self.parse_input(&ctx.props().blocks);
        }
        if ctx.props().export != old_props.export && ctx.props().export {
            ctx.props().on_export.emit(self.input.clone());
            return false;
        }
        if ctx.props().import != old_props.import && ctx.props().import.is_some() {
            self.input = ctx.props().import.clone().unwrap();
            self.output = ctx
                .props()
                .blocks
                .get(ctx.props().block_index)
                .unwrap()
                .get_result();
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && ctx.props().block_index == ctx.props().blocks.len() - 1 {
            let textarea_ref = ctx.props().textarea_ref.clone();
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea
                    .set_attribute("style", "height: 110px")
                    .expect("Failed to set style");
            }
        }
        if ctx.props().import.is_some() {
            let textarea_ref = ctx.props().textarea_ref.clone();
            let input_value = self.input.get_value().clone();
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea.set_value(&input_value);
            }
        }
        if ctx.props().import.is_some() && ctx.props().block_index == 0 {
            ctx.props().on_import.emit(());
        }
    }
}
