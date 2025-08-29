use frontend::prelude::PixelColor;
use leptos::logging::log;
use web_sys::CanvasRenderingContext2d;

use crate::prelude::{
    DrawingPixelCanvas, Position, RectSize, Rectangle, Vec2, get_window_rect, get_window_size,
};

// Constants for pixel canvas styling
pub const PIXEL_SIZE: f64 = 30.0;
pub const GAP: f64 = 4.0;
pub const BORDER_RADIUS: f64 = 5.0;
pub const GRID_SIZE: usize = 10;
pub const PIXEL_FILL_COLOR: &str = "#dddddd";
pub const PIXEL_HOVER_COLOR: &str = "#bbbbbb";
pub const PIXEL_STROKE_COLOR: &str = "#111111";
pub const PIXEL_LINE_WIDTH: f64 = 1.0;
pub const CANVAS_BACKGROUND_COLOR: &str = "#f0f0f0";
#[derive(Clone, Debug)]
pub struct GridIndex {
    pub x: usize,
    pub y: usize,
}

/// State for the pixel canvas
#[derive(Clone, Debug)]
pub struct PixelCanvas {
    /// Position offset of the pixel canvas in browser coordinates
    position: Position, //position not being public as position can't be positive,as position is
    //limited by bound checking
    /// Zoom level (1.0 = normal, 2.0 = 2x zoom, etc.)
    zoom: f64,
    /// Drawing canvas for pixel data
    main_canvas: DrawingPixelCanvas,
    drawing_canvas: DrawingPixelCanvas,
    temp_canvas: DrawingPixelCanvas,
}

impl Default for PixelCanvas {
    fn default() -> Self {
        Self {
            position: Position::new(-20.0, -20.0),
            zoom: 1.0,
            main_canvas: DrawingPixelCanvas::new(GRID_SIZE, GRID_SIZE),
            drawing_canvas: DrawingPixelCanvas::new(GRID_SIZE, GRID_SIZE),
            temp_canvas: DrawingPixelCanvas::new(GRID_SIZE, GRID_SIZE),
        }
    }
}

impl PixelCanvas {
    /// Create a new PixelCanvas with specified position and zoom
    pub fn new(x: f64, y: f64, zoom: f64) -> Self {
        Self {
            position: Position::new(x, y),
            zoom,
            main_canvas: DrawingPixelCanvas::new(GRID_SIZE, GRID_SIZE),
            drawing_canvas: DrawingPixelCanvas::new(GRID_SIZE, GRID_SIZE),
            temp_canvas: DrawingPixelCanvas::new(GRID_SIZE, GRID_SIZE),
        }
    }

