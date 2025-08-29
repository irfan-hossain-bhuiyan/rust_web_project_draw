use frontend::prelude::PixelColor;
use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::{ev, leptos_dom::helpers::window_event_listener};
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::types::pixel_canvas::{PixelCanvas, CANVAS_BACKGROUND_COLOR, GridIndex};
use crate::components::DrawingTool;

pub static mut PEN_TOUCHED:bool=false;
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

#[component]
pub fn Canvas(
    #[prop(into)] canvas_state: RwSignal<PixelCanvas>,
    #[prop(into)] selected_tool: RwSignal<DrawingTool>,
) -> impl IntoView {
    let canvas_ref = NodeRef::<html::Canvas>::new();
    // Create RwSignal for pixel canvas state
    // Drawing state enum
    #[derive(Clone, Debug)]
    enum DrawingState {
        NotClicked,
        Clicked { last_position: GridIndex },
    }

    // Mouse interaction state
    let is_dragging = RwSignal::new(false);
    let mouse_position = RwSignal::new(None::<(f64, f64)>);
    let drawing_state = RwSignal::new(DrawingState::NotClicked);

    //region input handler
    //region handle keyboard
    let handle_keydown = move |ev: web_sys::KeyboardEvent| {
        let step = 20.0; // Movement step size
        match ev.key().as_str() {
            "ArrowLeft" => {
                canvas_state.update(|pc| pc.x_shift(step));
                ev.prevent_default();
            }
            "ArrowRight" => {
                canvas_state.update(|pc| pc.x_shift(-step));
                ev.prevent_default();
            }
            "ArrowUp" => {
                canvas_state.update(|pc| pc.y_shift(step));
                ev.prevent_default();
            }
            "ArrowDown" => {
                canvas_state.update(|pc| pc.y_shift(-step));
                ev.prevent_default();
            }
            "=" | "+" => {
                // Zoom in
                canvas_state.update(|pc| pc.zoom_in(1.2));
                ev.prevent_default();
            }
            "-" => {
                // Zoom out
                canvas_state.update(|pc| pc.zoom_out(1.2));
                ev.prevent_default();
            }
            _ => {}
        }
    };
    //endregion
    //region handle mouse
    let handle_mousedown = move |ev: web_sys::MouseEvent| {
        match ev.button() {
            1 => {
                // Middle mouse button - panning
                is_dragging.set(true);
                ev.prevent_default();
            }
            0 => {
                // Left mouse button - drawing/erasing
                let mouse_x = ev.client_x() as f64;
                let mouse_y = ev.client_y() as f64;
                
                canvas_state.with(|pc| {
                    let pos = crate::prelude::Position::new(mouse_x, mouse_y);
                    let grid_pos = pc.closest_grid_index_from_point(pos);
                    drawing_state.set(DrawingState::Clicked { last_position: grid_pos });
                });
                
                ev.prevent_default();
            }
            _ => {} // Ignore other mouse buttons
        }
    };
    let handle_mouseup = move |ev: web_sys::MouseEvent| {
        match ev.button() {
            1 => {
                // Middle mouse button - stop panning
                is_dragging.set(false);
                ev.prevent_default();
            }
            0 => {
                // Left mouse button - stop drawing
                drawing_state.set(DrawingState::NotClicked);
                ev.prevent_default();
            }
            _ => {} // Ignore other mouse buttons
        }
    };

    let handle_mousemove = move |ev: web_sys::MouseEvent| {
        // Always track mouse position for hover effects
        let mouse_x = ev.client_x() as f64;
        let mouse_y = ev.client_y() as f64;
        mouse_position.set(Some((mouse_x, mouse_y)));
        
        // Handle panning (middle mouse drag)
        if is_dragging.get() {
            let delta_x = ev.movement_x() as f64;
            let delta_y = ev.movement_y() as f64;

            canvas_state.update(|pc| {
                pc.x_shift(delta_x);
                pc.y_shift(delta_y);
            });

            ev.prevent_default();
        }
        
        // Handle drawing during mouse movement
        if let DrawingState::Clicked { last_position } = drawing_state.get() {
            let current_tool = selected_tool.get();
            
            canvas_state.update(|pc| {
                let pos = crate::prelude::Position::new(mouse_x, mouse_y);
                let current_pos = pc.closest_grid_index_from_point(pos);
                
                match current_tool {
                    DrawingTool::Pen => {
                        // Draw line from last position to current position
                        log!("pixel drawing.");
                        //pc.pixel_draw(current_pos,PixelColor::BLACK);
                        pc.line_draw(last_position, current_pos,PixelColor::BLACK);
                        unsafe {PEN_TOUCHED=true;}
                        assert!(!pc.is_drawing_transperent());
    
                    }
                    DrawingTool::Eraser => {
                        // Erase line from last position to current position
                        log!("Line erasing");
                        unsafe {PEN_TOUCHED=false;}
                        pc.line_draw(last_position, current_pos,PixelColor::ERASE);
                    }
                }
            });
            
            // Update last position for next draw
            canvas_state.with(|pc| {
                let pos = crate::prelude::Position::new(mouse_x, mouse_y);
                let current_pos = pc.closest_grid_index_from_point(pos);
                drawing_state.set(DrawingState::Clicked { last_position: current_pos });
            });
            
            ev.prevent_default();
        }
    };

    let handle_mouseleave = move |_ev: web_sys::MouseEvent| {
        // Clear mouse position when mouse leaves canvas
        mouse_position.set(None);
    };

    let handle_wheel = move |ev: web_sys::WheelEvent| {
        let zoom_factor = if ev.delta_y() < 0.0 { 1.1 } else { 1.0 / 1.1 };
        web_sys::console::log_1(&"Mouse scrolling".into());
        // Get mouse position relative to the canvas
        let mouse_x = ev.client_x() as f64;
        let mouse_y = ev.client_y() as f64;

        canvas_state.update(|pc| {
            pc.zoom_at_point(zoom_factor, mouse_x, mouse_y);
        });

        ev.prevent_default();
    };
    //endregion
    //endregion
        //region canvas rendering
    //region canvas initialize
    Effect::new(move |_| {
        // effect initialize things after canvas_ref get connected.
        if let Some(canvas) = canvas_ref.get() {
            let (width, height) = get_window_size().into();
            web_sys::console::log_2(
                &"🎯 Initial canvas setup:".into(),
                &format!("{}x{}", width, height).into(),
            );
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            // Set up window resize listener
        }
    });
    //endregion
    // region canvas draw on state
    let draw = move |canvas| {
        let Ok(context) = get_canvas_2d_context(&canvas) else {
            web_sys::console::error_1(&"Failed to get canvas context".into());
            return;
        };

        // Clear canvas with background
        context.set_fill_style_str(CANVAS_BACKGROUND_COLOR);
        context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        // Convert mouse position to Position if available
        let mouse_pos = mouse_position.get().map(|(x, y)| {
            use crate::prelude::Position;
            Position::new(x, y)
        });

        // Draw the pixel canvas using its draw method
        let canvas_state = canvas_state.get(); // This creates the reactive dependency
        canvas_state.draw(&context, mouse_pos);
    };
    Effect::new(move |_| {
        // Create reactive dependencies
        let _canvas_state = canvas_state.get();
        let _mouse_pos = mouse_position.get();
        let _drawing_state = drawing_state.get(); // Add drawing state as dependency
        
        if let Some(canvas) = canvas_ref.get() {
            draw(canvas)
        }
    });
    //endregion
    //endregion
//region window resize
    use crate::prelude::get_window_size;
    window_event_listener(ev::resize, move |_| {
        web_sys::console::log_1(&"🔄 Window resize event triggered!".into());
        if let Some(canvas) = canvas_ref.get() {
            let (width, height) = get_window_size().into();
            web_sys::console::log_2(
                &"📏 New dimensions:".into(),
                &format!("{width}x{height}").into(),
            );
            canvas.set_width(width as u32);
            canvas.set_height(height as u32);
            draw(canvas);
        }
    });
    //endregion

    // Expanded view! macro - creating canvas element manually
    use leptos::html::canvas;
    use leptos::IntoView;
    let canvas_element = canvas()
        .node_ref(canvas_ref)
        .class("fullscreen-canvas")
        .on(leptos::ev::keydown, handle_keydown)
        .on(leptos::ev::mousedown, handle_mousedown)
        .on(leptos::ev::mouseup, handle_mouseup)
        .on(leptos::ev::mousemove, handle_mousemove)
        .on(leptos::ev::mouseleave, handle_mouseleave)
        .on(leptos::ev::wheel, handle_wheel)
        .tabindex("0");

    canvas_element.into_view()
}
