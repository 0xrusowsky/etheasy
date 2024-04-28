mod app;
mod components;
mod parser;
mod router;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
