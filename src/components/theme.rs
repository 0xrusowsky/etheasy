use yew::prelude::*;
use yew::Component;

pub enum Switch {
    LightMode,
    DarkMode,
}

pub struct ThemeComponent {
    dark_mode: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ThemeComponentProps {
    pub on_clicked: Callback<bool>,
}

impl Component for ThemeComponent {
    type Message = Switch;
    type Properties = ThemeComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { dark_mode: false }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Switch::DarkMode => {
                self.dark_mode = true;
            }
            Switch::LightMode => {
                self.dark_mode = false;
            }
        };
        // Inform the parent component about the change
        ctx.props().on_clicked.emit(self.dark_mode);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let dark_mode = format!(
            "transition-all duration-400 ease-in-out absolute h-8 md:h-7 w-1/2 rounded-full {}",
            if self.dark_mode {
                "bg-gradient-to-r from-emerald-400 to-teal-600 opacity-80 transform translate-x-full"
            } else {
                "bg-gradient-to-r from-gray-300 to-gray-500 opacity-50 transform translate-x-0"
            }
        );

        html! {
            <div class="relative flex w-fit items-center rounded-full">
                // Light Mode
                <button onclick={ctx.link().callback(|_| Switch::LightMode)}
                    class="group flex items-center gap-2 px-3 md:pl-3 md:pr-3.5 py-3 md:py-1.5 transition-colors relative z-10 text-gray-900 dark:text-gray-400 dark:hover:text-gray-300">
                    <svg class="dark:group-hover:scale-110" width="17" height="17" viewBox="-1 0 25 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="24" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
                    </svg>
                    <p class="text-sm hidden sm:inline">{"Light"}</p>
                </button>
                // Dark Mode
                <button onclick={ctx.link().callback(|_| Switch::DarkMode)}
                    class="group flex items-center gap-2 px-3 md:pl-3 md:pr-3.5 py-3 md:py-1.5 transition-colors relative z-10 text-gray-500 dark:text-gray-100 hover:text-gray-600">
                    <svg class="group-hover:scale-110 dark:group-hover:scale-100" width="17" height="17" viewBox="-1 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                    </svg>
                    <p class="text-sm hidden sm:inline">{"Dark"}</p>
                </button>
                <span class={dark_mode}/>
            </div>
        }
    }
}
