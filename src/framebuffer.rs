//! Framebuffer module.

use std::fmt;

use color::Color;

/// Let's define our framebuffer as struct so we can add some convenience methods for accessing it.
/// Data will be held as vector of u8s where each 4 represent one color in BGRA order.
pub struct Framebuffer {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Framebuffer {
    /// Create and initialize new framebuffer with specified width and height.
    pub fn new(width: u32, height: u32) -> Framebuffer {
        let data: Vec<u8> = vec!(0; (width * height * 4) as usize);
        Framebuffer {
            data: data,
            width: width,
            height: height,
        }
    }

    /// Set single index of buffer to value
    pub fn set(&mut self, i: usize, value: u8) {
        self.data[i] = value;
    }

    /// Set location in buffer to specified color.
    /// Location is taken in "buffer space"
    /// with x being: from 0 to buffer_width and y: 0 to buffer_height
    pub fn setxy(&mut self, x: u32, y: u32, color: Color) {
        assert!(x < self.width,
                "Given x({}) is bigger then buffer width({}).",
                x,
                self.width);
        assert!(y < self.height,
                "Given y({}) is bigger then buffer height({}).",
                y,
                self.height);
        let pos = ((x + y * self.width) * 4) as usize;
        self.data[pos] = color.b;
        self.data[pos + 1] = color.g;
        self.data[pos + 2] = color.r;
        self.data[pos + 3] = color.a;
    }

    /// Fill buffer with single color tuple (R, G, B, A).
    pub fn fill(&mut self, color: Color) {
        for i in (0..self.data.capacity()).filter(|i| i % 4 == 0) {
            self.data[i] = color.b;
            self.data[i + 1] = color.g;
            self.data[i + 2] = color.r;
            self.data[i + 3] = color.a;
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


/// Debugging formating for framebuffer.
/// Prints framebuffer as big list of hex values with associated indices.
/// This is quite a big list so it's mostly useful for testing on smaller scale or when you have
/// a really big screen ;)
impl fmt::Debug for Framebuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        result.push_str("Framebuffer:");
        for i in 0..self.data.len() {
            if i % (self.width * 4) as usize == 0 {
                result.push_str("\n");
            }
            if i % 4 == 0 {
                result.push_str(&format!("{:3}:{:2x}{:2x}{:2x}{:2x} ",
                                         i,
                                         self.data[i],
                                         self.data[i + 1],
                                         self.data[i + 2],
                                         self.data[i + 3]));
            }
        }
        write!(f, "{}", result)
    }
}
