use frontend::prelude::PixelColor;
use leptos::ev;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::html;
use web_sys::MouseEvent;

#[derive(Clone, Debug, PartialEq)]
pub enum DrawingTool {
    Pen(PixelColor),
    Eraser,
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
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            DrawingTool::Pen(_) => "Pen",
            DrawingTool::Eraser => "Eraser",
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
                        ev.prevent_default();
                        log!("right click pressed.");
                        show_color_picker.set(!show_color_picker.get());
                    }
                >
                    <span class="tool-icon">"✏️"</span>
                    <span class="tool-name">"Pen"</span>
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
            
            // Color picker
            <div 
                class=move || if show_color_picker.get() { "color-picker show" } else { "color-picker" }
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
                                    selected_tool.set(DrawingTool::Pen(color));
                                    log!("color picked {color:?}");
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
