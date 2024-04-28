use crate::components::{block::BlockComponent, theme::ThemeComponent};

use gloo_console::log;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTextAreaElement};
use yew::{prelude::*, Component};

pub enum Msg {
    // app config
    Toggle,
    SwitchTheme(bool),
    CheckScreenSize,
    // block config
    AddBlock,
    FocusBlock,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ScreenSize {
    XS,  // dec-hex: 14 | hex-full: 30
    SM,  // dec-hex: 18 | hex-full: 37
    MD,  // dec-hex: 23 | hex-full: 49
    LG,  // dec-hex: 33 | hex-full: 66
    XL,  // dec-hex: 33 | hex-full: 66
    XXL, // dec-hex: 40 | hex-full: 66
}

#[function_component(App)]
pub fn app() -> Html {
    html! { <Frame /> }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_view_change: Callback<()>,
}

struct Frame {
    dark_mode: bool,
    toggle: bool,
    size: ScreenSize,
    blocks: usize,
    focus: usize,
    focus_ref: NodeRef,
}

impl Frame {
    fn is_toggled(&self) -> bool {
        self.toggle
    }

    fn is_dark_mode(&self) -> bool {
        self.dark_mode
    }

    fn screen_size(&self) -> ScreenSize {
        self.size
    }
}

impl Frame {
    fn check_screen_size(&mut self) {
        let width = window().unwrap().inner_width().unwrap().as_f64().unwrap();

        self.size = if width < 640_f64 {
            ScreenSize::XS
        } else if width < 768_f64 {
            ScreenSize::SM
        } else if width < 1024_f64 {
            ScreenSize::MD
        } else if width < 1280_f64 {
            ScreenSize::LG
        } else if width < 1536_f64 {
            ScreenSize::XL
        } else {
            ScreenSize::XXL
        };

        log!(format!("Current screen size: {:#?}", self.size));
    }

    fn last_block(&self) -> usize {
        self.blocks - 1
    }
}

impl Component for Frame {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut frame = Self {
            dark_mode: false,
            toggle: false,
            size: ScreenSize::MD,
            blocks: 1,
            focus: 0,
            focus_ref: NodeRef::default(),
        };

        frame.check_screen_size();

        let link = ctx.link().clone();
        let on_resize = Closure::wrap(Box::new(move |_event: Event| {
            link.send_message(Msg::CheckScreenSize);
        }) as Box<dyn FnMut(Event)>);
        window()
            .expect("no global `window` exists")
            .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
            .expect("failed to listen for resize");
        on_resize.forget();

        frame
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CheckScreenSize => {
                self.check_screen_size();
            }
            Msg::AddBlock => {
                self.blocks += 1;
                self.focus = self.last_block();
            }
            Msg::FocusBlock => {
                self.focus = self.last_block();
            }
            Msg::Toggle => {
                self.toggle = !self.is_toggled();
            }
            Msg::SwitchTheme(is) => {
                self.dark_mode = is;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={if self.is_dark_mode() { "dark" } else { "" }}>
            <div class="min-h-screen px-3 bg-gray-100 dark:bg-dark-primary md:px-0">
            <div class="flex flex-col items-center justify-center w-full h-full space-y-8">
            <div class="w-full max-w-md md:max-w-2xl lg:max-w-4xl 2xl:max-w-6xl 4xl:max-w-8xl">
                // navbar
                <div class="flex items-center justify-between px-4 py-4 -mx-4 border-b border-gray-200 dark:border-gray-700 sm:mx-0 sm:px-0">
                    <h1 class="text-2xl font-extrabold tracking-tight text-gray-800 dark:text-gray-200">
                    {"Ethereum"}<span class="font-normal text-gray-700 dark:text-gray-300">{" development made "}</span>
                    {"easy"}<span class="font-normal text-gray-700 dark:text-gray-300">{"."}</span>
                    </h1>
                    <div class="flex items-center ml-6 space-x-2">
                        <ThemeComponent on_clicked={ctx.link().callback(Msg::SwitchTheme)}/>
                        <a href="https://github.com/0xrusowsky/etheasy" target="_blank" class="text-gray-600 dark:text-gray-400 transition-colors duration-200 hover:scale-110 hover:text-gray-900 dark:hover:text-gray-100">
                            <span class="hidden sm:inline"></span>
                            <svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                        </a>
                    </div>
                </div>

                <div class="font-mono text-xs md:text-sm">
                    // full evm word (bytes32) checkbox
                    <div class="form-control text-gray-600 dark:text-gray-400 pt-10 pb-2 flex justify-end">
                        <label class="cursor-pointer label">
                        <span>{"Display full EVM words "}</span>
                        <input type="checkbox" checked={self.is_toggled()} class="checkbox checkbox-accent accent-emerald-400 hover:scale-105" onclick={ ctx.link().callback(|_| Msg::Toggle) }/>
                        </label>
                    </div>
                    // code playground
                    <div class="subpixel-antialiased text-gray-500 bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl">
                    {
                        for (0..self.blocks).rev().map(|index| {
                            html! {
                                <BlockComponent key={index}
                                    block_count={self.last_block()} block_id={index} toggle={self.is_toggled()} size={self.screen_size()}
                                    on_enter={
                                        // only trigger AddBlock if Enter is pressed on the last block
                                        if index == self.last_block() {
                                            ctx.link().callback(move |_| Msg::AddBlock)

                                        }
                                        // otherwise, move focus back to last block
                                        else { ctx.link().callback(move |_| Msg::FocusBlock) }
                                    }
                                    textarea_ref={
                                        if self.focus == index {self.focus_ref.clone()} else {NodeRef::default()}
                                    }
                                />
                            }
                        })
                    }
                    </div>
                </div>
            </div>
            </div>
            </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(textarea) = self.focus_ref.cast::<HtmlTextAreaElement>() {
            let _ = textarea.focus();
        }
    }
}
