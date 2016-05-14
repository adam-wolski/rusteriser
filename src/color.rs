//! Simple color structure.
use cgmath::Vector4;
use utils;


/// Represent color values as 0.0 - 1.0 ranges. For (u8, u8, u8, u8) tuple.
pub fn tup8_as_ranges(clr: (u8, u8, u8, u8)) -> (f32, f32, f32, f32) {
    (clr.0 as f32 / 255.0,
     clr.1 as f32 / 255.0,
     clr.2 as f32 / 255.0,
     clr.3 as f32 / 255.0)
}

/// Represent color as u8 values.
/// Made consistent with usage in shaders, where we return Vec4 instead of tuple.
pub fn v4_as_values(clr: Vector4<f32>) -> (u8, u8, u8, u8) {
    ((clr.x * 255.0).round().floor() as u8,
     (clr.y * 255.0).round().floor() as u8,
     (clr.z * 255.0).round().floor() as u8,
     (clr.w * 255.0).round().floor() as u8)
}

/// Represent color as u32 value.
/// Made consistent with usage in shaders, where we return Vec4 instead of tuple.
/// Assumes BGRA order of colors.
pub fn v4_as_value(clr: Vector4<f32>) -> u32 {
    let clr_u8 = ((clr.w * 255.0).round().floor() as u8,
                  (clr.x * 255.0).round().floor() as u8,
                  (clr.y * 255.0).round().floor() as u8,
                  (clr.z * 255.0).round().floor() as u8);
    utils::tup8_to_32(clr_u8)
}


#[derive(Debug, Default, Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    /// u32 value with color bits in order blue, green, red, alpha.
    pub fn bgra(&self) -> u32 {
        utils::tup8_to_32((self.a, self.r, self.g, self.b))
    }

    pub fn rgba(&self) -> u32 {
        utils::tup8_to_32((self.a, self.b, self.g, self.r))
    }

    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub fn transparent() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}
