mod app;
mod components;
mod parser;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
