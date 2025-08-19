use web_sys::CanvasRenderingContext2d;

// Constants for pixel canvas styling
pub const PIXEL_SIZE: f64 = 30.0;
pub const GAP: f64 = 4.0;
pub const BORDER_RADIUS: f64 = 5.0;
pub const GRID_SIZE: usize = 100;
pub const PIXEL_FILL_COLOR: &str = "#eeeeee";
pub const PIXEL_STROKE_COLOR: &str = "#111111";
pub const PIXEL_LINE_WIDTH: f64 = 1.0;
pub const CANVAS_BACKGROUND_COLOR: &str = "#f0f0f0";

/// 2D position coordinates
#[derive(Clone, Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// State for the pixel canvas
#[derive(Clone, Debug)]
pub struct PixelCanvas {
    /// Position offset of the pixel canvas in browser coordinates
    pub position: Position,
    /// Zoom level (1.0 = normal, 2.0 = 2x zoom, etc.)
    pub zoom: f64,
}

impl Default for PixelCanvas {
    fn default() -> Self {
        Self {
            position: Position::new(20.0, 20.0),
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
        // Apply zoom transformations
        let scaled_pixel_size = PIXEL_SIZE * self.zoom;
        let scaled_gap = GAP * self.zoom;
        let scaled_border_radius = BORDER_RADIUS * self.zoom;
        
        // Set up stroke properties once
        context.set_stroke_style_str(PIXEL_STROKE_COLOR);
        context.set_line_width(PIXEL_LINE_WIDTH);
        
        // Draw grid
        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let x = self.position.x + (col as f64) * (scaled_pixel_size + scaled_gap);
                let y = self.position.y + (row as f64) * (scaled_pixel_size + scaled_gap);
                
                // Set fill color
                context.set_fill_style_str(PIXEL_FILL_COLOR);
                
                // Draw filled rounded rectangle
                self.draw_rounded_rect(context, x, y, scaled_pixel_size, scaled_pixel_size, scaled_border_radius);
                
                // Draw border
                context.begin_path();
                context.round_rect_with_f64(x, y, scaled_pixel_size, scaled_pixel_size, scaled_border_radius).unwrap();
                context.stroke();
            }
        }
    }

    /// Helper method to draw rounded rectangle
    fn draw_rounded_rect(
        &self,
        context: &CanvasRenderingContext2d,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        radius: f64,
    ) {
        context.begin_path();
        context.round_rect_with_f64(x, y, width, height, radius).unwrap();
        context.fill();
    }

    /// Set position
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position.x = x;
        self.position.y = y;
    }

    /// Set zoom level
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.max(0.1).min(10.0); // Clamp between 0.1x and 10x
    }

    /// Get the total canvas dimensions needed for this grid
    pub fn get_canvas_bounds(&self) -> (f64, f64) {
        let scaled_pixel_size = PIXEL_SIZE * self.zoom;
        let scaled_gap = GAP * self.zoom;
        
        let width = self.position.x + (GRID_SIZE as f64) * (scaled_pixel_size + scaled_gap);
        let height = self.position.y + (GRID_SIZE as f64) * (scaled_pixel_size + scaled_gap);
        
        (width, height)
    }
}