use super::{
    block::{BlockComponent, BlockState},
    label::LabelComponent,
};
use crate::parser::types::result::ParseResult;

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
    RenameBlock(usize, String),
    // FinishBlock(KeyboardEvent),
}

#[derive(Default, Debug)]
pub struct FrameComponent {
    toggle: bool,
    blocks: Vec<BlockState>,
    focus: usize,
    focus_ref: NodeRef,
    focus_on_render: bool,
    label_change: bool,
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub search_mode: bool,
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
}

impl Component for FrameComponent {
    type Message = Msg;
    type Properties = FrameProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            toggle: false,
            blocks: vec![BlockState::from_id(0)],
            focus: 0,
            focus_ref: NodeRef::default(),
            focus_on_render: true,
            label_change: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddBlock => {
                self.blocks.push(BlockState::from_id(self.num_blocks()));
                self.focus = self.last_block();
                self.focus_on_render = true;
            }
            Msg::FocusBlock => {
                self.focus = self.last_block();
                self.focus_on_render = true;
            }
            Msg::Toggle => {
                self.toggle = !self.is_toggled();
                self.focus_on_render = true;
            }
            Msg::UpdateBlock(index, result) => {
                if let Some(block) = self.blocks.get_mut(index) {
                    block.update_result(result);
                }
                self.focus = index;
                self.focus_on_render = false;
                self.label_change = !self.label_change;
            }
            Msg::RenameBlock(index, id) => {
                if let Some(block) = self.blocks.get_mut(index) {
                    block.update_id(id.clone());
                    self.focus_on_render = false;
                    self.label_change = !self.label_change;
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let hide = if ctx.props().search_mode {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class={hide}>
            <div style="min-height: 95vh; display: flex; flex-direction: column;">
                <div style="min-height: 3vh; display: flex; flex-direction: column;"/>
                <div class="font-mono text-xs md:text-sm">
                    // full evm word (bytes32) checkbox
                    <div class="form-control text-gray-600 dark:text-gray-400 pt-10 pb-2 flex justify-end">
                        <label class="cursor-pointer label">
                        <span>{"Display full EVM words "}</span>
                        <input type="checkbox" checked={self.is_toggled()} class="checkbox checkbox-accent accent-emerald-400 hover:scale-105" onclick={ ctx.link().callback(|_| Msg::Toggle) }/>
                        </label>
                    </div>
                    // code playground
                    <div class="subpixel-antialiased bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl">
                    {
                        for (0..self.num_blocks()).rev().map(|index| {
                            html! {
                                <div class="flex">
                                <LabelComponent block_index={index}
                                    input_ref={
                                        if self.focus == index {self.focus_ref.clone()} else {NodeRef::default()}
                                    }
                                    on_result={ctx.link().callback(move |result: String| {
                                        Msg::RenameBlock(index, result)})
                                    }
                                    on_enter={ ctx.link().callback(move |_| Msg::FocusBlock) }
                                />
                                <BlockComponent key={index}
                                    blocks={self.blocks.clone()} block_index={index} toggle={self.is_toggled()} label_change={self.label_change}
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
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render && self.focus_on_render {
            if let Some(textarea) = self.focus_ref.cast::<HtmlTextAreaElement>() {
                let _ = textarea.focus();
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if !ctx.props().search_mode && old_props.search_mode {
            self.focus_on_render = true;
            return true;
        }
        false
    }
}
