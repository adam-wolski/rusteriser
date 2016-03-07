//! Backbuffer module.

use std::fmt;


/// Let's define our backbuffer as struct so we can add some convenience methods for accessing it.
/// Data will be held as some vector of u8ts where each 4 represent one color in BGRA order.
pub struct Backbuffer {
    data: Vec<u8>,
    width: usize,
}

impl Backbuffer {
    /// Create and initialize new backbuffer with specified width and height.
    pub fn new(width: usize, height: usize) -> Backbuffer {
        let mut data: Vec<u8> = Vec::with_capacity(width * height * 4);

        // Fill with black
        for _ in 0..data.capacity() {
            data.push(0);
        }
        Backbuffer {
            data: data,
            width: width,
        }

    }

    /// Set single index of buffer to value
    pub fn set(&mut self, i: usize, value: u8) {
        self.data[i] = value;
    }

    /// Set location (x, y) in buffer to specified color tuple (R, G, B, A).
    pub fn set_loc(&mut self, loc: (usize, usize), color: (u8, u8, u8, u8)) {
        let pos = (loc.0 + loc.1 * self.width) * 4;
        debug!("Setting location: {:?} at index {}", loc, pos);
        self.data[pos] = color.2; // Blue
        self.data[pos + 1] = color.1; // Green
        self.data[pos + 2] = color.0; // Red
        self.data[pos + 3] = color.3; // Alpha
    }

    /// Fill buffer with single color tuple (R, G, B, A).
    pub fn fill(&mut self, color: (u8, u8, u8, u8)) {
        for i in (0..self.data.capacity()).filter(|i| i % 4 == 0) {
            self.data[i] = color.2; // Blue
            self.data[i + 1] = color.1; // Green
            self.data[i + 2] = color.0; // Red
            self.data[i + 3] = color.3; // Alpha
        }
    }

    /// Return stored data as reference.
    pub fn data_as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }

    /// Return stored data as copy of a vector.
    pub fn data_copy(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}


/// Debugging formating for backbuffer.
/// Prints backbuffer as big list of hex values with associated indices.
/// This is quite a big list so it's mostly useful for testing on smaller scale or when you have
/// a really big monitor ;)
impl fmt::Debug for Backbuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("Backbuffer:");
        for i in 0..self.data.len() {
            if i % (self.width * 4) == 0 {
                result.push_str("\n");
            }
            if i % 4 == 0 {
                result.push_str(&format!("{:3}:{:2x}{:2x}{:2x}{:2x} ",
                                         i,
                                         self.data[i],
                                         self.data[i + 1],
                                         self.data[i + 2],
                                         self.data[i + 2]));
            }
        }
        write!(f, "{}", result)
    }
}
