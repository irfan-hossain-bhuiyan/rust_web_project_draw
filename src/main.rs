use leptos::prelude::*;

mod components;
mod types;
mod services;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Hello, Leptos World!"</h1>
            <p>"Welcome to the Collaborative Pixel Canvas"</p>
            <div>"Ready to start building!"</div>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
