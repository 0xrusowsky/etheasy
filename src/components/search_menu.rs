use yew::prelude::*;

struct SearchItemData {
    command: &'static str,
    c_type: CommandType,
    c_alias: Option<&'static str>,
    desc: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
enum CommandType {
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

#[function_component(SearchMenuComponent)]
pub fn search_menu_component() -> Html {
    let search_query = use_state(|| "".to_string());

    html! {
        <div class="text-gray-400 bg-gray-600 rounded-lg">
            <div class="px-4 pt-4 pb-1"><div class="flex w-full py-2 px-3 bg-gray-500 rounded-md">
                <svg class="w-4 h-6 text-gray-400" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 512 662">
                    <g transform="translate(0, 75)"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M416 208c0 45.9-14.9 88.3-40 122.7L502.6 457.4c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0L330.7 376c-34.4 25.2-76.8 40-122.7 40C93.1 416 0 322.9 0 208S93.1 0 208 0S416 93.1 416 208zM208 352a144 144 0 1 0 0-288 144 144 0 1 0 0 288z"/></g>
                </svg>
                <input type="text" class="pl-2 w-full bg-inherit"
                    placeholder="Search commands..."
                    oninput={
                        let search_query = search_query.clone();
                        move |e: InputEvent| {
                            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                                search_query.set(input.value().to_lowercase());
                            }
                        }
                    }
                />
            </div></div>
            <div class="py-2">
                <ul class="py-1">
                    {
                        for SEARCH_ITEMS.iter()
                            .filter(|item| item.command.contains(&(*search_query)) || item.c_type.to_string().contains(&(*search_query)) || (item.c_alias.is_some() && item.c_alias.unwrap().contains(&(*search_query))))
                            .map(|item| {
                                html! {
                                    <li class="px-6 py-2 border-t-2 border-gray-400 text-gray-400/80">
                                        <div class="flex">
                                            <p class="pr-2 text-gray-400/60">{"command:"}</p><p class="text-sm font-mono font-bold" style="padding-top: 0.175rem;">{format!("{} ({:#?})", item.command, item.c_type)}</p>
                                            if item.c_alias.is_some() {
                                                <div class="flex ml-auto pl-6">
                                                <p class="pr-2 text-gray-400/60">{"aliases:"}</p><p class="text-sm font-mono font-bold" style="padding-top: 0.175rem;">{item.c_alias}</p></div>
                                            }
                                        </div>
                                        <div class="flex"><p class="pr-2 text-gray-400/60">{"description:"}</p><p>{item.desc}</p></div>
                                    </li>
                                }
                            })
                    }
                </ul>
            </div>
        </div>
    }
}
