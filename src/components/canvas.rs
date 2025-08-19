use leptos::prelude::*;
use leptos::html;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};

use crate::types::pixel_canvas::{PixelCanvas, CANVAS_BACKGROUND_COLOR};

/// Get Canvas2D rendering context from canvas element
fn get_canvas_2d_context(canvas: &HtmlCanvasElement) -> Result<CanvasRenderingContext2d, String> {
    canvas
        .get_context("2d")
        .map_err(|_| "Failed to get canvas context".to_string())?
        .ok_or("Canvas context is null".to_string())?
        .dyn_into::<CanvasRenderingContext2d>()
        .map_err(|_| "Failed to cast to CanvasRenderingContext2d".to_string())
}

/// Get window dimensions with fallback defaults
fn get_window_dimensions() -> (u32, u32) {
    const DEFAULT_WIDTH: u32 = 800;
    const DEFAULT_HEIGHT: u32 = 600;
    
    let Some(window) = window() else {
        web_sys::console::warn_1(&"Window object not available, using default dimensions".into());
        return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
    };
    
    // Get width: Result<JsValue> -> Option<f64> -> u32
    let Ok(width_js) = window.inner_width() else {
        return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
    };
    let Some(width) = width_js.as_f64() else {
        return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
    };
    let width = width as u32;
    
    // Get height: Result<JsValue> -> Option<f64> -> u32  
    let Ok(height_js) = window.inner_height() else {
        return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
    };
    let Some(height) = height_js.as_f64() else {
        return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
    };
    let height = height as u32;
    
    (width, height)
}





#[component]
pub fn Canvas() -> impl IntoView {
    let canvas_ref = NodeRef::<html::Canvas>::new();
    
    // Create RwSignal for pixel canvas state
    let pixel_canvas = RwSignal::new(PixelCanvas::default());
    
    // Keyboard input handler
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let step = 20.0; // Movement step size
        match ev.key().as_str() {
            "ArrowLeft" => {
                pixel_canvas.update(|pc| pc.position.x -= step);
                ev.prevent_default();
            }
            "ArrowRight" => {
                pixel_canvas.update(|pc| pc.position.x += step);
                ev.prevent_default();
            }
            "ArrowUp" => {
                pixel_canvas.update(|pc| pc.position.y -= step);
                ev.prevent_default();
            }
            "ArrowDown" => {
                pixel_canvas.update(|pc| pc.position.y += step);
                ev.prevent_default();
            }
            "=" | "+" => {
                // Zoom in
                pixel_canvas.update(|pc| pc.zoom = (pc.zoom * 1.2).min(5.0));
                ev.prevent_default();
            }
            "-" => {
                // Zoom out
                pixel_canvas.update(|pc| pc.zoom = (pc.zoom / 1.2).max(0.2));
                ev.prevent_default();
            }
            _ => {}
        }
    };
    
    // Effect to set up canvas dimensions (runs once)
    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let (width, height) = get_window_dimensions();
            canvas.set_width(width);
            canvas.set_height(height);
        }
    });
    
    // Effect to redraw canvas when pixel_canvas state changes
    Effect::new(move |_| {
        let canvas_state = pixel_canvas.get(); // This creates the reactive dependency
        
        if let Some(canvas) = canvas_ref.get() {
            let Ok(context) = get_canvas_2d_context(&canvas) else {
                web_sys::console::error_1(&"Failed to get canvas context".into());
                return;
            };
            
            let (width, height) = get_window_dimensions();
            
            // Clear canvas with background
            context.set_fill_style_str(CANVAS_BACKGROUND_COLOR);
            context.fill_rect(0.0, 0.0, width as f64, height as f64);
            
            // Draw the pixel canvas using its draw method
            canvas_state.draw(&context);
        }
    });

    view! {
        <canvas 
            node_ref=canvas_ref
            class="fullscreen-canvas"
            on:keydown=handle_keydown
            tabindex="0"  // Make canvas focusable for keyboard events
        >
        </canvas>
    }
}
