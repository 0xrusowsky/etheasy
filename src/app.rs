use super::parser;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew::Component;

enum Msg {
    AddText(String),
}

#[function_component(App)]
pub fn app() -> Html {
    html! { <Frame /> }
}

struct Frame {
    dec: String,
    hex: String,
    total: f64,
}

impl Component for Frame {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dec: String::from(""),
            hex: String::from(""),
            total: 0.0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddText(input) => {
                let mut output_dec = "".to_string();
                let mut output_hex = "".to_string();
                let mut total = 0.0;
                let split = input.split('\n');

                for s in split {
                    let p = parser::parse(s);
                    if p.is_normal() {
                        total = total + p;
                    }
                    let transformed = parser::transform(p);
                    output_dec = format!("{}{}\n", output_dec, &transformed);
                    output_hex = format!("{}{}\n", output_hex, transformed);
                }

                self.total = total;
                self.dec = output_dec;
                self.hex = output_hex;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let title = "devm calculator".to_string();
        let on_text_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            Msg::AddText(input.value())
        });
        html! {
            <div class="min-h-screen px-3 bg-gray-100 md:px-0">
            <div class="flex flex-col items-center justify-center w-full h-full space-y-8">
                <div class="flex items-center justify-between w-full max-w-md md:max-w-2xl lg:max-w-4xl px-4 py-4 -mx-4 border-b border-gray-200 sm:mx-0 sm:px-0">

                    <h1 class="text-2xl font-extrabold tracking-tight text-gray-900">{ title }</h1>
                    <div class="flex items-center ml-6 space-x-6 sm:space-x-10 sm:ml-10">
                    <a href="https://github.com/abhimanyu003/qubit" target="_blank" class="text-gray-600 transition-colors duration-200 hover:text-gray-900">
                        <span class="hidden sm:inline"></span>
                        <svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                    </a>
                    </div>
                </div>

                <div class="w-full max-w-md md:max-w-2xl lg:max-w-4xl subpixel-antialiased text-gray-500 bg-gray-900 rounded-md shadow-2xl text-md">
                    <div class="grid h-full grid-cols-3 p-4">
                        <div class="col-span-1 pt-5 font-mono text-sm text-gray-600">
                            <p class="mt-0 pt-0">{ "input:" }</p>
                            <textarea oninput={on_text_input}
                                class="w-full h-full text-gray-200 placeholder-gray-500 placeholder-opacity-50 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0"
                                    style="resize: none"
                                    data-gramm="false"
                                    placeholder="\n2 + 2 + sin ( 90 )\n1 ether to gwei\nnow - unix(2023,12,31,23,59,59)">
                            </textarea>
                        </div>
                        <div class="col-span-1 font-mono overflow-x-auto text-right text-yellow-300 border-l border-opacity-30">
                            <p class="pt-5 text-sm text-gray-600">{ "dec: " }</p>
                            <div> {
                                for self.dec.split('\n').into_iter().map(|v| {
                                    html!{
                                        <div class="w-full ">{ v }</div>
                                    } })
                                }
                            </div>
                        </div>
                        <div class="col-span-1 font-mono overflow-x-auto text-right text-green-300">
                            <p class="pt-5 text-sm text-gray-600">{ "hex: " }</p>
                            <div> {
                                for self.hex.split('\n').into_iter().map(|v| {
                                    html!{
                                        <div class="w-full ">{ v }</div>
                                    } })
                                }
                            </div>
                            <div class="pt-5 text-sm text-gray-600">{ "Total: " }{ self.total }</div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        }
    }
}
