use std::time::Duration;

use frontend::prelude::BytesPassthrough;
// This can be empty for now or contain server-side code if you plan to use SSR later
use leptos::{logging::{error, log}, prelude::*};
use leptos_use::{self, UseWebSocketReturn, use_websocket};
mod components;
mod services;
mod types;
use components::{Canvas, DrawingTool};
pub use frontend::prelude;

use crate::{components::toolbar::ToolbarWithTrigger, types::pixel_canvas::PixelCanvas};

#[component]
fn App() -> impl IntoView {
    // Shared state for the selected drawing tool
    let UseWebSocketReturn {
        ready_state,
        message,
        ws,
        open,
        close,
        send,
        ..
    } = use_websocket::<Vec<u8>, Vec<u8>, BytesPassthrough>("127.0.0.1:8081");

    let selected_tool = RwSignal::new(DrawingTool::default());
    let canvas_state = RwSignal::new(PixelCanvas::default());

    // region canvas input update
    Effect::new(move || {
        let Some(bin_data) = message.get() else {
            log!("Signal is null");
            return;
        };
        canvas_state.update(|x| {
            if let Err(err) = x.assign_pixel_bytes(&bin_data) {
                error!("{err}");
            }
        });
    });
    // endregion
    
    //set_interval(move ||{
    //    canvas_state.update(|x|{
    //        x.update_drawing();
    //        send(&x.to_bytes());
    //    });
    //}, Duration::from_secs_f32(1.0/4.0));
    view! {
        <div class="app">
            <ToolbarWithTrigger selected_tool=selected_tool />
            <Canvas selected_tool=selected_tool canvas_state=canvas_state/>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
