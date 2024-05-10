use super::card::SearchCardComponent;

use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchItemData {
    pub command: &'static str,
    pub c_type: CommandType,
    pub c_alias: Option<&'static str>,
    pub desc: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Function,
    Constant,
}

impl CommandType {
    fn to_string(&self) -> &'static str {
        match self {
            CommandType::Function => "function",
            CommandType::Constant => "constant",
        }
    }
}

static SEARCH_ITEMS: &[SearchItemData; 4] = &[
    SearchItemData {
        command: "max_uint",
        c_type: CommandType::Constant,
        c_alias: Some("max_u256, type(uint256).max"),
        desc: "Evaluates to `type(unt256).max`",
    },
    SearchItemData {
        command: "zero_address",
        c_type: CommandType::Constant,
        c_alias: Some("address(0), addr(0), address_zero, zadd"),
        desc: "Evaluates to the zero address",
    },
    SearchItemData {
        command: "now",
        c_type: CommandType::Constant,
        c_alias: None,
        desc: "Evaluates to the current unix timestamp",
    },
    SearchItemData {
        command: "unchecked",
        c_type: CommandType::Function,
        c_alias: None,
        desc: "Enables unchecked math for any calculation performed inside its brackets.",
    },
];

pub enum Msg {
    SearchQuery(String),
    CheckForArrows((KeyboardEvent, usize)),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchMenuComponent {
    search_query: String,
    focus_ref: NodeRef,
    focus_index: Option<usize>,
}

impl Component for SearchMenuComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            search_query: "".to_string(),
            focus_ref: NodeRef::default(),
            focus_index: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SearchQuery(query) => {
                self.search_query = query.trim().to_lowercase();
                self.focus_index = None;
            }
            Msg::CheckForArrows((e, items)) => {
                if (e.meta_key() || e.ctrl_key()) && e.key() == "ArrowDown" {
                    self.focus_index = Some(items - 1);
                } else if (e.meta_key() || e.ctrl_key()) && e.key() == "ArrowUp" {
                    self.focus_index = None;
                } else {
                    match e.key().as_str() {
                        "ArrowDown" => {
                            self.focus_index = match self.focus_index {
                                Some(i) => {
                                    if i < items - 1 {
                                        Some(i + 1)
                                    } else {
                                        None
                                    }
                                }
                                None => Some(0),
                            }
                        }
                        "ArrowUp" => {
                            self.focus_index = match self.focus_index {
                                Some(i) => {
                                    if i > 0 {
                                        Some(i - 1)
                                    } else {
                                        None
                                    }
                                }
                                None => Some(items),
                            }
                        }
                        _ => return false,
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let filtered_cards: Vec<_> = SEARCH_ITEMS
            .iter()
            .filter(|item| {
                item.command.contains(&self.search_query)
                    || item.c_type.to_string().contains(&self.search_query)
                    || item
                        .c_alias
                        .map_or(false, |alias| alias.contains(&self.search_query))
            })
            .collect();
        let filtered_cards_len = filtered_cards.len();
        let command_card = |index: usize, item: &SearchItemData| {
            html! {
                <SearchCardComponent card_id={index} item={item.clone()}
                    focus_ref={ if self.focus_index.is_some() && index == self.focus_index.unwrap() { self.focus_ref.clone() } else {NodeRef::default()}}
                />
            }
        };
        let on_key_down = ctx
            .link()
            .callback(move |e: KeyboardEvent| Msg::CheckForArrows((e, filtered_cards_len)));

        html! {
            <div onkeydown={on_key_down} style="min-height: 95vh; display: flex; flex-direction: column;">
                <div style="min-height: 10vh; display: flex; flex-direction: column;"/>
                <div class="text-gray-400 bg-gray-500/90 dark:bg-gray-400/80 rounded-lg">
                    <div class="px-4 pt-4 pb-1"><div class="flex w-full py-2 px-3 dark:bg-dark-gray350 bg-gray-400 rounded-md">
                        <svg class="w-4 h-6 text-gray-100" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 512 662">
                            <g transform="translate(0, 75)"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></g>
                        </svg>
                        <input type="text" class="pl-2.5 w-full text-gray-100 font-semibold placeholder:text-gray-100/60 dark:bg-dark-gray350 bg-gray-400 outline-none"
                            placeholder="Search commands..."
                            ref={if self.focus_index.is_none() {self.focus_ref.clone()} else {NodeRef::default()}}
                            oninput={ctx.link().callback(move |e: InputEvent| {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                Msg::SearchQuery(input.value())}
                            )}
                        />
                    </div></div>
                    <div class="pt-2">
                        <ul class="py-1"> { for filtered_cards.into_iter().enumerate().map(|(index, card)| command_card(index, card)) } </ul>
                    </div>
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(element) = self.focus_ref.cast::<HtmlElement>() {
            let _ = element.focus();
        }
    }
}
