use std::{
    collections::VecDeque,
    ops::{BitAndAssign, BitOr, BitOrAssign, BitXorAssign, Index, Not},
};

use bitvec::prelude::*;
use leptos::logging::log;
type RawBitVec = BitVec<u8, Lsb0>;
type RawBitSlice = BitSlice<u8, Lsb0>;

/// Returns `a` if `c` is 0 (false), else selects `b`.
///
/// # Panics
/// - if lengths don’t match.
///
/// # Examples
/// ```
/// use bitvec::prelude::*;
/// let a = bitvec![0, 1, 0, 1];
/// let b = bitvec![1, 1, 0, 0];
/// let c = bitvec![0, 1, 0, 0];
/// let out = select_bits(&a, &b, &c);
/// assert_eq!(out, bitvec![0, 1, 0, 1]);
/// ```
pub fn select_bits(
    a: &BitSlice<u8, Lsb0>,
    b: &BitSlice<u8, Lsb0>,
    c: &BitSlice<u8, Lsb0>,
) -> BitVec<u8, Lsb0> {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());
    // A ^ (C & (A ^ B))  -- fewer temporaries and correct
    let mut out = a.to_bitvec();
    let mut tmp = out.clone(); // tmp = A
    tmp ^= b; // tmp = A ^ B
    tmp &= c.to_bitvec(); // tmp = (A ^ B) & C
    out ^= tmp; // out = A ^ tmp = A ^ ((A^B)&C) -> desired
    out
}
// Note: depending on availability of `bitnot_assign()` on BitVec, you may need to invert per-bit by `!` mapping or manually.
/// erase_bit ans=a if !c else a & ~b
/// A 2D matrix of bits (true = black, false = white).
#[derive(Clone, Debug)]
pub struct BitMatrix {
    width: usize,
    height: usize,
    data: RawBitVec, // default Lsb0, usize store
}

impl BitMatrix {
    pub fn new(width: usize, height: usize, initial: bool) -> Self {
        let mut data = BitVec::with_capacity(width * height);
        data.resize(width * height, initial);
        BitMatrix {
            width,
            height,
            data,
        }
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
    unsafe fn idx_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.idx(x, y).map(|i| self.data[i])
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) -> Result<(), String> {
        if let Some(i) = self.idx(x, y) {
            self.data.set(i, value);
            Ok(())
        } else {
            Err(format!(
                "Out of boundary {x} x {y} index for size {} x {}",
                self.width, self.height
            ))
        }
    }
    pub fn as_bytes(&self) -> &BitSlice<u8> {
        &self.data
    }

    /// truncate the bitvec if the size is bugger than necessary.
    pub fn assign_bits(&mut self, bit_vec: RawBitVec) -> Result<(), String> {
        let mut bit_vec = bit_vec;
        if self.data.len() <= bit_vec.len() {
            bit_vec.truncate(self.data.len());
            self.data = bit_vec;
            return Ok(());
        }
        Err(format!(
            "Dimension size mismatch {}!={}",
            self.data.len(),
            bit_vec.len()
        ))
    }
    /// #Safety
    /// bit_vec must be equal to data vec size
    unsafe fn assign_bits_unchecked(&mut self, bit_vec: RawBitVec) {
        self.data = bit_vec;
    }

    pub fn set_to_one(&mut self) {
        self.data.fill(true);
    }
    pub fn clear(&mut self) {
        self.data.fill(false);
    }

    pub fn count_ones(&self) -> usize {
        self.data.count_ones()
    }

    fn area(&self) -> usize {
        self.data.len()
    }
    /// Required bytes to load the pixel values
    pub fn required_bytes(&self) -> usize {
        let area = self.area();
        (area + 7) / 8
    }
    pub fn from_bytes<'a>(&mut self, bytes: &'a [u8]) -> Result<&'a [u8], String> {
        let Some((head, tail)) = bytes.split_at_checked(self.required_bytes()) else {
            return Err("bytes are of less sized to convert".to_owned());
        };
        self.assign_bits(RawBitVec::from_slice(head)).unwrap();
        Ok(tail)
    }
    pub fn to_bytes(&self) -> &[u8] {
        self.data.as_raw_slice()
    }
}

impl Index<(usize, usize)> for BitMatrix {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[self.idx(x, y).unwrap()]
    }
}

pub struct BitMatrixIter<'a> {
    matrix: &'a BitMatrix,
    index: usize,
}

impl<'a> Iterator for BitMatrixIter<'a> {
    type Item = (usize, usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.matrix.data.len() {
            return None;
        }

        let idx = self.index;
        let x = idx % self.matrix.width;
        let y = idx / self.matrix.width;
        let value = self.matrix.data[idx];

        self.index += 1;

        Some((x, y, value))
    }
}

impl BitMatrix {
    pub fn iter(&self) -> BitMatrixIter<'_> {
        BitMatrixIter {
            matrix: self,
            index: 0,
        }
    }
}

