use serde_json::{json, Value};
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
    let indent_str = "\u{00a0}".repeat(indent * 4); // Each indent level adds four non-breaking spaces
    match value {
        Value::Object(map) => {
            if map.is_empty() {
                html! { <div>{"{}"}</div> }
            } else if map.len() == 1 {
                html! { <>
                { for map.iter().map(|(k, v)| html! {
                    <div>
                        <span>{format!("{}{}: {}", indent_str, k, v)}</span>
                    </div>
                })}
                </> }
            } else {
                html! { <>
                { for map.iter().map(|(k, v)| html! {
                    <div>
                        <span>{format!("{}{}: ", indent_str, k)}</span>
                        {json_to_html(v, indent + 1)}
                    </div>
                })}
                </> }
            }
        }
        Value::Array(vec) => {
            if vec.is_empty() {
                html! { <div>{"[]"}</div> }
            } else {
                html! {
                <>
                    <div>{format!("{}[", indent_str)}</div> <> {
                    for vec.iter().map(|item| html! {
                        <>
                            {json_to_html(item, indent + 1)}
                        </>
                    })
                     } </> <div>{format!("{}]{}", indent_str, if indent_str != "" {","} else {""})}</div>
                </>
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
