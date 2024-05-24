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
                class="text-sm px-6 py-2 border-t-2 border-gray-400 text-gray-200/60 focus:text-gray-200 hover:text-gray-200 focus:bg-gray-800/50 focus:bg-gray-800/50 hover:bg-gray-800/50 hover:bg-gray-800/50 ring-0 outline-0"
            >
            <div class="flex">
                <p class="pr-2 dark:text-gray-300/50 text-gray-200/50">{"command:"}</p>
                <p class="text-xs font-mono font-bold" style="padding-top: 0.175rem;">{format!("{} ({:#?})", item.command, item.c_type)}</p>
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
                    <div style="transform: translate(0, 7.5vh);"
                         class="text-sm text-gray-50 bg-gray-600/90 dark:bg-gray-500/90 rounded-lg p-4 max-lg:4/5 w-3/4 relative  border-4 border-gray-200/80"
                    >
                        <div class="flex font-bold">
                            <p class="pr-2 text-emerald-400/80">{"command:"}</p>
                            if item.params.is_some() {
                                <code class="font-mono pl-2 overflow-x-auto">{format_code_with_comments(item.params.unwrap(), "text-gray-400/80", "text-amber-300/80")} </code>
                                <p class="font-mono pl-2">{format!("({})", item.c_type.to_string())}</p>
                            } else {
                                <p class="font-mono font-bold">{format!("{} ({})", item.command, item.c_type.to_string())}</p>
                            }
                        </div>

                        if item.alias.is_some() {
                            <div class="flex pt-1 pb-3">
                                <p class="pl-5 pr-2 text-emerald-400/80 font-bold">{"aliases:"}</p>
                                <p class="font-mono pl-2" style="padding-top: 0.1rem;">{item.alias}</p>
                            </div>
                        }
                        <div class="flex pt-3">
                            <p class="pr-2 text-emerald-400/80 font-bold">{"description:"}</p>
                            <div class="flex-col">{ format_text_with_code(item.desc, "font-mono font-bold px-1 text-amber-300/70")}</div>
                        </div>
                        if item.example.is_some() {
                            <div class="pt-3">
                                <p class="pr-2 text-emerald-400/80 font-bold">{"examples:"}</p>
                                <pre class="bg-gray-800 text-white/80 text-xs font-mono p-4 rounded-lg overflow-x-auto">
                                    <code> {format_code_with_comments(item.example.unwrap(), "text-gray-400/80", "text-amber-300/60")} </code>
                                </pre>
                            </div>
                        }
                    </div>
                </div>
            },
            None => html! {<></>},
        }
    }
}

fn format_text_with_code(text: &str, code_style: &'static str) -> Html {
    html! {
        for text.split('\n').map(|line| parse_line(line, code_style))
    }
}

fn parse_line(line: &str, code_style: &'static str) -> Html {
    let mut elements = Vec::new();
    let mut in_code = false;

    for part in line.split('`') {
        if in_code {
            elements.push(html! { <code class={code_style}>{part}</code> });
        } else {
            elements.push(html! { <span>{part}</span> });
        }
        in_code = !in_code;
    }

    html! { <p>{ for elements }</p> }
}

fn format_code_with_comments(
    code: &str,
    comment_style: &'static str,
    param_style: &'static str,
) -> Html {
    html! {
        for code.split('\n').map(|line| {
            if let Some(comment_start) = line.find("//") {
                if comment_start == 0 {
                    html! { <div class="pb-1"><span class={comment_style}>{line}</span></div> }
                } else {
                    let (code_part, comment_part) = line.split_at(comment_start);
                    html! {
                        <div class="flex">
                            <span>{parse_line(code_part, param_style)}</span>
                            <span class={comment_style}>{comment_part}</span>
                        </div>
                    }
                }
            } else {
                html! { <div>{parse_line(line, param_style)}</div> }
            }
        })
    }
}
