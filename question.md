you implemented this
```rust 

pub struct BitMatrix {
    width: usize,
    height: usize,
    data: BitVec,  // default Lsb0, usize store
}

impl BitMatrix {
    /// Create a new matrix of the specified dimensions, initialized with `initial` boolean.
    pub fn new(width: usize, height: usize, initial: bool) -> Self {
        let mut data = BitVec::with_capacity(width * height);
        data.resize(width * height, initial);
        BitMatrix { width, height, data }
    }
    
    /// Returns (width, height)
    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }
    
    /// Convert (x, y) to linear index
    fn idx(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }
    fn idx_unchecked(&self,x:usize,y:usize) ->usize{
        y*self.width+x
    }
    
    /// Get the value at (x, y). Returns Some(bool) or None if out-of-bounds.
    pub fn get(&self, x: usize, y: usize) -> Option<bool> {
        self.idx(x, y).map(|i| self.data[i])
    }
    
    /// Set the value at (x, y) to `value`, returns true if successful.
    pub fn set(&mut self, x: usize, y: usize, value: bool) -> bool {
        if let Some(i) = self.idx(x, y) {
            self.data.set(i, value);
            true
        } else {
            false
        }
    }
    
    /// Clear all bits to false
    pub fn clear(&mut self) {
        self.data.fill(false);
    }
    
    /// Count number of true bits
    pub fn count_ones(&self) -> usize {
        self.data.count_ones()
    }
    
    /// Iterate over rows as slices of bool
    pub fn row(&self, y: usize) -> Option<impl Iterator<Item=bool> + '_> {
        if y < self.height {
            let start = y * self.width;
            let end = start + self.width;
            let slice = &self.data[start..end];
            Some(slice.iter().by_vals())
        } else {
            None
        }
    }
}

impl Index<(usize, usize)> for BitMatrix {
    type Output = bool;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (x, y) = index;
        // BitVec supports indexing to &bool
        &self.data[self.idx_unchecked(x, y)]
    }
}

```
now make a PixelCanvas struct being a newtype of BitMatrix struct,It will have function like drawLine and drawPixel and draw only supports black and white,implement in those ideas.
