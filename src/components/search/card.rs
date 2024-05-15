use yew::prelude::*;

use super::docs::SearchItemData;

pub enum Msg {
    Expand,
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
    pub on_click: Callback<usize>,
}

impl Component for SearchCardComponent {
    type Message = Msg;
    type Properties = CardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_focused: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Focus => {
                self.is_focused = true;
            }
            Msg::Blur => {
                self.is_focused = false;
            }
            Msg::Expand => {
                self.is_focused = false;
                ctx.props().on_click.emit(ctx.props().card_id);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = &ctx.props().item;

        html! {
            <li tabindex={ctx.props().card_id.to_string()} ref={ctx.props().focus_ref.clone()}
                onblur={ctx.link().callback(|_| Msg::Blur)}
                onfocus={ctx.link().callback(|_| Msg::Focus)}
                onclick={ctx.link().callback(|_| Msg::Expand)}
                class="text-sm px-6 py-2 border-t-2 border-gray-400 text-gray-200/60 focus:bg-gray-700/50 focus:bg-gray-700/50 hover:bg-gray-700/50 hover:bg-gray-700/50 ring-0 outline-0"
            >
            <div class="flex">
                <p class="pr-2 dark:text-gray-300/50 text-gray-200/50">{"command:"}</p>
                <p class="text-xs font-mono font-bold" style="padding-top: 0.175rem;">{format!("{} ({:#?})", item.command, item.c_type)}</p>
                if item.alias.is_some() {
                    <div class="flex ml-auto pl-6">
                        <p class="pr-2 dark:text-gray-300/50 text-gray-200/50">{"aliases:"}</p>
                        <p class="text-xs font-mono font-bold" style="padding-top: 0.175rem;">{item.alias}</p>
                    </div>
                }
            </div>
            <div class={"flex"}><p class="pr-2 dark:text-gray-300/50 text-gray-200/50">{"desc:"}</p>
            <div class="flex-col"> <p>{item.desc.split('\n').into_iter().next()}</p></div>
            </div>
        </li>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DetailCardComponent;

#[derive(Properties, PartialEq)]
pub struct DetailCardProps {
    pub item: Option<SearchItemData>,
}

impl Component for DetailCardComponent {
    type Message = ();
    type Properties = DetailCardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let item = &filtered_cards[index];
        match &ctx.props().item {
            Some(item) => html! {
                <div class="fixed inset-0 flex items-center justify-center z-10">
                    <div class="text-sm text-gray-50 bg-gray-600/90 dark:bg-gray-500/90 rounded-lg p-4 w-2/3 relative" style="transform: translate(0, calc(-50%));">
                        <div class="flex">
                            <p class="pr-2 text-emerald-400/70">{"command:"}</p>
                            <p class="font-mono font-bold" style="padding-top: 0.1rem;">{format!("{} ({})", item.command, item.c_type.to_string())}</p>
                            if item.alias.is_some() {
                                <div class="flex ml-auto pl-6">
                                    <p class="pr-2 text-emerald-400/70">{"aliases:"}</p>
                                    <p class="text-xs font-mono font-bold" style="padding-top: 0.175rem;">{item.alias}</p>
                                </div>
                            }
                        </div>
                        <div class="flex pt-2">
                            <p class="pr-2 text-emerald-400/70">{"desc:"}</p>
                            <div class="flex-col">{ for item.desc.split('\n').map(|line| html! { <p>{line}</p> })}</div>
                        </div>
                        <div class="pt-2">
                            <p class="pr-2 text-emerald-400/70">{"examples:"}</p>
                            <pre class="bg-gray-800 text-white text-xs font-mono p-4 rounded-lg overflow-x-auto">
                                <code> {item.example} </code>
                            </pre>
                        </div>
                    </div>
                </div>
            },
            None => html! {<></>},
        }
    }
}
