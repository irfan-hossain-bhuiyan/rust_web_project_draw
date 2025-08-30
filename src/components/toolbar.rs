use frontend::prelude::PixelColor;
use leptos::ev;
use leptos::html;
use leptos::logging::log;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::MouseEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum DrawingTool {
    Pen(PixelColor),
    Eraser,
    BucketFill(PixelColor),
}

impl Default for DrawingTool {
    fn default() -> Self {
        DrawingTool::Pen(PixelColor::BLACK)
    }
}

impl DrawingTool {
    pub fn icon(&self) -> &'static str {
        match self {
            DrawingTool::Pen(_) => "✏️",
            DrawingTool::Eraser => "🧽",
            DrawingTool::BucketFill(_) => "🪣",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            DrawingTool::Pen(_) => "Pen",
            DrawingTool::Eraser => "Eraser",
            DrawingTool::BucketFill(_) => "Bucket Fill",
        }
    }
    pub fn change_color(&mut self, color: PixelColor) {
        *self = match *self {
            DrawingTool::Eraser => DrawingTool::Eraser,
            DrawingTool::Pen(_) => DrawingTool::Pen(color),
            DrawingTool::BucketFill(_) => DrawingTool::BucketFill(color)
        }
    }
}

const COLORS: [(&str, PixelColor); 8] = [
    ("#000000", PixelColor::BLACK),
    ("#ff0000", PixelColor::RED),
    ("#00ff00", PixelColor::GREEN),
    ("#0000ff", PixelColor::BLUE),
    ("#ffff00", PixelColor::YELLOW),
    ("#ff00ff", PixelColor::MAGENTA),
    ("#00ffff", PixelColor::CYAN),
    ("#ffffff", PixelColor::WHITE),
];

#[component]
pub fn Toolbar(#[prop(into)] selected_tool: RwSignal<DrawingTool>) -> impl IntoView {
    let show_color_picker = RwSignal::new(false);
    let color_picker_position = RwSignal::new((0f64, 0f64)); // (left, top) in pixels

    let handle_color_picker = move |ev: MouseEvent, tool: DrawingTool| {
        ev.prevent_default();
        
        // Get the button element that was right-clicked
        if let Some(button) = ev.target().and_then(|t| {
            // Try to get the button element (might be the button itself or a child span)
            if let Ok(element) = t.dyn_into::<web_sys::Element>() {
                if element.tag_name() == "BUTTON" {
                    Some(element)
                } else {
                    element.closest("button").ok().flatten()
                }
            } else {
                None
            }
        }) {
            let rect = button.get_bounding_client_rect();
            let toolbar_rect = button.closest(".toolbar")
                .ok()
                .flatten()
                .map(|toolbar| toolbar.get_bounding_client_rect());
            
            if let Some(toolbar_rect) = toolbar_rect {
                // Calculate position relative to toolbar
                let left = rect.left() - toolbar_rect.left() + (rect.width() / 2.0) - 75.0; 
                // Center under button, 60px is half of color picker width
                let top = rect.bottom() - toolbar_rect.top() + 5.0; // 5px gap below button
                
                color_picker_position.set((left, top));
            }
        }
        
        // Set the tool and show color picker
        selected_tool.set(tool);
        show_color_picker.set(!show_color_picker.get());
    };

    view! {
        <div class="toolbar">
            <div class="toolbar-title">Drawing Tools</div>
            <div class="tool-buttons">
                // Pen button
                <button
                    class=move || {
                        if matches!(selected_tool.get(), DrawingTool::Pen(_)) {
                            "tool-button active"
                        } else {
                            "tool-button"
                        }
                    }
                    on:click=move |_| {
                        selected_tool.set(DrawingTool::Pen(PixelColor::BLACK));
                    }
                    on:contextmenu=move |ev: MouseEvent| {
                        handle_color_picker(ev, DrawingTool::Pen(PixelColor::BLACK));
                    }
                >
                    <span class="tool-icon">"✏️"</span>
                    <span class="tool-name">"Pen"</span>
                </button>

                // Bucket Fill button
                <button
                    class=move || {
                        if matches!(selected_tool.get(), DrawingTool::BucketFill(_)) {
                            "tool-button active"
                        } else {
                            "tool-button"
                        }
                    }
                    on:click=move |_| {
                        selected_tool.set(DrawingTool::BucketFill(PixelColor::BLACK));
                    }
                    on:contextmenu=move |ev: MouseEvent| {
                        log!("right click pressed on bucket fill.");
                        handle_color_picker(ev, DrawingTool::BucketFill(PixelColor::BLACK));
                    }
                >
                    <span class="tool-icon">"🪣"</span>
                    <span class="tool-name">"Fill"</span>
                </button>

                // Eraser button
                <button
                    class=move || {
                        if matches!(selected_tool.get(), DrawingTool::Eraser) {
                            "tool-button active"
                        } else {
                            "tool-button"
                        }
                    }
                    on:click=move |_| {
                        selected_tool.set(DrawingTool::Eraser);
                    }
                >
                    <span class="tool-icon">"🧽"</span>
                    <span class="tool-name">"Eraser"</span>
                </button>
            </div>

            // Color picker (dynamically positioned)
            <div
                class=move || if show_color_picker.get() { "color-picker show" } else { "color-picker" }
                style=move || {
                    let (left, top) = color_picker_position.get();
                    format!(
                        "position: absolute; left: {left}px; top: {top}px;",
                    )
                }
            >
                <div class="color-grid">
                    {COLORS.iter().map(|(hex, pixel_color)| {
                        let color = *pixel_color;
                        let hex_str = hex.to_string();
                        view! {
                            <button
                                class="color-button"
                                style=move || format!("background-color: {hex_str}")
                                on:click=move |_| {
                                    selected_tool.update(|x| x.change_color(color));
                                    show_color_picker.set(false);
                                }
                            />
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ToolbarWithTrigger(#[prop(into)] selected_tool: RwSignal<DrawingTool>) -> impl IntoView {
    let show_toolbar = RwSignal::new(false);

    window_event_listener(ev::mousemove, move |ev: MouseEvent| {
        let y = ev.client_y();
        if y < 20 {
            show_toolbar.set(true);
        } else if !show_toolbar.get() {
            show_toolbar.set(false);
        }
    });

    view! {
        <div
            class=move || if show_toolbar.get() { "toolbar-container show" } else { "toolbar-container" }
            on:mouseenter=move |_| show_toolbar.set(true)
            on:mouseleave=move |_| show_toolbar.set(false)
        >
            <Toolbar selected_tool=selected_tool />
        </div>
    }
}
