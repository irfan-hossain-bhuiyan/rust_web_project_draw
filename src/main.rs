use leptos::prelude::*;

mod components;
mod types;
mod services;

use components::Canvas;

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