/// Allow `for (x, y, val) in &matrix`
impl<'a> IntoIterator for &'a BitMatrix {
    type Item = (usize, usize, bool);
    type IntoIter = BitMatrixIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitMatrixIter {
            matrix: self,
            index: 0,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct PixelColor {
    pub r: bool,
    pub g: bool,
    pub b: bool,
    pub a: bool,
}

impl PixelColor {
    pub const ALPHA: PixelColor = PixelColor::new(false, false, false, false);
    pub const BLACK: PixelColor = PixelColor::new(false, false, false, true);
    pub const RED: PixelColor = PixelColor::new(true, false, false, true);
    pub const GREEN: PixelColor = PixelColor::new(false, true, false, true);
    pub const BLUE: PixelColor = PixelColor::new(false, false, true, true);
    pub const YELLOW: PixelColor = PixelColor::new(true, true, false, true);
    pub const MAGENTA: PixelColor = PixelColor::new(true, false, true, true);
    pub const CYAN: PixelColor = PixelColor::new(false, true, true, true);
    pub const WHITE: PixelColor = PixelColor::new(true, true, true, true);
    pub const ERASE: PixelColor = PixelColor::new(true, false, false, false);
    pub const fn new(r: bool, g: bool, b: bool, a: bool) -> Self {
        Self { r, g, b, a }
    }
    pub const fn color(r: bool, g: bool, b: bool) -> Self {
        Self { r, g, b, a: false }
    }
    pub fn to_rgb(&self) -> &'static str {
        if self.is_transperent() {
            return "#dddddd";
        }
        match (self.r, self.g, self.b) {
            (false, false, false) => "#000000", // Black
            (true, false, false) => "#ff0000",  // Red
            (false, true, false) => "#00ff00",  // Green
            (false, false, true) => "#0000ff",  // Blue
            (true, true, false) => "#ffff00",   // Yellow
            (true, false, true) => "#ff00ff",   // Magenta
            (false, true, true) => "#00ffff",   // Cyan
            (true, true, true) => "#ffffff",    // White
        }
    }
    pub fn is_transperent(&self) -> bool {
        !self.a
    }
    pub fn is_eraser(&self) -> bool {
        *self
            == Self {
                r: true,
                g: false,
                b: false,
                a: false,
            }
    }
}

pub struct DrawingPixelCanvasIter<'a> {
    canvas: &'a DrawingPixelCanvas,
    x: usize,
    y: usize,
}

impl<'a> DrawingPixelCanvasIter<'a> {
    fn new(canvas: &'a DrawingPixelCanvas) -> Self {
        DrawingPixelCanvasIter { canvas, x: 0, y: 0 }
    }
}

impl<'a> Iterator for DrawingPixelCanvasIter<'a> {
    type Item = (usize, usize, PixelColor);

    fn next(&mut self) -> Option<Self::Item> {
        let (width, height) = self.canvas.dimension();

        if self.y >= height {
            return None;
        }

        let color = self.canvas.get_pixel(self.x, self.y);
        let item = (self.x, self.y, color);

        // advance position
        self.x += 1;
        if self.x >= width {
            self.x = 0;
            self.y += 1;
        }

        Some(item)
    }
}

/// High-level drawing API for black/white pixel canvas
#[derive(Clone, Debug)]
pub struct DrawingPixelCanvas {
    r_array: BitMatrix,
    g_array: BitMatrix,
    b_array: BitMatrix,
    a_array: BitMatrix,
}

