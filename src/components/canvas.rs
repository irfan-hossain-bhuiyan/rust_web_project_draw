use leptos::prelude::*;

#[component]
pub fn Canvas() -> impl IntoView {
    // Create a 50x50 grid of pixels
    let grid_size = 1000;
    
    // Generate the pixel grid
    let pixels = (0..grid_size * grid_size)
        .map(|i| {
            let row = i / grid_size;
            let col = i % grid_size;
            
            view! {
                <div 
                    class="pixel"
                    data-row={row}
                    data-col={col}
                >
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="canvas-container">
            <h1 class="canvas-title">"Collaborative Pixel Canvas"</h1>
            <div class="canvas-grid">
                {pixels}
            </div>
        </div>
    }
}