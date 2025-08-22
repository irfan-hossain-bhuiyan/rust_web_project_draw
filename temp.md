#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use leptos::prelude::*;
mod components {
    pub mod canvas {
        use leptos::prelude::*;
        use leptos::html;
        use wasm_bindgen::JsCast;
        use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, window};
        use crate::types::pixel_canvas::{PixelCanvas, CANVAS_BACKGROUND_COLOR};
        /// State for tracking mouse drag operations
        struct DragState {
            is_dragging: bool,
            start_mouse_pos: Option<(f64, f64)>,
            start_canvas_pos: Option<(f64, f64)>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for DragState {
            #[inline]
            fn clone(&self) -> DragState {
                DragState {
                    is_dragging: ::core::clone::Clone::clone(&self.is_dragging),
                    start_mouse_pos: ::core::clone::Clone::clone(&self.start_mouse_pos),
                    start_canvas_pos: ::core::clone::Clone::clone(&self.start_canvas_pos),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for DragState {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "DragState",
                    "is_dragging",
                    &self.is_dragging,
                    "start_mouse_pos",
                    &self.start_mouse_pos,
                    "start_canvas_pos",
                    &&self.start_canvas_pos,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for DragState {
            #[inline]
            fn default() -> DragState {
                DragState {
                    is_dragging: ::core::default::Default::default(),
                    start_mouse_pos: ::core::default::Default::default(),
                    start_canvas_pos: ::core::default::Default::default(),
                }
            }
        }
        /// Get Canvas2D rendering context from canvas element
        fn get_canvas_2d_context(
            canvas: &HtmlCanvasElement,
        ) -> Result<CanvasRenderingContext2d, String> {
            canvas
                .get_context("2d")
                .map_err(|_| "Failed to get canvas context".to_string())?
                .ok_or("Canvas context is null".to_string())?
                .dyn_into::<CanvasRenderingContext2d>()
                .map_err(|_| "Failed to cast to CanvasRenderingContext2d".to_string())
        }
        /// Create all input event handlers for the canvas
        /// Get window dimensions with fallback defaults
        fn get_window_dimensions() -> (u32, u32) {
            const DEFAULT_WIDTH: u32 = 800;
            const DEFAULT_HEIGHT: u32 = 600;
            let Some(window) = window() else {
                web_sys::console::warn_1(
                    &"Window object not available, using default dimensions".into(),
                );
                return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
            };
            let Ok(width_js) = window.inner_width() else {
                return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
            };
            let Some(width) = width_js.as_f64() else {
                return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
            };
            let width = width as u32;
            let Ok(height_js) = window.inner_height() else {
                return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
            };
            let Some(height) = height_js.as_f64() else {
                return (DEFAULT_WIDTH, DEFAULT_HEIGHT);
            };
            let height = height as u32;
            (width, height)
        }
        /// Props for the [`Canvas`] component.
        ///
        #[builder(crate_module_path = ::leptos::typed_builder)]
        #[allow(non_snake_case)]
        pub struct CanvasProps {}
        #[automatically_derived]
        impl CanvasProps {
            /**
                Create a builder for building `CanvasProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `CanvasProps`.
                */
            #[allow(dead_code, clippy::default_trait_access)]
            pub fn builder() -> CanvasPropsBuilder<()> {
                CanvasPropsBuilder {
                    fields: (),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct CanvasPropsBuilder<TypedBuilderFields = ()> {
            fields: TypedBuilderFields,
            phantom: ::core::marker::PhantomData<()>,
        }
        #[automatically_derived]
        impl<TypedBuilderFields> Clone for CanvasPropsBuilder<TypedBuilderFields>
        where
            TypedBuilderFields: Clone,
        {
            #[allow(clippy::default_trait_access)]
            fn clone(&self) -> Self {
                Self {
                    fields: self.fields.clone(),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl CanvasPropsBuilder<()> {
            #[allow(
                clippy::default_trait_access,
                clippy::used_underscore_binding,
                clippy::no_effect_underscore_binding
            )]
            pub fn build(self) -> CanvasProps {
                let () = self.fields;
                #[allow(deprecated)] CanvasProps {}.into()
            }
        }
        #[allow(missing_docs)]
        impl ::leptos::component::Props for CanvasProps {
            type Builder = CanvasPropsBuilder;
            fn builder() -> Self::Builder {
                CanvasProps::builder()
            }
        }
        #[allow(non_snake_case, clippy::too_many_arguments)]
        #[allow(clippy::needless_lifetimes)]
        pub fn Canvas() -> impl IntoView {
            ::leptos::reactive::graph::untrack_with_diagnostics(move || { __Canvas() })
        }
        #[doc(hidden)]
        #[allow(
            non_snake_case,
            dead_code,
            clippy::too_many_arguments,
            clippy::needless_lifetimes
        )]
        pub fn __Canvas() -> impl IntoView {
            let canvas_ref = NodeRef::<html::Canvas>::new();
            let pixel_canvas = RwSignal::new(PixelCanvas::default());
            let drag_state = RwSignal::new(DragState::default());
            let handle_keydown = move |ev: web_sys::KeyboardEvent| {
                let step = 20.0;
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
                        pixel_canvas.update(|pc| pc.zoom = (pc.zoom * 1.2).min(5.0));
                        ev.prevent_default();
                    }
                    "-" => {
                        pixel_canvas.update(|pc| pc.zoom = (pc.zoom / 1.2).max(0.2));
                        ev.prevent_default();
                    }
                    _ => {}
                }
            };
            let handle_mousedown = move |ev: web_sys::MouseEvent| {
                if ev.button() == 1 {
                    let current_canvas = pixel_canvas.get();
                    drag_state
                        .update(|ds| {
                            ds.is_dragging = true;
                            ds.start_mouse_pos = Some((
                                ev.client_x() as f64,
                                ev.client_y() as f64,
                            ));
                            ds.start_canvas_pos = Some((
                                current_canvas.position.x,
                                current_canvas.position.y,
                            ));
                        });
                    ev.prevent_default();
                }
            };
            let handle_mousemove = move |ev: web_sys::MouseEvent| {
                let ds = drag_state.get();
                if ds.is_dragging {
                    if let (
                        Some((start_mouse_x, start_mouse_y)),
                        Some((start_canvas_x, start_canvas_y)),
                    ) = (ds.start_mouse_pos, ds.start_canvas_pos) {
                        let delta_x = ev.client_x() as f64 - start_mouse_x;
                        let delta_y = ev.client_y() as f64 - start_mouse_y;
                        pixel_canvas
                            .update(|pc| {
                                pc.position.x = start_canvas_x + delta_x;
                                pc.position.y = start_canvas_y + delta_y;
                            });
                    }
                }
            };
            let handle_mouseup = move |ev: web_sys::MouseEvent| {
                if ev.button() == 1 {
                    drag_state
                        .update(|ds| {
                            ds.is_dragging = false;
                            ds.start_mouse_pos = None;
                            ds.start_canvas_pos = None;
                        });
                }
            };
            Effect::new(move |_| {
                if let Some(canvas) = canvas_ref.get() {
                    let (width, height) = get_window_dimensions();
                    canvas.set_width(width);
                    canvas.set_height(height);
                }
            });
            Effect::new(move |_| {
                let canvas_state = pixel_canvas.get();
                if let Some(canvas) = canvas_ref.get() {
                    let Ok(context) = get_canvas_2d_context(&canvas) else {
                        web_sys::console::error_1(
                            &"Failed to get canvas context".into(),
                        );
                        return;
                    };
                    let (width, height) = get_window_dimensions();
                    context.set_fill_style_str(CANVAS_BACKGROUND_COLOR);
                    context.fill_rect(0.0, 0.0, width as f64, height as f64);
                    canvas_state.draw(&context);
                }
            });
            {
                #[allow(unused_braces)]
                {
                    ::leptos::prelude::View::new(
                            ::leptos::tachys::html::element::canvas()
                                .class("fullscreen-canvas")
                                .node_ref(canvas_ref)
                                .on(::leptos::tachys::html::event::keydown, handle_keydown)
                                .on(
                                    ::leptos::tachys::html::event::mousedown,
                                    handle_mousedown,
                                )
                                .on(::leptos::tachys::html::event::mouseup, handle_mouseup)
                                .on(
                                    ::leptos::tachys::html::event::mousemove,
                                    handle_mousemove,
                                )
                                .tabindex("0"),
                        )
                        .with_view_marker("src-components-canvas.rs-178")
                }
            }
        }
    }
    pub use canvas::Canvas;
}
mod types {
    pub mod pixel_canvas {
        use web_sys::CanvasRenderingContext2d;
        pub const PIXEL_SIZE: f64 = 30.0;
        pub const GAP: f64 = 4.0;
        pub const BORDER_RADIUS: f64 = 5.0;
        pub const GRID_SIZE: usize = 100;
        pub const PIXEL_FILL_COLOR: &str = "#eeeeee";
        pub const PIXEL_STROKE_COLOR: &str = "#111111";
        pub const PIXEL_LINE_WIDTH: f64 = 1.0;
        pub const CANVAS_BACKGROUND_COLOR: &str = "#f0f0f0";
        /// 2D position coordinates
        pub struct Position {
            pub x: f64,
            pub y: f64,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Position {
            #[inline]
            fn clone(&self) -> Position {
                Position {
                    x: ::core::clone::Clone::clone(&self.x),
                    y: ::core::clone::Clone::clone(&self.y),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Position {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Position",
                    "x",
                    &self.x,
                    "y",
                    &&self.y,
                )
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Position {
            #[inline]
            fn default() -> Position {
                Position {
                    x: ::core::default::Default::default(),
                    y: ::core::default::Default::default(),
                }
            }
        }
        impl Position {
            pub fn new(x: f64, y: f64) -> Self {
                Self { x, y }
            }
        }
        /// State for the pixel canvas
        pub struct PixelCanvas {
            /// Position offset of the pixel canvas in browser coordinates
            pub position: Position,
            /// Zoom level (1.0 = normal, 2.0 = 2x zoom, etc.)
            pub zoom: f64,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for PixelCanvas {
            #[inline]
            fn clone(&self) -> PixelCanvas {
                PixelCanvas {
                    position: ::core::clone::Clone::clone(&self.position),
                    zoom: ::core::clone::Clone::clone(&self.zoom),
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PixelCanvas {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PixelCanvas",
                    "position",
                    &self.position,
                    "zoom",
                    &&self.zoom,
                )
            }
        }
        impl Default for PixelCanvas {
            fn default() -> Self {
                Self {
                    position: Position::new(0.0, 0.0),
                    zoom: 1.0,
                }
            }
        }
        impl PixelCanvas {
            /// Create a new PixelCanvas with specified position and zoom
            pub fn new(x: f64, y: f64, zoom: f64) -> Self {
                Self {
                    position: Position::new(x, y),
                    zoom,
                }
            }
            /// Draw the pixel canvas grid
            pub fn draw(&self, context: &CanvasRenderingContext2d) {
                let scaled_pixel_size = PIXEL_SIZE * self.zoom;
                let scaled_gap = GAP * self.zoom;
                let scaled_border_radius = BORDER_RADIUS * self.zoom;
                context.set_stroke_style_str(PIXEL_STROKE_COLOR);
                context.set_line_width(PIXEL_LINE_WIDTH);
                for row in 0..GRID_SIZE {
                    for col in 0..GRID_SIZE {
                        let x = self.position.x
                            + (col as f64) * (scaled_pixel_size + scaled_gap);
                        let y = self.position.y
                            + (row as f64) * (scaled_pixel_size + scaled_gap);
                        context.set_fill_style_str(PIXEL_FILL_COLOR);
                        self.draw_rounded_rect(
                            context,
                            x,
                            y,
                            scaled_pixel_size,
                            scaled_pixel_size,
                            scaled_border_radius,
                            true,
                            true,
                        );
                    }
                }
            }
            /// Helper method to draw rounded rectangle with fill and stroke
            fn draw_rounded_rect(
                &self,
                context: &CanvasRenderingContext2d,
                x: f64,
                y: f64,
                width: f64,
                height: f64,
                radius: f64,
                fill: bool,
                stroke: bool,
            ) {
                context.begin_path();
                context.round_rect_with_f64(x, y, width, height, radius).unwrap();
                if fill {
                    context.fill();
                }
                if stroke {
                    context.stroke();
                }
            }
            /// Set position
            pub fn set_position(&mut self, x: f64, y: f64) {
                self.position.x = x;
                self.position.y = y;
            }
            /// Set zoom level
            pub fn set_zoom(&mut self, zoom: f64) {
                self.zoom = zoom.clamp(0.1, 10.0);
            }
            /// Get the total canvas dimensions needed for this grid
            pub fn get_canvas_bounds(&self) -> (f64, f64) {
                let scaled_pixel_size = PIXEL_SIZE * self.zoom;
                let scaled_gap = GAP * self.zoom;
                let width = self.position.x
                    + (GRID_SIZE as f64) * (scaled_pixel_size + scaled_gap);
                let height = self.position.y
                    + (GRID_SIZE as f64) * (scaled_pixel_size + scaled_gap);
                (width, height)
            }
        }
    }
}
mod services {}
use components::Canvas;
/// Props for the [`App`] component.
///
#[builder(crate_module_path = ::leptos::typed_builder)]
#[allow(non_snake_case)]
struct AppProps {}
#[automatically_derived]
impl AppProps {
    /**
                Create a builder for building `AppProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `AppProps`.
                */
    #[allow(dead_code, clippy::default_trait_access)]
    fn builder() -> AppPropsBuilder<()> {
        AppPropsBuilder {
            fields: (),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
struct AppPropsBuilder<TypedBuilderFields = ()> {
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<()>,
}
#[automatically_derived]
impl<TypedBuilderFields> Clone for AppPropsBuilder<TypedBuilderFields>
where
    TypedBuilderFields: Clone,
{
    #[allow(clippy::default_trait_access)]
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl AppPropsBuilder<()> {
    #[allow(
        clippy::default_trait_access,
        clippy::used_underscore_binding,
        clippy::no_effect_underscore_binding
    )]
    pub fn build(self) -> AppProps {
        let () = self.fields;
        #[allow(deprecated)] AppProps {}.into()
    }
}
#[allow(missing_docs)]
impl ::leptos::component::Props for AppProps {
    type Builder = AppPropsBuilder;
    fn builder() -> Self::Builder {
        AppProps::builder()
    }
}
#[allow(non_snake_case, clippy::too_many_arguments)]
#[allow(clippy::needless_lifetimes)]
fn App() -> impl IntoView {
    ::leptos::reactive::graph::untrack_with_diagnostics(move || { __App() })
}
#[doc(hidden)]
#[allow(
    non_snake_case,
    dead_code,
    clippy::too_many_arguments,
    clippy::needless_lifetimes
)]
pub fn __App() -> impl IntoView {
    {
        #[allow(unused_braces)]
        {
            ::leptos::prelude::View::new(
                    ::leptos::tachys::html::element::div()
                        .child(
                            #[allow(unused_braces)]
                            {
                                {
                                    #[allow(unreachable_code)] #[allow(unused_mut)]
                                    #[allow(clippy::let_and_return)]
                                    ::leptos::component::component_view(
                                        #[allow(clippy::needless_borrows_for_generic_args)]
                                        &Canvas,
                                        {
                                            let mut props = ::leptos::component::component_props_builder(
                                                    &Canvas,
                                                )
                                                .build();
                                            props
                                        },
                                    )
                                }
                            },
                        )
                        .class("app"),
                )
                .with_view_marker("src-main.rs-11")
        }
    }
}
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