impl DrawingPixelCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        DrawingPixelCanvas {
            r_array: BitMatrix::new(width, height, false),
            g_array: BitMatrix::new(width, height, false),
            b_array: BitMatrix::new(width, height, false),
            a_array: BitMatrix::new(width, height, false),
        }
    }
    pub fn iter(&self) -> DrawingPixelCanvasIter<'_> {
        DrawingPixelCanvasIter::new(self)
    }
    pub fn dimension(&self) -> (usize, usize) {
        self.r_array.dimensions()
    }
    pub fn equals(&self, other: &Self) -> bool {
        if self.dimension() != other.dimension() {
            return false;
        }

        self.r_array.as_bytes() == other.r_array.as_bytes()
            && self.g_array.as_bytes() == other.g_array.as_bytes()
            && self.b_array.as_bytes() == other.b_array.as_bytes()
            && self.a_array.as_bytes() == other.a_array.as_bytes()
    }
    /// Draw a single pixel (true = black, false = white).
    pub fn draw_pixel_ignore(&mut self, x: usize, y: usize, color: PixelColor) {
        let _ = self.r_array.set(x, y, color.r);
        let _ = self.g_array.set(x, y, color.g);
        let _ = self.b_array.set(x, y, color.b);
        let _ = self.a_array.set(x, y, color.a);
        if let Some(x)=self.a_array.get(x,y){
            assert_eq!(x,color.a);
        }
    }
    pub fn is_transpernet_debug(&self) -> bool {
        !self.a_array.as_bytes().any()
    }
    pub fn bucket_fill(
        &mut self,
        start_x: usize,
        start_y: usize,
        new_color: PixelColor,
        reference_canvas: &DrawingPixelCanvas,
    ) {
        let (w, h) = self.dimension();

        if start_x >= w || start_y >= h {
            return;
        }

        // Get the target color (the region to replace)
        let target_color = reference_canvas.get_pixel(start_x, start_y);

        // If target color == new color, no need to fill
        if target_color == new_color {
            return;
        }

        let mut visited = BitMatrix::new(w, h, false);
        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y));
        visited.set(start_x, start_y, true).unwrap();

        while let Some((x, y)) = queue.pop_front() {
            // Only fill if pixel matches the target color
            if reference_canvas.get_pixel(x, y) == target_color {
                self.draw_pixel_ignore(x, y, new_color);

                // 4-connected neighbors
                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx < w && ny < h && !visited.get(nx, ny).unwrap() {
                        visited.set(nx, ny, true).unwrap();
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }
    pub fn non_transperent_value(&self) -> std::option::Option<(usize, usize)> {
        for (x, y, b) in self.a_array.iter() {
            if b {
                return Some((x, y));
            }
        }
        None
    }
    pub fn search_color(&self, color: PixelColor) -> Option<(usize, usize)> {
        for (x, y, c) in self.iter() {
            if c == color {
                return Some((x, y));
            }
        }
        None
    }
    pub fn merge_top(&mut self, top_layer: &Self) {
        assert_eq!(self.dimension(), top_layer.dimension());

        let (w, h) = self.dimension();
        for y in 0..h {
            for x in 0..w {
                let top = top_layer.get_pixel(x, y);

                if top == PixelColor::ERASE {
                    // special erase: clear this pixel
                    self.draw_pixel_ignore(x, y, PixelColor::ALPHA);
                } else if top.a {
                    // normal blending: replace bottom with top
                    self.draw_pixel_ignore(x, y, top);
                }
                // else: leave bottom unchanged
            }
        }
    }

    pub fn layer_overlay(&self, top_layer: &Self) -> DrawingPixelCanvas {
        let mut main = self.clone();
        main.merge_top(top_layer);
        main
    }

    /// returns pixel value,
    /// alpha value if out of bound
    pub fn get_pixel(&self, x: usize, y: usize) -> PixelColor {
        let r = self.r_array.get(x, y).unwrap_or(false);
        let g = self.g_array.get(x, y).unwrap_or(false);
        let b = self.b_array.get(x, y).unwrap_or(false);
        let a = self.a_array.get(x, y).unwrap_or(false);
        PixelColor { r, g, b, a }
    }
    pub fn get_pixel_checked(&self, x: usize, y: usize) -> Option<PixelColor> {
        let r = self.r_array.get(x, y)?;
        let g = self.g_array.get(x, y)?;
        let b = self.b_array.get(x, y)?;
        let a = self.a_array.get(x, y)?;
        Some(PixelColor { r, g, b, a })
    }
    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: PixelColor) {
        if (x0, y0) == (x1, y1) {
            self.draw_pixel_ignore(x0, y0, color);
            return;
        }

        let (mut x0, mut y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.draw_pixel_ignore(x0 as usize, y0 as usize, color);
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

    /// Clear canvas (alpha background)
    pub fn clear(&mut self) {
        self.r_array.clear();
        self.g_array.clear();
        self.b_array.clear();
        self.a_array.clear();
    }

    /// For debugging: dump as ASCII art
    pub fn to_ascii(&self) -> String {
        let (w, h) = self.r_array.dimensions();
        let mut out = String::new();
        for y in 0..h {
            for x in 0..w {
                out.push(if self.get_pixel(x, y).is_transperent() {
                    ' '
                } else {
                    '8'
                });
            }
            out.push('\n');
        }
        out
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.r_array.area() * 3);
        result.extend_from_slice(self.r_array.to_bytes());
        result.extend_from_slice(self.g_array.to_bytes());
        result.extend_from_slice(self.b_array.to_bytes());

        result
    }
    pub fn assign_bytes<'a>(&mut self, data: &'a [u8]) -> Result<&'a [u8], String> {
        if self.a_array.required_bytes() * 3 <= data.len() {
            return Err("not enough data for pixel canvas".to_owned());
        }
        let tail = self.r_array.from_bytes(data).unwrap();
        let tail = self.g_array.from_bytes(tail).unwrap();
        let tail = self.b_array.from_bytes(tail).unwrap();
        let tail = self.a_array.from_bytes(tail).unwrap();
        Ok(tail)
    }

    pub fn get_alpha(&self) -> &BitMatrix {
        &self.a_array
    }
}
