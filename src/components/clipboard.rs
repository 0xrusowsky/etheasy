use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yew_hooks::use_clipboard;

#[derive(Properties, PartialEq)]
pub struct ClipboardProps {
    pub text: String,
    pub text_style: &'static str,
}

#[function_component(ClipboardComponent)]
pub fn clipboard_component(props: &ClipboardProps) -> Html {
    let style = format!("{} {}", "w-4 h-6", props.text_style);
    let clipboard = use_clipboard();
    let copied = use_state(|| false);

    let on_click = {
        let text = props.text.clone();
        let clipboard = clipboard.clone();
        let copied = copied.clone();
        Callback::from(move |_: MouseEvent| {
            clipboard.write_text(text.clone());
            copied.set(true);
            let copied = copied.clone();
            Timeout::new(1000, move || copied.set(false)).forget();
        })
    };

    let icon = if *copied {
        html! {
            <svg class={style} fill="currentColor" viewBox="0 0 448 512" xmlns="http://www.w3.org/2000/svg">
                <g transform="translate(0, -25) scale(0.70)">
                    <path d="M438.6 154.6L194.1 400.4l-107-107c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l128 128c9.4 9.4 24.6 9.4 33.9 0l256-256c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0z"/>
                </g>
            </svg>
        }
    } else {
        html! {
            <svg class={style} fill="currentColor" viewBox="0 0 448 512" xmlns="http://www.w3.org/2000/svg">
                <g transform="translate(0, -25) scale(0.70)">
                    <path d="M384 336H192c-8.8 0-16-7.2-16-16V64c0-8.8 7.2-16 16-16l140.1 0L400 115.9V320c0 8.8-7.2 16-16 16zM192 384H384c35.3 0 64-28.7 64-64V115.9c0-12.7-5.1-24.9-14.1-33.9L366.1 14.1c-9-9-21.2-14.1-33.9-14.1H192c-35.3 0-64 28.7-64 64V320c0 35.3 28.7 64 64 64zM64 128c-35.3 0-64 28.7-64 64V448c0 35.3 28.7 64 64 64H256c35.3 0 64-28.7 64-64V416H272v32c0 8.8-7.2 16-16 16H64c-8.8 0-16-7.2-16-16V192c0-8.8 7.2-16 16-16H96V128H64z"/>
                </g>
            </svg>
        }
    };

    html! {
        <button onclick={on_click}>{icon}</button>
    }
}
