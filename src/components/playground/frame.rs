use super::{
    block::BlockComponent,
    label::LabelComponent,
    types::{BlockInput, BlockState, Notebook, NotebookBlock},
};
use crate::{
    components::playground::types::{download_notebook, load_notebook},
    parser::types::result::ParseResult,
};

use web_sys::{File, HtmlInputElement, HtmlTextAreaElement};
use yew::{prelude::*, Component};

pub enum Msg {
    // app state
    Toggle,
    Search,
    Export,
    Import,
    ImportFinished,
    LoadFile(File),
    NotebookLoaded((Vec<BlockState>, Vec<BlockInput>)),
    // block state
    AddBlock,
    FocusBlock,
    UpdateBlock(usize, ParseResult),
    RenameBlock(usize, String),
    ExportBlock(BlockInput),
}

#[derive(Default, Debug)]
pub struct FrameComponent {
    toggle: bool,
    export: bool,
    blocks: Vec<BlockState>,
    inputs: Option<Vec<BlockInput>>,
    focus: usize,
    focus_on_render: bool,
    label_change: bool,
    export_buffer: Vec<BlockInput>,
    import_ref: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct FrameProps {
    pub search_mode: bool,
    pub import_mode: bool,
    pub export_mode: bool,
    pub focus_ref: NodeRef,
    pub on_search: Callback<()>,
    pub on_import: Callback<()>,
    pub on_export: Callback<()>,
}

impl FrameComponent {
    fn is_toggled(&self) -> bool {
        self.toggle
    }

    fn do_export(&self) -> bool {
        self.export
    }

    fn should_import_input(&self, index: usize) -> Option<BlockInput> {
        let inputs = self.inputs.clone();
        match inputs {
            Some(inputs) => inputs.get(index).cloned(),
            None => None,
        }
    }

