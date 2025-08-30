use std::time::Duration;

use bincode::config;
use frontend::prelude::BytesPassthrough;
// This can be empty for now or contain server-side code if you plan to use SSR later
use leptos::{
    logging::{error, log},
    prelude::*,
};
use leptos_router::{
    components::{A, Route, Router, Routes},
    hooks::{use_params_map, use_query_map},
    path,
};
use leptos_use::{self, UseWebSocketReturn, core::ConnectionReadyState, use_websocket};
mod components;
mod types;
use components::{Canvas, DrawingTool};
pub use frontend::prelude;
use shared::DataPass;
use uuid::Uuid;

use crate::{
    components::toolbar::ToolbarWithTrigger,
    types::pixel_canvas::{GridIndex, PixelCanvas},
};

#[component]
fn HomePage() -> impl IntoView {
    let width = RwSignal::new(100usize);
    let height = RwSignal::new(100usize);

    view! {
        <div class="homepage">
            <div class="intro">
                <h1>"🎨 Pixel Canvas"</h1>
                <p>
                    "Welcome to Pixel Canvas – a collaborative pixel art platform where you and your friends
                    can create amazing pixel art together on the same canvas in real time!"
                </p>
            </div>

            <div class="form-section">
                <h2>"Create a New Canvas"</h2>

                <label class="input-label">
                    "Width: "
                    <input
                        type="number"
                        prop:value=width
                        class="input-box"
                        min="10"
                        max="500"
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse() {
                                width.set(val);
                            }
                        }
                    />
                </label>

                <label class="input-label">
                    "Height: "
                    <input
                        type="number"
                        prop:value=height
                        class="input-box"
                        min="10"
                        max="500"
                        on:input=move |ev| {
                            if let Ok(val) = event_target_value(&ev).parse() {
                                height.set(val);
                            }
                        }
                    />
                </label>

                <A
                    attr:class="start-btn"
                    href=move || {
                        let id = Uuid::new_v4();
                        format!("/drawing/{}?width={}&height={}", id, width.get(), height.get())
                    }
                >
                    "🚀 Start Drawing"
                </A>
            </div>
        </div>
    }
}

#[component]
fn DrawingPage() -> impl IntoView {
    let params = use_params_map();
    let query = use_query_map();
    let UseWebSocketReturn {
        ready_state,
        message,
        ws,
        open,
        close,
        send,
        ..
    } = use_websocket::<Vec<u8>, Vec<u8>, BytesPassthrough>("ws://127.0.0.1:8081");

    let session_id = params
        .with(|p| p.get("id"))
        .unwrap_or_else(|| "unknown".to_string());
    let width: usize = query.with(|q| q.get("width").and_then(|v| v.parse().ok()).unwrap_or(100));
    let height: usize = query.with(|q| q.get("height").and_then(|v| v.parse().ok()).unwrap_or(100));
    let data = DataPass::Whid {
        width,
        height,
        id: session_id.clone(),
    };
    let buf = bincode::serde::encode_to_vec(&data, bincode::config::standard()).unwrap();
    let sent = RwSignal::new(false);
    let send_c = send.clone();
    Effect::new(move |_| {
        if !sent.get() && ready_state.get() == ConnectionReadyState::Open {
            match bincode::serde::encode_to_vec(&data, bincode::config::standard()) {
                Ok(buf) => {
                    log!("Connection established! Sending data:");
                    log!("width: {width}, height: {height}");
                    log!("encoded data: {buf:?}");
                    send_c(&buf);
                    sent.set(true); // Mark as sent
                }
                Err(e) => {
                    log!("Failed to encode data: {e:?}");
                }
            }
        }
    });
    log!(
        "
        now width is {width},
        height is {height},
        new send from {buf:?}"
    );
    view! {
        <div>
            <p>{format!("Session: {session_id}")}</p>
            <App width=width height=height message=message send=send ready_state=ready_state/>
        </div>
    }
}
#[component]
fn App(
    width: usize,
    height: usize,
    message: Signal<Option<Vec<u8>>>,
    send: impl Fn(&Vec<u8>) + Clone + Send + Sync + 'static,
    ready_state: Signal<ConnectionReadyState>,
) -> impl IntoView {
    // Shared state for the selected drawing tool
    let send_c = send.clone();
    let selected_tool = RwSignal::new(DrawingTool::default());
    let canvas_state = RwSignal::new(PixelCanvas::new_in_middle(GridIndex {
        x: width,
        y: height,
    }));

    //region outgoing call
    //Effect::new(move || {
    //    if ready_state.get() == ConnectionReadyState::Open {
    //        let bytes = canvas_state.get().main_canvas_to_bytes();
    //        let bytes = bincode::serde::encode_to_vec(
    //            &DataPass::Canvas { data: bytes },
    //            config::standard(),
    //        )
    //        .unwrap();
    //        send_c(&bytes);
    //    }
    //});
    let send_c=send.clone();
    set_interval(
        move || {
            let bytes = canvas_state.get().drawing_canvas_to_bytes();
            let bytes = bincode::serde::encode_to_vec(
                &DataPass::Canvas { data: bytes },
                config::standard(),
            )
            .unwrap();
            send_c(&bytes);

            canvas_state.update(|x| {
                x.update_drawing();
            });
        },
        Duration::from_secs_f32(1.0 / 4.0),
    );

    //endregion
    // region canvas ingoing call
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

    view! {
        <div class="app">
            <ToolbarWithTrigger selected_tool=selected_tool canvas=canvas_state/>
            <Canvas selected_tool=selected_tool canvas_state=canvas_state/>
        </div>
    }
}

#[component]
fn Root() -> impl IntoView {
    view! {
        <Router>
            <Routes fallback=|| view! { <p>"Not found"</p> }>
                <Route path=path!("/") view=HomePage />
                <Route path=path!("/drawing/:id") view=DrawingPage />
            </Routes>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(Root);
}
