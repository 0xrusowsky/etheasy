use super::card::{DetailCardComponent, SearchCardComponent};
use super::docs::{SearchItemData, SEARCH_ITEMS};

use web_sys::{HtmlElement, HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

const ITEMS_PER_PAGE: usize = 10;

pub enum Msg {
    ExpandCard(usize),
    HideCard,
    SearchQuery(String),
    CheckForAction(KeyboardEvent),
    Escape,
}

#[derive(Properties, PartialEq)]
pub struct SearchProps {
    pub on_escape: Callback<()>,
    pub focus_ref: Option<NodeRef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SearchMenuComponent {
    search_query: String,
    focus_index: Option<usize>,
    expanded_index: Option<usize>,
    start_index: usize,
    end_index: usize,
    items: usize,
}

impl Component for SearchMenuComponent {
    type Message = Msg;
    type Properties = SearchProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            search_query: "".to_string(),
            focus_index: None,
            expanded_index: None,
            start_index: 0,
            end_index: ITEMS_PER_PAGE + 1,
            items: SEARCH_ITEMS.len(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SearchQuery(query) => {
                self.search_query = query.trim().to_lowercase();
                self.focus_index = None;

                let filtered_cards: Vec<_> = SEARCH_ITEMS
                    .iter()
                    .filter(|item| {
                        item.command.contains(&self.search_query)
                            || item.c_type.to_string().contains(&self.search_query)
                            || item
                                .alias
                                .map_or(false, |alias| alias.contains(&self.search_query))
                            || item.desc.to_lowercase().contains(&self.search_query)
                    })
                    .collect();

                self.start_index = 0;
                self.items = filtered_cards.len();
                self.end_index = std::cmp::min(ITEMS_PER_PAGE, self.items);
            }
            Msg::CheckForAction(e) => {
                if e.key() == "Enter" {
                    self.expanded_index = if self.expanded_index.is_some() {
                        None
                    } else {
                        Some(self.start_index + self.focus_index.unwrap())
                    };
                } else if (e.meta_key() || e.ctrl_key()) && e.key() == "ArrowDown" {
                    self.expanded_index = None;
                    self.focus_index = Some(std::cmp::min(ITEMS_PER_PAGE, self.items));
                    self.start_index = std::cmp::max(0, self.items - (ITEMS_PER_PAGE + 1));
                    self.end_index = self.items;
                } else if (e.meta_key() || e.ctrl_key()) && e.key() == "ArrowUp" {
                    self.expanded_index = None;
                    self.focus_index = Some(0);
                    self.start_index = 0;
                    self.end_index = std::cmp::min(ITEMS_PER_PAGE + 1, self.items);
                } else {
                    match e.key().as_str() {
                        "ArrowDown" => {
                            self.expanded_index = None;
                            self.start_index = match self.focus_index {
                                Some(i) => {
                                    if i == ITEMS_PER_PAGE && self.end_index < self.items {
                                        self.start_index + 1
                                    } else {
                                        self.start_index
                                    }
                                }
                                None => 0,
                            };
                            self.focus_index = match self.focus_index {
                                Some(i) => {
                                    if self.items != 0 {
                                        if i < std::cmp::min(ITEMS_PER_PAGE, self.items - 1)
                                            && self.end_index <= self.items
                                        {
                                            Some(i + 1)
                                        } else {
                                            Some(i)
                                        }
                                    } else {
                                        None
                                    }
                                }
                                None => Some(0),
                            };
                        }
                        "ArrowUp" => {
                            self.expanded_index = None;
                            self.start_index = match self.focus_index {
                                Some(i) => {
                                    if i == 0 && self.start_index != 0 {
                                        self.start_index - 1
                                    } else {
                                        self.start_index
                                    }
                                }
                                None => 0,
                            };
                            self.focus_index = match self.focus_index {
                                Some(i) => {
                                    if i > 0 {
                                        Some(i - 1)
                                    } else if self.start_index != 0 {
                                        Some(0)
                                    } else {
                                        None
                                    }
                                }
                                None => None,
                            };
                        }
                        _ => return false,
                    }
                    self.end_index =
                        self.start_index + std::cmp::min(ITEMS_PER_PAGE + 1, self.items);
                }
            }
            Msg::Escape => {
                if self.expanded_index.is_some() {
                    self.expanded_index = None;
                } else {
                    self.search_query = "".to_string();
                    self.focus_index = None;
                    self.start_index = 0;
                    self.end_index = ITEMS_PER_PAGE + 1;
                    self.items = SEARCH_ITEMS.len();
                    ctx.props().on_escape.emit(());
                }
            }
            Msg::ExpandCard(index) => {
                self.focus_index = Some(index);
                self.expanded_index = Some(self.start_index + index);
            }
            Msg::HideCard => {
                self.expanded_index = None;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().focus_ref {
            None => html! {<> </>},
            Some(focus) => {
                // Filter based on command, type, alias, and description. Results are sorted by command, then type, then alias.
                let mut filtered_cards: Vec<_> = SEARCH_ITEMS
                    .iter()
                    .filter(|item| {
                        item.command.contains(&self.search_query)
                            || item
                                .alias
                                .map_or(false, |alias| alias.contains(&self.search_query))
                    })
                    .collect();
                let mut filtered_cards_type: Vec<_> = SEARCH_ITEMS
                    .iter()
                    .filter(|item| {
                        item.c_type.to_string().contains(&self.search_query)
                            && !filtered_cards.contains(&item)
                    })
                    .collect();
                filtered_cards.append(&mut filtered_cards_type);
                let mut filtered_cards_desc: Vec<_> = SEARCH_ITEMS
                    .iter()
                    .filter(|item| {
                        item.desc.to_lowercase().contains(&self.search_query)
                            && !filtered_cards.contains(&item)
                    })
                    .collect();
                filtered_cards.append(&mut filtered_cards_desc);

                let command_card = |index: usize, item: &SearchItemData| {
                    html! {
                        <SearchCardComponent card_id={index} item={item.clone()}
                            focus_ref={ if self.focus_index.is_some() && index == self.focus_index.unwrap() { focus.clone() } else {NodeRef::default()}}
                            on_click={ctx.link().callback(move |i| Msg::ExpandCard(i))}
                        />
                    }
                };

                html! {
                    <div class="absolute left-0 z-10 w-full flex">
                    <div class="w-1/5"/>
                    <div style="display: flex; flex-direction: column; transform: translate(0, calc(10vh));"
                        onkeydown={ctx.link().callback(move |e: KeyboardEvent| Msg::CheckForAction(e))}
                        onclick={ctx.link().callback(move |_| Msg::HideCard)}
                        class="w-3/5"
                    >
                        <div style="min-height: 10vh; display: flex; flex-direction: column;" onclick={ctx.link().callback(move |_| Msg::HideCard)}/>
                        <div class="text-gray-400 bg-gray-600/90 dark:bg-gray-500/80 rounded-lg">
                            <div class="px-4 pt-4 pb-1"><div class="flex w-full py-2 px-3 bg-gray-400 rounded-md">
                                <svg class="w-4 h-6 text-gray-100" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 512 662">
                                    <g transform="translate(0, 75)"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></g>
                                </svg>
                                <input type="text" class="pl-2.5 w-full text-gray-100 font-semibold placeholder:text-gray-100/80 bg-gray-400 outline-none"
                                    placeholder="Search commands..."
                                    ref={if self.focus_index.is_none() {focus.clone()} else {NodeRef::default()}}
                                    oninput={ctx.link().callback(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        Msg::SearchQuery(input.value())}
                                    )}
                                />
                                <button type="button" onclick={ ctx.link().callback(|_| Msg::Escape) }
                                    class="bg-white/0 items-center text-sm text-gray-400 rounded-md ring-1 ring-gray-900/10 shadow-sm py-1 px-3 hover:ring-gray-300 dark:bg-gray-500 dark:highlight-white/5 dark:hover:bg-gray-700 outline-gray-300 outline-offset-4">
                                    <span class="text-gray-100 font-semibold text-center">{"esc"}</span>
                                </button>
                            </div></div>
                            <DetailCardComponent item={
                                match self.expanded_index {
                                    Some(index) => Some(filtered_cards[index].clone()),
                                    None => None,
                                }
                            }/>
                            <div class="pt-2">
                                <ul class="py-1">
                                    { for filtered_cards[self.start_index..self.end_index].into_iter().enumerate()
                                        .map(|(index, card)| {command_card(index, card)}) }
                                </ul>
                            </div>
                        </div>
                    </div>
                    </div>
                }
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        match &ctx.props().focus_ref {
            Some(focus) => {
                if let Some(element) = focus.cast::<HtmlElement>() {
                    let _ = element.focus();
                }
            }
            None => {}
        }
    }
}