    fn should_import_state(&self, index: usize) -> Option<BlockState> {
        if self.inputs.is_some() {
            self.blocks.get(index).cloned()
        } else {
            None
        }
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
            export: false,
            blocks: vec![BlockState::from_id(0)],
            inputs: None,
            focus: 0,
            focus_on_render: true,
            label_change: false,
            export_buffer: Vec::new(),
            import_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
            Msg::Search => {
                ctx.props().on_search.emit(());
            }
            Msg::Export => {
                self.export = true;
            }
            Msg::Import => {
                if let Some(input) = self.import_ref.cast::<HtmlInputElement>() {
                    input.click();
                }
                return false;
            }
            Msg::LoadFile(file) => {
                load_notebook(
                    file,
                    ctx.link().callback(Msg::NotebookLoaded),
                    ctx.link().callback(|_| Msg::ImportFinished),
                );
                return false;
            }
            Msg::NotebookLoaded((states, inputs)) => {
                self.blocks = states;
                self.inputs = Some(inputs);
            }
            Msg::ImportFinished => {
                self.inputs = None;
                self.focus_on_render = true;
                self.focus = self.last_block();
                ctx.props().on_import.emit(());
                return false;
            }
            Msg::ExportBlock(input) => {
                self.export_buffer.push(input);
                if self.export_buffer.len() == self.blocks.len() {
                    // process buffer and download json file
                    let mut notebook: Notebook = Vec::new();
                    for (state, input) in self.blocks.iter().zip(self.export_buffer.iter()) {
                        // Do something with item1 and item2
                        notebook.push(NotebookBlock::new(input, state))
                    }
                    self.export = false;
                    download_notebook(notebook);
                }
                return false;
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let blur = if ctx.props().search_mode {
            "filter: blur(1px);" //"blur-sm"
        } else {
            ""
        };
        let on_file_change = ctx.link().callback(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().and_then(|files| files.get(0));
            match file {
                Some(file) => {
                    gloo_console::log!("Importing file:", &file.name());
                    Msg::LoadFile(file)
                }
                None => Msg::ImportFinished,
            }
        });

        html! {
            <div style="min-height: 95vh; display: flex; flex-direction: column;">
            <div style="min-height: 5vh;"/>
                <div class="font-mono text-xs md:text-sm">
                    <div class="w-full flex" style={blur}>
                        // search bar
                        <div class="justify-strart items-end pt-9"><div class="flex">
                        <button type="button" onclick={ ctx.link().callback(|_| Msg::Search) }
                                class="hidden h-7 w-1/8 lg:flex items-center text-sm text-gray-400 rounded-md ring-1 ring-gray-900/10 shadow-sm pl-2 pr-3 hover:ring-gray-400 dark:bg-dark-code bg-gray-200 hover:bg-gray-300/60 hover:text-gray-500 dark:highlight-white/5 dark:hover:bg-gray-700 dark:hover:text-gray-300 outline-gray-300 outline-offset-4">
                                <svg width="24" height="24" fill="none" aria-hidden="true" class="mr-3 flex-none"><path d="m19 19-3.5-3.5" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></path><circle cx="11" cy="11" r="6" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"></circle></svg>
                                {"Command reference"}<span class="ml-auto pl-4 pt-0.5 flex-none text-lg font-semibold">{"⌘"}</span><span class="ml-auto pl-1 pt-0.5 flex-none text-xs font-semibold">{"K"}</span>
                            </button>
                            <div class="w-2"/>
                            <button type="button" onclick={ctx.link().callback(|_| Msg::Import)}
                                class="hidden h-7 w-1/8 md:flex items-center text-sm text-gray-400 rounded-md ring-1 ring-gray-900/10 shadow-sm px-3 hover:ring-gray-400 dark:bg-dark-code bg-gray-200 hover:bg-gray-300/60 hover:text-gray-500 dark:highlight-white/5 dark:hover:bg-gray-700 dark:hover:text-gray-300 outline-gray-300 outline-offset-4">
                                {"Import"}<span class="ml-auto pl-3 pt-0.5 flex-none text-lg font-semibold">{"⌘"}</span><span class="ml-auto pl-0.5 pt-0.5 flex-none text-xs font-semibold">{"I"}</span>
                            </button>
                            <div class="w-2"/>
                            <input type="file" style="display: none;" ref={self.import_ref.clone()} oncancel={ctx.link().callback(|_| Msg::ImportFinished)} onchange={on_file_change}/>
                            <button type="button" onclick={ctx.link().callback(|_| Msg::Export)}
                                class="hidden h-7 w-1/8 md:flex items-center text-sm text-gray-400 rounded-md ring-1 ring-gray-900/10 shadow-sm px-3 hover:ring-gray-400 dark:bg-dark-code bg-gray-200 hover:bg-gray-300/60 hover:text-gray-500 dark:highlight-white/5 dark:hover:bg-gray-700 dark:hover:text-gray-300 outline-gray-300 outline-offset-4">
                                {"Export"}<span class="ml-auto pl-3 pt-0.5 flex-none text-lg font-semibold">{"⌘"}</span><span class="ml-auto pl-1 pt-0.5 flex-none text-xs font-semibold">{"E"}</span>
                            </button>
                        </div></div>
                        <div class="flex-grow"/>
                        // full evm word (bytes32) checkbox
                        <div class="form-control text-gray-600 dark:text-gray-400 pt-10 pb-3 flex justify-end">
                            <label class="cursor-pointer label">
                            <span>{"Display full EVM words "}</span>
                            <input type="checkbox" checked={self.is_toggled()} class="checkbox checkbox-accent accent-emerald-400 hover:scale-105" onclick={ ctx.link().callback(|_| Msg::Toggle) }/>
                            </label>
                        </div>
                    </div>
                    // code playground
                    <div class="subpixel-antialiased bg-gray-900 dark:bg-dark-code rounded-md shadow-2xl dark:shadow-gray-400/5">
                    {
                        for (0..self.num_blocks()).rev().map(|index| {
                            html! {
                                <div class="flex">
                                <LabelComponent block_index={index} blur_style={blur}
                                    input_ref={NodeRef::default()}
                                    on_result={ctx.link().callback(move |result: String| {
                                        Msg::RenameBlock(index, result)})
                                    }
                                    on_enter={ ctx.link().callback(move |_| Msg::FocusBlock) }
                                    import={self.should_import_state(index)}
                                />
                                <div class="w-full" style={blur}>
                                <BlockComponent key={index}
                                    blocks={self.blocks.clone()}
                                    block_index={index}
                                    export={self.do_export()}
                                    toggle={self.is_toggled()}
                                    import={self.should_import_input(index)}
                                    label_change={self.label_change}
                                    on_enter={
                                        // only trigger AddBlock if Enter is pressed on the last block
                                        if index == self.last_block() {
                                            ctx.link().callback(|_| Msg::AddBlock)
                                        }
                                        // otherwise, move focus back to last block
                                        else { ctx.link().callback(|_| Msg::FocusBlock) }
                                    }
                                    on_result={ctx.link().callback(move |result| Msg::UpdateBlock(index, result))}
                                    on_export={ctx.link().callback(move |input| Msg::ExportBlock(input))}
                                    on_import={ctx.link().callback(|_| Msg::ImportFinished)}
                                    textarea_ref={
                                        if self.focus == index {ctx.props().focus_ref.clone()} else {NodeRef::default()}
                                    }
                                /></div></div>
                            }
                        })
                    }
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render && self.focus_on_render && !ctx.props().search_mode {
            if let Some(textarea) = ctx.props().focus_ref.cast::<HtmlTextAreaElement>() {
                let _ = textarea.focus();
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().search_mode != old_props.search_mode {
            self.focus_on_render = !ctx.props().search_mode;
            return true;
        }
        if ctx.props().import_mode {
            if let Some(input) = self.import_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        }
        if ctx.props().export_mode && !self.export {
            self.export = true;
            return true;
        }
        false
    }
}
