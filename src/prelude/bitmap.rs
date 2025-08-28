use std::ops::Index;

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


