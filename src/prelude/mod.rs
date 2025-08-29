use leptos::{logging::warn};
use codee::{Decoder, Encoder};
use web_sys::window;
pub mod geometry;
pub use geometry::*;
pub mod bitmap;
pub use bitmap::*;
pub fn get_window_size() -> RectSize {
    const DEFAULT_WIDTH: f64 = 800.0;
    const DEFAULT_HEIGHT: f64 = 600.0;

    let Some(win) = window() else {
        warn!("Window object not available, using default dimensions");
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

pub struct BytesPassthrough;
impl codee::Encoder<Vec<u8>> for BytesPassthrough {
    type Error = core::convert::Infallible;
    type Encoded = Vec<u8>;
    fn encode(val: &Vec<u8>) -> Result<Vec<u8>, Self::Error> { Ok(val.clone()) }
}

impl codee::Decoder<Vec<u8>> for BytesPassthrough {
    type Error = core::convert::Infallible;
    type Encoded = [u8];
    fn decode(val: &Self::Encoded) -> Result<Vec<u8>, Self::Error> { Ok(val.to_vec()) }
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
        m.set_to_one();
        assert_eq!(m.count_ones(), 0);

        // row iteration
        m.set(0,2, true);
        
    }
}