    /// Get mutable reference to drawing canvas
    pub fn assign_pixel_bytes<'a>(&mut self, data: &'a[u8]) -> Result<&'a[u8], String> {
        self.main_canvas.assign_bytes(data)
    }
    pub fn update_drawing(&mut self){
        self.main_canvas.merge_top(&self.drawing_canvas);
        self.drawing_canvas.clear();
    }
    pub fn to_bytes(&self)->Vec<u8>{
        self.main_canvas.to_bytes()
    }
    pub fn rendered_canvas(&self) -> DrawingPixelCanvas {
        self.main_canvas
            .layer_overlay(&self.drawing_canvas)
            .layer_overlay(&self.temp_canvas)
    }
    /// Implement lineDraw for PixelCanvas as requested
    pub fn line_draw(&mut self, pos1: GridIndex, pos2: GridIndex, color: PixelColor) {
        self.drawing_canvas
            .draw_line(pos1.x, pos1.y, pos2.x, pos2.y, color);

        log!("{}",self.drawing_canvas.to_ascii());
    }
    pub fn set_position(&mut self, x: f64, y: f64) {
        self.position = Position::new(x, y);
        self.clamp_position();
    }
    pub fn x_shift(&mut self, x: f64) {
        self.position.0.x += x;
        self.clamp_position();
    }
    pub fn y_shift(&mut self, y: f64) {
        self.position.0.y += y;
        self.clamp_position();
    }
    fn clamp_position(&mut self) {
        let max_pos = Position::zero();
        let min_pos = Position::from(get_window_size().into_vec2() - self.get_size().into_vec2());
        let min_pos = min_pos.pos_clamp(max_pos, false, false);
        self.position = self
            .position
            .rect_clamp(unsafe { Rectangle::from_ul_dr_unchecked(min_pos, max_pos) });
    }
    pub fn get_rect(&self) -> Rectangle {
        Rectangle::from_pos_size(self.position, self.get_size())
    }
    /// Draw the pixel canvas grid
    pub fn draw(&self, context: &CanvasRenderingContext2d, mouse_pos: Option<Position>) {
        // Apply zoom transformations
        let scaled_pixel_size = PIXEL_SIZE * self.zoom;
        let scaled_gap = GAP * self.zoom;
        let scaled_border_radius = BORDER_RADIUS * self.zoom;
        let rendered_canvas = self.drawing_canvas.clone();
        // Set up stroke properties once
        context.set_stroke_style_str(PIXEL_STROKE_COLOR);
        context.set_line_width(PIXEL_LINE_WIDTH);

        // Calculate hovered pixel if mouse position is provided
        let hovered_grid = mouse_pos.map(|pos| self.closest_grid_index_from_point(pos));

        let (index_ul, index_dr) = self.viewed_row_coloumed();
        // Draw grid
        for row in index_ul.y..index_dr.y {
            for col in index_ul.x..index_dr.x {
                let x = self.position.x() + (col as f64) * (scaled_pixel_size + scaled_gap);
                let y = self.position.y() + (row as f64) * (scaled_pixel_size + scaled_gap);

                // Set fill color based on whether this pixel is hovered and pixel data
                let is_hovered = hovered_grid
                    .as_ref()
                    .map(|grid| grid.x == col && grid.y == row)
                    .unwrap_or(false);

                // Check if this pixel is black in the drawing canvas
                let pixel_color = rendered_canvas.get_pixel(col, row);

                let fill_color = if is_hovered {
                    PIXEL_HOVER_COLOR
                } else {
                    pixel_color.to_rgb()
                };
                context.set_fill_style_str(fill_color);

                // Draw filled rounded rectangle
                self.draw_rounded_rect(
                    context,
                    x,
                    y,
                    scaled_pixel_size,
                    scaled_pixel_size,
                    scaled_border_radius,
                );

                // Draw border
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
        context
            .round_rect_with_f64(x, y, width, height, radius)
            .unwrap();
        context.stroke();
        context.fill();
    }

    /// Set position
    fn zoom_clamp(&mut self) {
        const ZOOM_MAX: f64 = 2.0;
        const ZOOM_MIN: f64 = 0.6;
        self.zoom = self.zoom.clamp(ZOOM_MIN, ZOOM_MAX);
    }
    /// Set zoom level
    pub fn set_zoom(&mut self, zoom_value: f64) {
        self.zoom = zoom_value;
        self.zoom_clamp();
    }
    pub fn zoom_in(&mut self, factor: f64) {
        self.zoom *= factor;
        self.zoom_clamp();
    }

    pub fn zoom_out(&mut self, factor: f64) {
        self.zoom /= factor;
        self.zoom_clamp();
    }

    /// Zoom in/out from a specific point (keeping that point stationary)
    pub fn zoom_at_point(&mut self, factor: f64, point_x: f64, point_y: f64) {
        // Apply zoom
        self.zoom *= factor;
        self.zoom_clamp();

        // Calculate the actual zoom change after clamping
        let actual_zoom_change = factor;
        // Adjust position to keep the point under cursor stationary
        // Formula: new_pos = point - (point - old_pos) * zoom_ratio
        let new_x = point_x - (point_x - self.position.x()) * actual_zoom_change;
        let new_y = point_y - (point_y - self.position.y()) * actual_zoom_change;
        self.position = Position::new(new_x, new_y);
        self.clamp_position();
    }
    /// Get the total canvas dimensions needed for this grid
    pub fn get_size(&self) -> RectSize {
        self.get_unzoomed_size().scale(self.zoom)
    }
    pub fn get_unzoomed_size(&self) -> RectSize {
        let width = (GRID_SIZE as f64) * (PIXEL_SIZE + GAP);
        let height = (GRID_SIZE as f64) * (PIXEL_SIZE + GAP);

        unsafe { RectSize::new_unchecked(width, height) }
    }
    fn viewed_row_coloumed(&self) -> (GridIndex, GridIndex) {
        let win_rect = get_window_rect();
        let canvas_rect = self.get_rect();
        let relative_rectangle = canvas_rect.relative_rect(win_rect);
        let grid_index_ul = GridIndex {
            x: (relative_rectangle.ul().x() * GRID_SIZE as f64).floor() as usize,
            y: (relative_rectangle.ul().y() * GRID_SIZE as f64).floor() as usize,
        };
        let grid_index_dr = GridIndex {
            x: (relative_rectangle.dr().x() * GRID_SIZE as f64).ceil() as usize,
            y: (relative_rectangle.dr().y() * GRID_SIZE as f64).ceil() as usize,
        };
        (grid_index_ul, grid_index_dr)
    }
    pub fn closest_grid_index_from_point(&self, pos: Position) -> GridIndex {
        let relative_pos = self.get_rect().ratio_of(pos).0;
        GridIndex {
            x: (GRID_SIZE as f64 * relative_pos.x).floor() as usize,
            y: (GRID_SIZE as f64 * relative_pos.y).floor() as usize,
        }
    }
}
