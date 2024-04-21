use super::components::theme::ThemeComponent;
use super::parser::{self, types::ParseResult};
use alloy_core::primitives::U256;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::Component;

use gloo_console::*;

enum Msg {
    AddText(String),
    Toggle,
    SwitchTheme(bool),
}

#[function_component(App)]
pub fn app() -> Html {
    html! { <Frame /> }
}

struct Frame {
    dark_mode: bool,
    toggle: bool,
    input: String,
    dec: String,
    hex: String,
    total: U256,
}

impl Frame {
    fn is_toggled(&self) -> bool {
        self.toggle
    }

    fn is_dark_mode(&self) -> bool {
        self.dark_mode
    }
}

impl Frame {
    fn parse_input(&mut self) {
        let mut output_dec = "".to_string();
        let mut output_hex = "".to_string();
        let mut total = U256::from(0);
        let split = self.input.split('\n');

        for s in split {
            let p = parser::parse(s);
            match p {
                ParseResult::Value(u) => total = total.checked_add(u).unwrap(),
                _ => (),
            };

            let (dec, hex) = parser::utils::stringify(p, self.is_toggled());
            output_dec = format!("{}{}\n", output_dec, dec);
            output_hex = format!("{}{}\n", output_hex, hex);
        }

        self.total = total;
        self.dec = output_dec;
        self.hex = output_hex;
    }
}

impl Component for Frame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dark_mode: false,
            toggle: false,
            input: "".to_string(),
            dec: String::from(""),
            hex: String::from(""),
            total: U256::from(0),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddText(input) => {
                self.input = input;
                self.parse_input();
            }
            Msg::Toggle => {
                self.toggle = !self.is_toggled();
                self.parse_input();
            }
            Msg::SwitchTheme(is) => {
                self.dark_mode = is;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = "devm toolkit".to_string();
        let dark_mode = if self.is_dark_mode() { "dark" } else { "" };
        let total = if self.dec.len() == 0 {
            "".to_string()
        } else {
            format!("Total: {}", self.total)
        };
        let on_text_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::AddText(input.value())
        });

        html! {
            <div class={dark_mode}>
            <div class="min-h-screen px-3 bg-gray-100 dark:bg-dark-primary md:px-0">
            <div class="flex flex-col items-center justify-center w-full h-full space-y-8">
            <div class="w-full max-w-md md:max-w-2xl lg:max-w-4xl 2xl:max-w-6xl 4xl:max-w-8xl">
                // navbar
                <div class="flex items-center justify-between px-4 py-4 -mx-4 border-b border-gray-200 sm:mx-0 sm:px-0">
                    <h1 class="text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100">{ title }</h1>
                    <div class="flex items-center ml-6 space-x-2">
                        <ThemeComponent on_clicked={ctx.link().callback(Msg::SwitchTheme)}/>
                        <a href="https://github.com/0xrusowsky/devm-toolkit" target="_blank" class="text-gray-600 dark:text-gray-400 transition-colors duration-200 hover:scale-110 hover:text-gray-900 dark:hover:text-gray-100">
                            <span class="hidden sm:inline"></span>
                            <svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                        </a>
                    </div>
                </div>

                <div class="font-mono text-xs md:text-sm">
                    // bytes32 checkbox
                    <div class="form-control text-gray-600 dark:text-gray-400 pt-10 pb-2 flex justify-end">
                        <label class="cursor-pointer label">
                        <span>{"Display full EVM words "}</span>
                        <input type="checkbox" checked={self.is_toggled()} class="checkbox checkbox-accent accent-emerald-400 hover:scale-105" onclick={ctx.link().callback(|_| Msg::Toggle)}/>
                        </label>
                    </div>
                    // code playground
                    <div class="subpixel-antialiased text-gray-500 bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl">
                        <div class="grid h-full grid-cols-3 p-4">
                            <div class="col-span-1 pt-0 text-gray-400">
                                <p class="mt-0 pt-0">{ "input:" }</p>
                                <textarea oninput={on_text_input}
                                    class="w-full h-full min-h-[100px] font-mono text-gray-50 placeholder-gray-600 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0 pb-2"
                                        data-gramm="false"
                                        placeholder="\n1 ether to gwei\nnow - unix(2023,12,31)\naddress(0)\n0x1234 + 5678">
                                </textarea>
                            </div>
                            if self.is_toggled() {
                                <div class="col-span-2 overflow-x-auto text-right text-emerald-400 border-l border-opacity-30">
                                    <p class="pt-0 text-gray-400">{ "hex: " }</p>
                                    <div> {
                                        for self.hex.split('\n').into_iter().map(|v| {
                                            html!{
                                                <div class="w-full ">{ v }</div>
                                            } })
                                        }
                                    </div>
                                    <div class="pt-5 text-gray-400">{ total }</div>
                                </div>
                            } else {
                                    <div class="col-span-1 overflow-x-auto text-right text-amber-300 border-l border-opacity-30">
                                        <p class="pt-0 text-gray-400">{ "dec: " }</p>
                                        <div> {
                                            for self.dec.split('\n').into_iter().map(|v| {
                                                html!{
                                                    <div class="w-full ">{ v }</div>
                                                } })
                                            }
                                        </div>
                                    </div>
                                <div class="col-span-1 overflow-x-auto text-right text-emerald-400">
                                    <p class="pt-0 text-gray-400">{ "hex: " }</p>
                                    <div> {
                                        for self.hex.split('\n').into_iter().map(|v| {
                                            html!{
                                                <div class="w-full ">{ v }</div>
                                            } })
                                        }
                                </div>
                                <div class="pt-5 text-gray-400">{ total }</div>
                                </div>
                            }
                        </div>
                    </div>
                </div>
            </div>
            </div>
            </div>
            </div>
        }
    }
}
