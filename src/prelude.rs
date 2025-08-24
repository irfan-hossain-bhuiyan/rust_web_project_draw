use std::{cmp::max, ops::{Index, IndexMut}};

use web_sys::window;

pub fn get_window_size() -> RectSize {
    const DEFAULT_WIDTH: f64 = 800.0;
    const DEFAULT_HEIGHT: f64 = 600.0;

    let Some(win) = window() else {
        web_sys::console::warn_1(&"Window object not available, using default dimensions".into());
        return RectSize::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    };

    let width = win
        .inner_width()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(DEFAULT_WIDTH);

    let height = win
        .inner_height()
        .ok()
        .and_then(|v| v.as_f64())
        .unwrap_or(DEFAULT_HEIGHT);

    RectSize::new(width, height)
}

pub fn get_window_rect() -> Rectangle {
    Rectangle::from_pos_size(Position::zero(), get_window_size())
}
/// 2D vector
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }
    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
/// 2D position coordinates (newtype over Vec2)
#[derive(Copy, Clone, Debug, Default)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
    pub fn from_vec2(v: Vec2) -> Self {
        Self(v)
    }
    pub fn vec(self) -> Vec2 {
        self.0
    }
    pub fn x(self) -> f64 {
        self.0.x
    }
    pub fn y(self) -> f64 {
        self.0.y
    }
    pub fn rect_clamp(self, rect: Rectangle) -> Position {
        Position::new(
            self.x().clamp(rect.ul().x(), rect.dr().x()),
            self.y().clamp(rect.ul().y(), rect.dr().y()),
        )
    }
    //You can think of this like the rectangle ones,clip to value but the rectangle's other point
    //is in infinite
    pub fn pos_clamp(self, pos: Position, x_positive: bool, y_positive: bool) -> Self {
        let pos_x = if x_positive {
            pos.x().max(self.x())
        } else {
            pos.x().min(self.x())
        };
        let pos_y = if y_positive {
            pos.y().max(self.y())
        } else {
            pos.y().min(self.y())
        };
        Self::new(pos_x, pos_y)
    }
    /// Return a new `Position` offset by `delta`.
    pub fn offset(self, delta: Position) -> Position {
        Position(self.0 + delta.0)
    }
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

// Conversions between Position and Vec2
impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Position(v)
    }
}

impl From<Position> for Vec2 {
    fn from(p: Position) -> Self {
        p.0
    }
}

/// Rectangle size with width and height
#[derive(Copy, Clone, Debug, Default)]
pub struct RectSize {
    width: f64, //height and width should always be positive
    height: f64,
}
impl Into<(f64,f64)> for RectSize{
    fn into(self) -> (f64,f64) {
        (self.width,self.height)
    }
}

