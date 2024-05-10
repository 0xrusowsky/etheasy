use yew::prelude::*;

use super::menu::SearchItemData;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchCardComponent;

#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub item: SearchItemData,
    pub focus_ref: NodeRef,
    pub card_id: usize,
}

impl Component for SearchCardComponent {
    type Message = ();
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = &ctx.props().item;
        html! {
            <li tabindex={ctx.props().card_id.to_string()} ref={ctx.props().focus_ref.clone()} class="px-6 py-2 border-t-2 border-gray-400 text-gray-400/80 focus:bg-gray-600 focus:border-emerald-400">
            <div class="flex">
                <p class="pr-2 text-gray-400/60">{"command:"}</p>
                <p class="text-sm font-mono font-bold" style="padding-top: 0.175rem;">{format!("{} ({:#?})", item.command, item.c_type)}</p>
                if item.c_alias.is_some() {
                    <div class="flex ml-auto pl-6">
                        <p class="pr-2 text-gray-400/60">{"aliases:"}</p>
                        <p class="text-sm font-mono font-bold" style="padding-top: 0.175rem;">{item.c_alias}</p>
                    </div>
                }
            </div>
            <div class="flex"><p class="pr-2 text-gray-400/60">{"description:"}</p><p>{item.desc}</p></div>
        </li>
        }
    }
}
