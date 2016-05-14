use std::path::Path;
use image;

/// Get 1D position from 2D coordinates.
/// Useful for setting/getting point in buffer.
#[inline]
pub fn xy(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

#[inline]
pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v > max {
        max
    } else if v < min {
        min
    } else {
        v
    }
}

#[inline]
pub fn saturate(v: f32) -> f32 {
    clamp(v, 0.0, 1.0)
}

/// Convert array of 32 bit unsigned integers to 8 bit one.
/// Assumes that every 8 bits of 32bit integer is separate number.
pub fn arr32_to_8(input: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for val in input {
        result.push(*val as u8);
        result.push((*val >> 8) as u8);
        result.push((*val >> 16) as u8);
        result.push((*val >> 24) as u8);
    }
    result
}

/// Convert buffer data `bf` with 32bit values to vector of 8bit values.
/// Assumes that order of colors in buffer is BGRA but output will be converted to RGBA order.
fn bf_to_image(bf: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for val in bf {
        result.push((*val >> 16) as u8);
        result.push((*val >> 8) as u8);
        result.push(*val as u8);
        result.push((*val >> 24) as u8);
    }
    result
}

/// Convert tuple of 8bit integers to one 32bit.
/// Useful for packing colors in arrays.
#[inline]
pub fn tup8_to_32(input: (u8, u8, u8, u8)) -> u32 {
    ((input.0 as u32) << 24) | ((input.1 as u32) << 16) | ((input.2 as u32) << 8) | (input.3 as u32)
}

pub fn save_buffer_as_image(path: &Path, buffer: &[u32], width: u32, height: u32) {
    let clrtype = image::ColorType::RGBA(8);
    image::save_buffer(path, bf_to_image(buffer).as_ref(), width, height, clrtype).unwrap();
}

/// Convert screen (-1 to 1) coordinates to image space (0 - screen size) based on image
/// width and height.
pub fn screen_to_image_space(x: f32, y: f32, width: u32, height: u32) -> (u32, u32) {
    assert!(x <= 1.0 && x >= -1.0,
            "x value: {}, is not a valid screen space coordinate",
            x);
    assert!(y <= 1.0 && y >= -1.0,
            "y value: {}, is not a vallid screen space coordinate",
            y);
    (((x + 1.0) / 2.0 * (width - 1) as f32) as u32,
     ((y + 1.0) / 2.0 * (height - 1) as f32) as u32)
}

/// Convert texcoord (0 to 1) coordinates to image space (0 - screen size) based on image
/// width and height.
pub fn texcoord_to_image_space(x: f32, y: f32, width: u32, height: u32) -> (u32, u32) {
    assert!(x <= 1.0 && x >= 0.0,
            "x value: {}, is not a valid texture coordinate",
            x);
    assert!(y <= 1.0 && y >= 0.0,
            "y value: {}, is not a valid texture coordinate",
            y);
    (((x * (width - 1) as f32) as u32,
      ((y * (height - 1) as f32) as u32)))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screen_to_image_space() {
        let width = 64;
        let height = 64;
        assert_eq!(screen_to_image_space(-1.0, 0.0, width, height), (0, 31));
        assert_eq!(screen_to_image_space(0.0, 1.0, width, height), (31, 63));
    }
    #[test]
    fn test_clamp() {
        assert!(clamp(5.0, 0.0, 1.0) <= 1.0);
        assert!(clamp(-1.0, 0.0, 1.0) >= 0.0);
    }
}