impl RectSize {
    pub unsafe fn new_unchecked(width: f64, height: f64) -> Self {
        Self { width, height }
    }
    pub fn new(width: f64, height: f64) -> Self {
        let width = width.max(0.0);
        let height = height.max(0.0);
        Self { width, height }
    }
    pub fn from_vec2(v: Vec2) -> Self {
        Self {
            width: v.x.max(0.0),
            height: v.y.max(0.0),
        }
    }
    pub fn into_vec2(self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
    pub fn is_empty(self) -> bool {
        self.width <= 0.0 || self.height <= 0.0
    }
    pub fn can_fit(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
    pub fn scale(&self, factor: f64) -> Self {
        Self::new(self.width * factor, self.height * factor)
    }
}

// Conversions between RectSize and Vec2
impl From<Vec2> for RectSize {
    fn from(v: Vec2) -> Self {
        Self {
            width: v.x,
            height: v.y,
        }
    }
}

impl From<RectSize> for Vec2 {
    fn from(s: RectSize) -> Self {
        Vec2::new(s.width, s.height)
    }
}

pub struct Rectangle {
    ul: Position,
    dr: Position,
}

impl Rectangle {
    /// Create a new rectangle from two corners. Automatically normalizes so that
    /// `ul` is the upper-left (min x,y) and `dr` is the down-right (max x,y).
    pub fn from_point(point1: Position, point2: Position) -> Self {
        let ulx = point1.x().min(point2.x());
        let uly = point1.y().min(point2.y());
        let drx = point1.x().max(point2.x());
        let dry = point1.y().max(point2.y());
        Self {
            ul: Position::new(ulx, uly),
            dr: Position::new(drx, dry),
        }
    }
    //For this function you need to make sure dr is larger than ul
    pub unsafe fn from_ul_dr_unchecked(ul: Position, dr: Position) -> Self {
        Self { ul, dr }
    }

    /// Create rectangle from upper-left position and size
    pub fn from_pos_size(ul: Position, size: RectSize) -> Self {
        let br = Position::new(ul.x() + size.width, ul.y() + size.height);
        // Reuse normalization logic
        unsafe { Self::from_ul_dr_unchecked(ul, br) }
    }

    /// Upper-left corner
    pub fn ul(&self) -> Position {
        self.ul
    }
    /// Upper-right corner
    pub fn ur(&self) -> Position {
        Position::new(self.dr.x(), self.ul.y())
    }
    /// Down-left corner
    pub fn dl(&self) -> Position {
        Position::new(self.ul.x(), self.dr.y())
    }
    /// Down-right corner
    pub fn dr(&self) -> Position {
        self.dr
    }

    /// Width of the rectangle (non-negative)
    pub fn width(&self) -> f64 {
        self.dr.x() - self.ul.x()
    }
    /// Height of the rectangle (non-negative)
    pub fn height(&self) -> f64 {
        self.dr.y() - self.ul.y()
    }

    /// Center point of the rectangle
    pub fn center(&self) -> Position {
        Position::new(
            (self.ul.x() + self.dr.x()) * 0.5,
            (self.ul.y() + self.dr.y()) * 0.5,
        )
    }

    /// Scale the rectangle about its center by `factor` and mutate `self`.
    /// factor > 1.0 expands; factor < 1.0 shrinks.
    pub fn scale(&mut self, factor: f64) {
        let c = self.center();
        let half_w = self.width() * 0.5 * factor;
        let half_h = self.height() * 0.5 * factor;
        let ul = Position::new(c.x() - half_w, c.y() - half_h);
        let dr = Position::new(c.x() + half_w, c.y() + half_h);

        //scaling valid rectangle is still valid
        *self = unsafe { Rectangle::from_ul_dr_unchecked(ul, dr) };
    }

    /// Expand this rectangle to include `other` (mutates to the bounding union).
    pub fn expand_to_include(&mut self, other: Rectangle) {
        let ul = Position::new(self.ul.x().min(other.ul.x()), self.ul.y().min(other.ul.y()));
        let dr = Position::new(self.dr.x().max(other.dr.x()), self.dr.y().max(other.dr.y()));
        *self = unsafe { Rectangle::from_ul_dr_unchecked(ul, dr) };
    }

    /// Size of the rectangle
    pub fn size(&self) -> RectSize {
        RectSize::new(self.width(), self.height())
    }

    /// Given a point, return its relative position in the rectangle as ratios (0..1 range if inside)
    /// where (0,0) is `ul` and (1,1) is `dr`.
    pub fn ratio_of(&self, p: Position) -> Position {
        let w = self.width();
        let h = self.height();
        if w == 0.0 || h == 0.0 {
            return Position::new(0.0, 0.0);
        }
        Position::new((p.x() - self.ul.x()) / w, (p.y() - self.ul.y()) / h)
    }
    pub fn relative_rect(&self, rect: Rectangle) -> Rectangle {
        let ul = self.ratio_of(rect.ul);
        let dr = self.ratio_of(rect.dr);
        unsafe { Rectangle::from_ul_dr_unchecked(ul, dr) }
    }
}


use bitvec::prelude::*;

/// A 2D matrix of bits (true = black, false = white).
#[derive(Clone,Debug)]
pub struct BitMatrix {
    width: usize,
    height: usize,
    data: BitVec, // default Lsb0, usize store
}

impl BitMatrix {
    pub fn new(width: usize, height: usize, initial: bool) -> Self {
        let mut data = BitVec::with_capacity(width * height);
        data.resize(width * height, initial);
        BitMatrix { width, height, data }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn idx_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.idx(x, y).map(|i| self.data[i])
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        if let Some(i) = self.idx(x, y) {
            self.data.set(i, value);
            true
        } else {
            false
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(false);
    }

    pub fn count_ones(&self) -> usize {
        self.data.count_ones()
    }
}

impl Index<(usize, usize)> for BitMatrix {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[self.idx_unchecked(x, y)]
    }
}

/// High-level drawing API for black/white pixel canvas
#[derive(Clone,Debug)]
pub struct DrawingPixelCanvas(BitMatrix);

impl DrawingPixelCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        DrawingPixelCanvas(BitMatrix::new(width, height, false))
    }

    /// Draw a single pixel (true = black, false = white).
    pub fn draw_pixel(&mut self, x: usize, y: usize, black: bool) {
        let _ = self.0.set(x, y, black);
    }

    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, black: bool) {
        let (mut x0, mut y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.draw_pixel(x0 as usize, y0 as usize, black);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Clear canvas (white background)
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// For debugging: dump as ASCII art
    pub fn to_ascii(&self) -> String {
        let (w, h) = self.0.dimensions();
        let mut out = String::new();
        for y in 0..h {
            for x in 0..w {
                out.push(if self.0.get(x, y).unwrap_or(false) { '█' } else { ' ' });
            }
            out.push('\n');
        }
        out
    }
    pub fn get_array(&self)->&BitMatrix{
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_usage() {
        let mut m = BitMatrix::new(4, 3, false);
        assert_eq!(m.dimensions(), (4,3));
        assert_eq!(m.count_ones(), 0);
        assert_eq!(m.get(2,1), Some(false));
        assert!(m.set(2,1, true));
        assert_eq!(m.get(2,1), Some(true));
        assert_eq!(m.count_ones(), 1);

        // out-of-bounds
        assert_eq!(m.get(10,10), None);
        assert!(!m.set(10,0, true));

        // clear
        m.clear();
        assert_eq!(m.count_ones(), 0);

        // row iteration
        m.set(0,2, true);
        
    }
}
