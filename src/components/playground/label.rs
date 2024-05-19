use super::types::BlockState;

use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::{prelude::*, Component};

pub enum Msg {
    Blur,
    InputChanged(String),
    CheckForEnter(KeyboardEvent),
}

#[derive(Properties, PartialEq)]
pub struct LabelProps {
    pub on_result: Callback<String>,
    pub on_enter: Callback<()>,
    pub input_ref: NodeRef,
    // app state
    pub block_index: usize,
    pub import: Option<BlockState>,
}

#[derive(Debug)]
pub struct LabelComponent {
    id: String,
}

impl Component for LabelComponent {
    type Message = Msg;
    type Properties = LabelProps;

    fn create(ctx: &Context<Self>) -> Self {
        match &ctx.props().import {
            Some(_) => {
                gloo_console::log!(
                    "label",
                    ctx.props().import.clone().unwrap().get_id().to_string()
                );
                Self {
                    id: ctx.props().import.clone().unwrap().get_id().to_string(),
                }
            }
            None => Self {
                id: format!("block_{}", ctx.props().block_index),
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(input) => {
                if input.trim().len() == 0 && ctx.props().import.is_none() {
                    self.id = format!("block_{}", ctx.props().block_index);
                } else {
                    self.id = input.trim().to_lowercase().replace(" ", "_");
                }
                true
            }
            Msg::CheckForEnter(e) => {
                if e.key() == "Enter" {
                    e.prevent_default();
                    ctx.props().on_result.emit(self.id.clone());
                    ctx.props().on_enter.emit(());
                }
                false
            }
            Msg::Blur => {
                gloo_console::log!(self.id.clone());
                ctx.props().on_result.emit(self.id.clone());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let on_key_down = ctx.link().callback(Msg::CheckForEnter);
        let on_blur = ctx.link().callback(move |_: FocusEvent| Msg::Blur);
        let on_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            Msg::InputChanged(input.value())
        });
        let on_key_down = ctx
            .link()
            .callback(move |e: KeyboardEvent| Msg::CheckForEnter(e));

        html! {
            <form class="max-[900px]:hidden absolute left-0 w-1/12">
                <br/>
                <input class=" break-all text-xs w-full bg-gray-200 dark:bg-inherit dark:outline-gray-50 outline-offset-2 text-gray-600/50 placeholder:text-gray-600/50 focus:text-gray-800 focus:dark:text-gray-50 placeholder:dark:text-gray-200/50 dark:text-gray-200/40 pl-4 p-2 border-dashed border-2 dark:border-gray-200/40 border-gray-600/40 border-l-0 dark:focus:border-0"
                    placeholder={format!("block_{}", ctx.props().block_index)}
                    onkeydown={on_key_down}
                    oninput={on_input}
                    onblur={on_blur}
                    ref={ctx.props().input_ref.clone()}
                />
            </form>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        if ctx.props().import != old_props.import && ctx.props().import.is_some() {
            self.id = ctx.props().import.clone().unwrap().get_id().to_string();
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if ctx.props().import.is_some() {
            let textarea_ref = ctx.props().input_ref.clone();
            let input_value = ctx.props().import.clone().unwrap().get_id().to_string();
            if let Some(textarea) = textarea_ref.cast::<HtmlTextAreaElement>() {
                textarea.set_value(&input_value);
            }
        }
    }
}
