// This can be empty for now or contain server-side code if you plan to use SSR later
use leptos::prelude::*;

mod components;
mod types;
mod services;
use components::Canvas;
pub use rust_web::prelude;
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="app">
            <Canvas />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
