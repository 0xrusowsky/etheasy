use crate::components::clipboard::ClipboardComponent;
use crate::parser::utils::trim_quotes;

use serde_json::{Map, Value};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct JsonProps {
    pub value: serde_json::Value,
}

pub struct JsonComponent {}
impl Component for JsonComponent {
    type Message = ();
    type Properties = JsonProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let pretty_html = json_to_html(&ctx.props().value, 0);

        html! {
            <div style="font-family: monospace;">
                {pretty_html}
            </div>
        }
    }
}

fn json_to_html(value: &Value, indent: usize) -> Html {
    let indent_str = "\u{00a0}".repeat(indent * 2);
    match value {
        Value::Object(map) => single_obj_to_html(map, indent),
        Value::Array(vec) => {
            if indent != 0 {
                array_to_html(vec, indent)
            } else {
                if vec.is_empty() {
                    html! { <div>{format!("{}{}", indent_str, "[],")}</div> }
                } else {
                    html! { <> {
                        for vec.iter().map(|item| html! {
                            <>
                                {json_to_html(item, indent + 1)}
                            </>
                        })
                    } </> }
                }
            }
        }
        Value::String(s) => html! {
            <div>{format!("{}\"{}\"", indent_str, s)}</div>
        },
        _ => html! {
            <div>{format!("{}{}", indent_str, value)}</div>
        },
    }
}

fn single_obj_to_html(obj: &Map<String, Value>, indent: usize) -> Html {
    let indent_str = "\u{00a0}".repeat(indent * 2);
    if obj.is_empty() {
        html! { <div>{"{}"}</div> }
    } else if obj.len() == 1 {
        let (k, v) = obj.iter().next().unwrap();
        html! {<>
            <div class="flex">
                <span class="pr-2">{format!("{}{}: {}", indent_str, k, v)}</span>
                    <ClipboardComponent
                        text={trim_quotes(&v.to_string())}
                        text_style={"text-gray-500 hover:text-gray-50"}
                    />
            </div> if k == "fn_selector" { <br /> }
        </>}
    } else {
        html! { <>
        { for obj.iter().map(|(k, v)| html! {
            <div>
                <span>{format!("{}{}: {}", indent_str, k, "\u{007b}")}</span>
                {json_to_html(v, indent + 1)}
                <span>{format!("{}\u{00a7}", indent_str)}</span>
            </div>
        })}
        </> }
    }
}

fn array_obj_to_html(obj: Option<&Map<String, Value>>, indent: usize) -> Html {
    let indent_str = "\u{00a0}".repeat(indent * 2);
    match obj {
        Some(obj) => {
            if obj.is_empty() {
                html! { <div>{format!("{}{}", indent_str, "{},")}</div> }
            } else if obj.len() == 1 {
                html! { <>
                { for obj.iter().map(|(_, v)| html! {
                    <div>
                        <span>{format!("{}{},", indent_str, v)}</span>
                    </div>
                })}
                </> }
            } else {
                html! { <>
                { for obj.iter().map(|(_, v)| html! {
                    <div>
                        <span>{format!("{}{},", indent_str, "\u{007b}")}</span>
                        {json_to_html(v, indent + 1)}
                        <span>{format!("{}\u{00a7}", indent_str)}</span>
                    </div>
                })}
                </> }
            }
        }
        None => html! { <div>{"{},"}</div> },
    }
}

fn array_to_html(vec: &Vec<Value>, indent: usize) -> Html {
    let indent_str = "\u{00a0}".repeat(indent * 2);
    if vec.is_empty() {
        html! { <div>{"[]"}</div> }
    } else {
        match &vec[0] {
            Value::Object(obj) => {
                if !obj.is_empty() {
                    let obj_type = obj.iter().next().unwrap().0;
                    html! {
                    <>
                        <div>{format!("{}{}[]: [", indent_str, obj_type)}</div> <> {
                        for vec.iter().map(|item| html! {
                            <>
                                {array_obj_to_html(item.as_object(), indent + 1)}
                            </>
                        })
                        } </> <div>{format!("{}],", indent_str)}</div>
                    </>
                        }
                } else {
                    html! { <div>{"[]"}</div> }
                }
            }
            _ => {
                html! {
                <>
                    <div>{format!("{}[", indent_str)}</div> <> {
                    for vec.iter().map(|item| html! {
                        <>
                            {json_to_html(item, indent + 1)}
                        </>
                    })
                     } </> <div>{"{}],"}</div>
                </>
                    }
            }
        }
    }
}
