// This can be empty for now or contain server-side code if you plan to use SSR later
use leptos::prelude::*;
use leptos_use::{self, use_websocket, UseWebSocketReturn};
mod components;
mod types;
mod services;
use components::{Canvas, Toolbar, DrawingTool};
pub use frontend::prelude;

use crate::components::toolbar::ToolbarWithTrigger;

#[component]
fn App() -> impl IntoView {
    // Shared state for the selected drawing tool
    let selected_tool = RwSignal::new(DrawingTool::default());
    let UseWebSocketReturn{ ready_state, message, ws, open, close, send, .. }=use_websocket::<Vec<u8>,Vec<u8>,DummyCodec>("127.0.0.1:8081");
    
    view! {
        <div class="app">
            <ToolbarWithTrigger selected_tool=selected_tool />
            <Canvas selected_tool=selected_tool />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
