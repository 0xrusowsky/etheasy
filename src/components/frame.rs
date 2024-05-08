use crate::{components::block::BlockComponent, parser::types::result::ParseResult};

use web_sys::HtmlTextAreaElement;
use yew::{prelude::*, Component};

pub enum Msg {
    // app config
    Toggle,
    // block config
    AddBlock,
    FocusBlock,
    // block state
    UpdateBlock(usize, ParseResult),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub on_view_change: Callback<()>,
}

#[derive(Debug)]
pub struct BlockState {
    id: String,
    result: ParseResult,
}

impl BlockState {
    pub fn from_id(id: usize) -> Self {
        Self {
            id: format!("block_{}", id),
            result: ParseResult::NAN,
        }
    }

    pub fn from_name(id: String) -> Self {
        Self {
            id,
            result: ParseResult::NAN,
        }
    }
}

#[derive(Default, Debug)]
pub struct FrameComponent {
    toggle: bool,
    blocks: Vec<BlockState>,
    focus: usize,
    focus_ref: NodeRef,
}

impl FrameComponent {
    fn is_toggled(&self) -> bool {
        self.toggle
    }

    fn last_block(&self) -> usize {
        self.blocks.len() - 1
    }

    fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    fn get_block(&self, id: usize) -> Option<&BlockState> {
        self.blocks.get(id)
    }
}

impl Component for FrameComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            toggle: false,
            blocks: vec![BlockState::from_id(0)],
            focus: 0,
            focus_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddBlock => {
                self.blocks.push(BlockState::from_id(self.num_blocks()));
                self.focus = self.last_block();
            }
            Msg::FocusBlock => {
                self.focus = self.last_block();
            }
            Msg::Toggle => {
                self.toggle = !self.is_toggled();
            }
            Msg::UpdateBlock(id, result) => {
                if let Some(_) = self.get_block(id) {
                    self.blocks[id].result = result;
                }
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
                    <div class="subpixel-antialiased dark:bg-dark-code rounded-md shadow-2xl">
                    {
                        for (0..self.num_blocks()).rev().map(|index| {
                            html! {
                                <div class="flex text-gray-800 dark:text-gray-200">
                                    <div class="max-sz900:hidden absolute left-0 max-w-1/12 whitespace-normal break-all">
                                        <br/>
                                        <p class="text-xs text-gray-600/50 dark:text-gray-400/50 pl-4 p-2 border-dashed border-2 border-gray-500/50 border-l-0">
                                            {&self.get_block(index).unwrap().id}
                                        </p>
                                    </div>
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
                                    on_result={ctx.link().callback(move |result| Msg::UpdateBlock(index, result))}
                                    textarea_ref={
                                        if self.focus == index {self.focus_ref.clone()} else {NodeRef::default()}
                                    }
                                /></div>
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
