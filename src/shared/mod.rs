use bytemuck::{Pod, Zeroable};

/// Binary protocol for efficient drawing communication
/// Each message is exactly 9 bytes for optimal performance
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct DrawEvent {
    /// Message type: 0 = DrawLine, 1 = FullSync, 2 = Clear
    pub msg_type: u8,
    /// Start X coordinate (0-99 for 100x100 grid)
    pub x0: u8,
    /// Start Y coordinate (0-99 for 100x100 grid)
    pub y0: u8,
    /// End X coordinate (0-99 for 100x100 grid)
    pub x1: u8,
    /// End Y coordinate (0-99 for 100x100 grid)
    pub y1: u8,
    /// Drawing color: 0 = white/erase, 1 = black/draw
    pub is_black: u8,
    /// Padding for 8-byte alignment
    pub _padding: [u8; 3],
}

impl DrawEvent {
    /// Create a new draw line event
    pub fn draw_line(x0: usize, y0: usize, x1: usize, y1: usize, is_black: bool) -> Self {
        Self {
            msg_type: 0,
            x0: x0 as u8,
            y0: y0 as u8,
            x1: x1 as u8,
            y1: y1 as u8,
            is_black: if is_black { 1 } else { 0 },
            _padding: [0; 3],
        }
    }

    /// Create a full sync request event
    pub fn full_sync_request() -> Self {
        Self {
            msg_type: 1,
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
            is_black: 0,
            _padding: [0; 3],
        }
    }

    /// Create a clear canvas event
    pub fn clear_canvas() -> Self {
        Self {
            msg_type: 2,
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
            is_black: 0,
            _padding: [0; 3],
        }
    }

    /// Convert to bytes for transmission
    pub fn to_bytes(&self) -> [u8; 9] {
        bytemuck::cast(*self)
    }

    /// Create from bytes received
    pub fn from_bytes(bytes: &[u8; 9]) -> Self {
        bytemuck::cast(*bytes)
    }
}

/// Full canvas state for initial sync (100x100 bits = 1250 bytes)
/// Using a compact bit representation for the entire canvas
pub struct CanvasState {
    /// Bit array representing the 100x100 canvas (1 bit per pixel)
    /// Total size: 10,000 bits = 1,250 bytes
    pub data: [u8; 1250],
}

impl CanvasState {
    /// Create empty canvas
    pub fn new() -> Self {
        Self { data: [0; 1250] }
    }

    /// Set a pixel in the canvas state
    pub fn set_pixel(&mut self, x: usize, y: usize, is_black: bool) {
        if x >= 100 || y >= 100 {
            return;
        }
        
        let bit_index = y * 100 + x;
        let byte_index = bit_index / 8;
        let bit_offset = bit_index % 8;
        
        if is_black {
            self.data[byte_index] |= 1 << bit_offset;
        } else {
            self.data[byte_index] &= !(1 << bit_offset);
        }
    }

    /// Get a pixel from the canvas state
    pub fn get_pixel(&self, x: usize, y: usize) -> bool {
        if x >= 100 || y >= 100 {
            return false;
        }
        
        let bit_index = y * 100 + x;
        let byte_index = bit_index / 8;
        let bit_offset = bit_index % 8;
        
        (self.data[byte_index] & (1 << bit_offset)) != 0
    }

    /// Clear the entire canvas
    pub fn clear(&mut self) {
        self.data.fill(0);
    }

    /// Convert to bytes for transmission
    pub fn to_bytes(&self) -> &[u8; 1250] {
        &self.data
    }

    /// Create from bytes received
    pub fn from_bytes(bytes: &[u8; 1250]) -> Self {
        Self { data: *bytes }
    }
}
