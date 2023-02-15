mod app;
mod components;
mod helpers;
mod icon_paths;
mod types;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
