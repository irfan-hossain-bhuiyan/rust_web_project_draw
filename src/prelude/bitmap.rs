use std::ops::{BitAndAssign, BitOrAssign, BitXorAssign, Index, Not};

use bitvec::prelude::*;
type RawBitVec = BitVec<u8, Lsb0>;
type RawBitSlice = BitSlice<u8, Lsb0>;

/// Computes per-bit f(a0, a1, r1) = 0 if (a1 = 0 and r1 = 1), else (a0 OR a1)
///
/// Boolean expression per bit:
/// ```text
/// f = ¬(¬a1 ∧ r1) ∧ (a0 ∨ a1)
///    ≡ a1 ∨ (a0 ∧ ¬r1)
/// ```
/// Simplified: **f = a1 | (a0 & !r1)**
///
/// # Panics
/// Panics if the input slices have different lengths.
///
/// # Examples
/// ```
/// use bitvec::prelude::*;
/// let a0 = bitvec![0, 1, 0, 1];
/// let a1 = bitvec![0, 0, 1, 1];
/// let r1 = bitvec![1, 0, 1, 0];
/// let out = custom_bits(&a0, &a1, &r1);
/// assert_eq!(out, bitvec![0, 1, 1, 1]);
/// ```
fn alpha_bits(
    a0: &BitSlice<u8, Lsb0>,
    a1: &BitSlice<u8, Lsb0>,
    r1: &BitSlice<u8, Lsb0>,
) -> BitVec<u8, Lsb0> {
    assert_eq!(a0.len(), a1.len());
    assert_eq!(a0.len(), r1.len());

    let mut out = a1.to_bitvec(); // start with a1
    let mut tmp = a0.to_bitvec(); // copy a0

    // invert r1 bits
    tmp &= !r1.to_bitvec(); // tmp = a0 & !r1
    out |= tmp; // out = a1 | (a0 & !r1)

    out
}

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
fn select_bits(
    a: &BitSlice<u8, Lsb0>,
    b: &BitSlice<u8, Lsb0>,
    c: &BitSlice<u8, Lsb0>,
) -> BitVec<u8, Lsb0> {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());

    let mut ans = a.to_bitvec();
    ans ^= b.to_bitvec();
    ans &= c.to_bitvec();
    ans ^= a.to_bitvec();

    ans
}

/// Returns `a` if `c` is 0, else returns `a & !b`.
///
/// # Panics
/// - if lengths don’t match.
///
/// # Examples
/// ```
/// use bitvec::prelude::*;
/// let a = bitvec![1, 1, 0, 1];
/// let b = bitvec![0, 1, 1, 0];
/// let c = bitvec![0, 1, 0, 1];
/// let out = erase_bit(&a, &b, &c);
/// assert_eq!(out, bitvec![1, 0, 0, 1]);
/// ```
fn erase_bit(
    a: &BitSlice<u8, Lsb0>,
    b: &BitSlice<u8, Lsb0>,
    c: &BitSlice<u8, Lsb0>,
) -> BitVec<u8, Lsb0> {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());

    let mut result = a.to_bitvec();

    // compute c & b in-place, store it in a temp BitVec
    let mut cb = c.to_bitvec();
    cb &= b.to_bitvec();

    // invert cb: now contains !(c & b)
    cb = !cb;

    // apply mask: result &= cb
    result &= cb;

    result
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

impl PixelColor {
    pub const ALPHA: PixelColor = PixelColor::new(false, false, false, false);
    pub const BLACK: PixelColor = PixelColor::new(false, false, false, true);
    pub const ERASE: PixelColor = PixelColor::new(true, false, false, false);
    pub const fn new(r: bool, g: bool, b: bool, a: bool) -> Self {
        Self { r, g, b, a }
    }
    pub const fn color(r: bool, g: bool, b: bool) -> Self {
        Self { r, g, b, a: false }
    }
    pub fn to_rgb(&self) -> &'static str {
        if !self.a {
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
    pub fn dimension(&self) -> (usize, usize) {
        self.r_array.dimensions()
    }
    /// Draw a single pixel (true = black, false = white).
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: PixelColor) {
        self.r_array.set(x, y, color.r);
        self.g_array.set(x, y, color.g);
        self.b_array.set(x, y, color.b);
        self.a_array.set(x, y, color.a);
    }
    pub fn merge_top(&mut self, top_layer: &Self) {
        assert!(self.dimension() == top_layer.dimension());
        self.r_array
            .assign_bits(select_bits(
                self.r_array.as_bytes(),
                top_layer.r_array.as_bytes(),
                self.a_array.as_bytes(),
            ))
            .unwrap();
        self.g_array
            .assign_bits(select_bits(
                self.g_array.as_bytes(),
                top_layer.g_array.as_bytes(),
                self.a_array.as_bytes(),
            ))
            .unwrap();
        self.b_array
            .assign_bits(select_bits(
                self.b_array.as_bytes(),
                top_layer.b_array.as_bytes(),
                self.a_array.as_bytes(),
            ))
            .unwrap();
        // TODO: Can be optimized later maybe.
        self.a_array
            .assign_bits(alpha_bits(
                self.a_array.as_bytes(),
                top_layer.a_array.as_bytes(),
                top_layer.r_array.as_bytes(),
            ))
            .unwrap();
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
        let a = self.b_array.get(x, y).unwrap_or(false);
        PixelColor { r, g, b, a }
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
