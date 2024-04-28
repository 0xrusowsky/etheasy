use crate::components::frame::FrameComponent;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Landing,
    #[at("/playground")]
    Playground,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Landing => html! { <h1>{"Rust WebAssembly"}</h1> },
        Route::Playground => {
            html! { <FrameComponent /> }
        }
        Route::NotFound => html! { <p class="text-white">{ "Not found" }</p> },
    }
}
