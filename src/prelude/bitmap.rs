use std::ops::Index;

use bitvec::prelude::*;
type RawBitVec = BitVec<u8, Lsb0>;
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
    pub fn as_bytes(&self) -> &BitSlice<u8> {
        &self.data
    }
    pub fn assign_bytes(&mut self, bit_vec: RawBitVec) -> Result<(), String> {
        if self.data.len() == bit_vec.len() {
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
    unsafe fn assign_bytes_unchecked(&mut self, bit_vec: RawBitVec) {
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
}

impl Index<(usize, usize)> for BitMatrix {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        &self.data[self.idx_unchecked(x, y)]
    }
}
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct PixelColor {
    r: bool,
    g: bool,
    b: bool,
    a: bool,
}
/// High-level drawing API for black/white pixel canvas
#[derive(Clone, Debug)]
pub struct DrawingPixelCanvas {
    array0: BitMatrix,
    array1: BitMatrix,
    array2: BitMatrix,
}

impl PixelColor {
    pub const ALPHA: PixelColor = PixelColor::new(true, true, true, false);
    pub const BLACK: PixelColor = PixelColor::new(false, false, false, false);
    pub const fn new(r: bool, g: bool, b: bool, a: bool) -> self {
        Self { r, g, b, a }
    }
    pub const fn color(r: bool, g: bool, b: bool) -> self {
        Self { r, g, b, a: false }
    }
    pub fn to_rgb(&self) -> &'static str {
        if self.a {
            return "#ffffff";
        }
        match (self.r, self.g, self.b) {
            (false, false, false) => "#000000", // Black
            (true, false, false) => "#ff0000",  // Red
            (false, true, false) => "#00ff00",  // Green
            (false, false, true) => "#0000ff",  // Blue
            (true, true, false) => "#ffff00",   // Yellow
            (true, false, true) => "#ff00ff",   // Magenta
            (false, true, true) => "#00ffff",   // Cyan
            (true, true, true) => "#ffffff",    // White (transperant)
        }
    }
    pub fn is_alpha(&self) -> bool {
        self.r & self.g & self.b
    }
}
impl DrawingPixelCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        DrawingPixelCanvas {
            array0: BitMatrix::new(width, height, true),
            array1: BitMatrix::new(width, height, true),
            array2: BitMatrix::new(width, height, true),
        }
    }
    pub fn size(&self) -> (usize, usize) {
        self.array0.dimensions()
    }

    /// Draw a single pixel (true = black, false = white).
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: PixelColor) {
        let _ = self.array0.set(x, y, color.r);
        let _ = self.array1.set(x, y, color.g);
        let _ = self.array2.set(x, y, color.b);
    }
    pub fn layer_overlay(&self, top_layer: &Self) -> DrawingPixelCanvas {
        let mut main = self.clone();
        main.merge_top(top_layer);
        main
    }
    pub fn merge_top(&mut self, top_layer: &Self) {
        let (width, height) = self.array0.dimensions();
        for y in 0..height {
            for x in 0..width {
                let top_pixel = top_layer.get_pixel(x, y);
                if !top_pixel.is_alpha() {
                    self.draw_pixel(x, y, top_pixel);
                }
            }
        }
    }
    /// returns pixel value,
    /// alpha value if out of bound
    pub fn get_pixel(&self, x: usize, y: usize) -> PixelColor {
        let r = self.array0.get(x, y).unwrap_or(true);
        let g = self.array1.get(x, y).unwrap_or(true);
        let b = self.array2.get(x, y).unwrap_or(true);
        PixelColor { r, g, b }
    }
    /// Draw a line using Bresenham's algorithm.
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: PixelColor) {
        let (mut x0, mut y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 {
                self.draw_pixel(x0 as usize, y0 as usize, color);
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
        self.array0.set_to_one();
        self.array1.set_to_one();
        self.array2.set_to_one();
    }

    /// For debugging: dump as ASCII art
    pub fn to_ascii(&self) -> String {
        let (w, h) = self.array0.dimensions();
        let mut out = String::new();
        for y in 0..h {
            for x in 0..w {
                out.push(if self.get_pixel(x, y).is_alpha() {
                    '█'
                } else {
                    ' '
                });
            }
            out.push('\n');
        }
        out
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let array0_bytes = self.array0.as_bytes().to_bitvec().into_vec();
        let array1_bytes = self.array1.as_bytes().to_bitvec().into_vec();
        let array2_bytes = self.array2.as_bytes().to_bitvec().into_vec();

        let mut result =
            Vec::with_capacity(array0_bytes.len() + array1_bytes.len() + array2_bytes.len());
        result.extend_from_slice(&array0_bytes);
        result.extend_from_slice(&array1_bytes);
        result.extend_from_slice(&array2_bytes);

        result
    }
    pub fn assign_bytes(&mut self, data: &[u8]) -> Result<(), String> {
        let data_bits_per_color = data.len() / 3;
        let (width, height) = self.size();
        let total_bits_per_color = width * height;
        // Check if the data size is sufficient
        if data_bits_per_color < total_bits_per_color {
            return Err("byte data is not enough".into());
        }

        // Calculate the number of bits for each array

        // Split the data into three parts
        let (data0, rest) = data.split_at(data_bits_per_color);
        let (data1, data2) = rest.split_at(data_bits_per_color);

        // Assign the bytes to each array
        let mut array0 = RawBitVec::from_vec(data0.to_vec());
        array0.truncate(total_bits_per_color);
        let mut array1 = RawBitVec::from_vec(data1.to_vec());
        array1.truncate(total_bits_per_color);
        let mut array2 = RawBitVec::from_vec(data2.to_vec());
        array2.truncate(total_bits_per_color);

        self.array0.assign_bytes(array0).unwrap();
        self.array1.assign_bytes(array1).unwrap();
        self.array2.assign_bytes(array2).unwrap();

        Ok(())
    }
}
