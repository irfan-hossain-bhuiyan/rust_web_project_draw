use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, MessageEvent, WebSocket};
use rust_web::{DrawEvent, CanvasState};
use crate::types::pixel_canvas::GridIndex;

/// WebSocket service for collaborative drawing
#[derive(Clone)]
pub struct WebSocketService {
    websocket: Option<WebSocket>,
    on_canvas_update: WriteSignal<Option<CanvasState>>,
    on_draw_event: WriteSignal<Option<DrawEvent>>,
}

impl WebSocketService {
    /// Create a new WebSocket service
    pub fn new(
        on_canvas_update: WriteSignal<Option<CanvasState>>,
        on_draw_event: WriteSignal<Option<DrawEvent>>,
    ) -> Self {
        Self {
            websocket: None,
            on_canvas_update,
            on_draw_event,
        }
    }

    /// Connect to the WebSocket server
    pub fn connect(&mut self) -> Result<(), JsValue> {
        let ws_url = if web_sys::window()
            .unwrap()
            .location()
            .protocol()
            .unwrap()
            .starts_with("https")
        {
            format!("wss://{}/ws", web_sys::window().unwrap().location().host().unwrap())
        } else {
            format!("ws://{}/ws", web_sys::window().unwrap().location().host().unwrap())
        };

        let websocket = WebSocket::new(&ws_url)?;
        websocket.set_binary_type(BinaryType::Arraybuffer);

        // Clone signals for closures
        let on_canvas_update = self.on_canvas_update;
        let on_draw_event = self.on_draw_event;

        // Set up message handler
        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(array_buffer) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                let data: Vec<u8> = uint8_array.to_vec();

                match data.len() {
                    9 => {
                        // Draw event
                        let mut event_bytes = [0u8; 9];
                        event_bytes.copy_from_slice(&data);
                        let event = DrawEvent::from_bytes(&event_bytes);
                        
                        web_sys::console::log_2(
                            &"📨 Received draw event:".into(),
                            &format!("{:?}", event).into(),
                        );
                        
                        on_draw_event.set(Some(event));
                    }
                    1251 => {
                        // Full canvas sync (1 byte header + 1250 bytes canvas data)
                        if data[0] == 1 {
                            let mut canvas_bytes = [0u8; 1250];
                            canvas_bytes.copy_from_slice(&data[1..]);
                            let canvas_state = CanvasState::from_bytes(&canvas_bytes);
                            
                            web_sys::console::log_1(&"🎨 Received full canvas sync".into());
                            
                            on_canvas_update.set(Some(canvas_state));
                        }
                    }
                    _ => {
                        web_sys::console::warn_2(
                            &"⚠️ Received unknown message length:".into(),
                            &data.len().into(),
                        );
                    }
                }
            }
        }) as Box<dyn FnMut(_)>);

        websocket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        // Set up connection handlers
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            web_sys::console::log_1(&"🔗 WebSocket connected".into());
        }) as Box<dyn FnMut(_)>);

        websocket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |_| {
            web_sys::console::error_1(&"❌ WebSocket error".into());
        }) as Box<dyn FnMut(_)>);

        websocket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let onclose_callback = Closure::wrap(Box::new(move |_| {
            web_sys::console::log_1(&"🔌 WebSocket disconnected".into());
        }) as Box<dyn FnMut(_)>);

        websocket.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
        onclose_callback.forget();

        self.websocket = Some(websocket);
        Ok(())
    }

    /// Send a draw event to the server
    pub fn send_draw_event(&self, from: GridIndex, to: GridIndex, is_black: bool) {
        if let Some(ref websocket) = self.websocket {
            let event = DrawEvent::draw_line(from.x, from.y, to.x, to.y, is_black);
            let bytes = event.to_bytes();
            
            web_sys::console::log_2(
                &"📤 Sending draw event:".into(),
                &format!("{:?}", event).into(),
            );
            
            if let Err(e) = websocket.send_with_u8_array(&bytes) {
                web_sys::console::error_2(&"Failed to send draw event:".into(), &e);
            }
        }
    }

    /// Send a clear canvas event
    pub fn send_clear_event(&self) {
        if let Some(ref websocket) = self.websocket {
            let event = DrawEvent::clear_canvas();
            let bytes = event.to_bytes();
            
            web_sys::console::log_1(&"🧹 Sending clear event".into());
            
            if let Err(e) = websocket.send_with_u8_array(&bytes) {
                web_sys::console::error_2(&"Failed to send clear event:".into(), &e);
            }
        }
    }

    /// Check if WebSocket is connected
    pub fn is_connected(&self) -> bool {
        self.websocket
            .as_ref()
            .map(|ws| ws.ready_state() == WebSocket::OPEN)
            .unwrap_or(false)
    }
}
