use crate::components::block::BlockComponent;

use gloo_console::log;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlTextAreaElement};
use yew::{prelude::*, Component};

pub enum Msg {
    // app config
    Toggle,
    // block config
    AddBlock,
    FocusBlock,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_view_change: Callback<()>,
}

pub struct FrameComponent {
    dark_mode: bool,
    toggle: bool,
    blocks: usize,
    focus: usize,
    focus_ref: NodeRef,
}

impl FrameComponent {
    fn is_toggled(&self) -> bool {
        self.toggle
    }

    fn is_dark_mode(&self) -> bool {
        self.dark_mode
    }
}

impl FrameComponent {
    fn last_block(&self) -> usize {
        self.blocks - 1
    }
}

impl Component for FrameComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dark_mode: false,
            toggle: false,
            blocks: 1,
            focus: 0,
            focus_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div style="min-height: 95vh; display: flex; flex-direction: column;">
                <div style="min-height: 5vh; display: flex; flex-direction: column;"/>
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
                                    block_count={self.last_block()} block_id={index} toggle={self.is_toggled()}
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
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(textarea) = self.focus_ref.cast::<HtmlTextAreaElement>() {
            let _ = textarea.focus();
        }
    }
}
