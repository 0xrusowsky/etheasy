use yew::prelude::*;

use super::menu::SearchItemData;

pub enum Msg {
    Focus,
    Blur,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchCardComponent {
    is_focused: bool,
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub item: SearchItemData,
    pub focus_ref: NodeRef,
    pub card_id: usize,
}

impl Component for SearchCardComponent {
    type Message = Msg;
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_focused: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Focus => {
                self.is_focused = true;
            }
            Msg::Blur => {
                self.is_focused = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = &ctx.props().item;
        let child_focus = if self.is_focused {
            "pr-2 text-emerald-400/50"
        } else {
            "pr-2 text-gray-400/60"
        };

        html! {
            <li tabindex={ctx.props().card_id.to_string()} ref={ctx.props().focus_ref.clone()}
                onblur={ctx.link().callback(|_| Msg::Blur)}
                onfocus={ctx.link().callback(|_| Msg::Focus)}
                class="text-sm px-6 py-2 border-t-2 border-gray-400 focus:text-gray-50 text-gray-400/80 focus:bg-gray-600 ring-0 outline-0"
            >
            <div class="flex">
                <p class={child_focus}>{"command:"}</p>
                <p class="text-xs font-mono font-bold" style="padding-top: 0.175rem;">{format!("{} ({:#?})", item.command, item.c_type)}</p>
                if item.c_alias.is_some() {
                    <div class="flex ml-auto pl-6">
                        <p class={child_focus}>{"aliases:"}</p>
                        <p class="text-xs font-mono font-bold" style="padding-top: 0.175rem;">{item.c_alias}</p>
                    </div>
                }
            </div>
            <div class="flex"><p class={child_focus}>{"description:"}</p><p>{item.desc}</p></div>
        </li>
        }
    }
}
