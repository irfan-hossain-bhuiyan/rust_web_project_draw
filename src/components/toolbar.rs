use frontend::prelude::PixelColor;
use leptos::ev;
use leptos::prelude::*;
use leptos::html;

#[derive(Clone, Debug, PartialEq)]

pub enum DrawingTool {
    Pen(PixelColor),
    Eraser,
    // Add more tools later if needed
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


#[component]
pub fn Toolbar(
    #[prop(into)] selected_tool: RwSignal<DrawingTool>,
) -> impl IntoView {
    let tools = vec![DrawingTool::Pen(PixelColor::BLACK), DrawingTool::Eraser];
    
    view! {
        <div class="toolbar">
            <div class="toolbar-title">Drawing Tools</div>
            <div class="tool-buttons">
                {tools.into_iter().map(|tool| {
                    let tool_clone = tool.clone();
                    let tool_for_click = tool.clone();
                    
                    view! {
                        <button
                            class=move || {
                                let current_tool = selected_tool.get();
                                if current_tool == tool_clone {
                                    "tool-button active"
                                } else {
                                    "tool-button"
                                }
                            }
                            on:click=move |_| {
                                selected_tool.set(tool_for_click.clone());
                            }
                            title=tool.name()
                        >
                            <span class="tool-icon">{tool.icon()}</span>
                            <span class="tool-name">{tool.name()}</span>
                        </button>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}

use web_sys::Event;
use web_sys::MouseEvent;

#[component]
pub fn ToolbarWithTrigger(
    #[prop(into)] selected_tool: RwSignal<DrawingTool>,
) -> impl IntoView {
    let show_toolbar = RwSignal::new(false);

    // Track mouse position
    window_event_listener(ev::mousemove, move |ev: MouseEvent| {
        let y = ev.client_y();
        if y < 20 {
            show_toolbar.set(true);
        } else {
            // Only hide if not hovering toolbar
            if !show_toolbar.get() {
                show_toolbar.set(false);
            }
        }
    });

    view! {
        <div
            class=move || if show_toolbar.get() { "toolbar-container show" } else { "toolbar-container" }
            // Keep it visible while hovered
            on:mouseenter=move |_| show_toolbar.set(true)
            on:mouseleave=move |_| show_toolbar.set(false)
        >
            <Toolbar selected_tool=selected_tool />
        </div>
    }
}


