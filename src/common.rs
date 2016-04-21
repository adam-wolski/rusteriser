use std::path::Path;
use image;

/// Get 1D position from 2D coordinates.
/// Useful for setting/getting point in buffer.
#[inline]
pub fn xy(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

/// Convert array of 32 bit unsigned integers to 8 bit one.
/// Assumes that every 8 bits of 32bit integer is separate number.
pub fn vec32_to_8(input: &[u32]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for val in input {
        result.push(*val as u8);
        result.push((*val >> 8) as u8);
        result.push((*val >> 16) as u8);
        result.push((*val >> 24) as u8);
    }
    result
}

pub fn save_buffer_as_image(path: &Path, buffer: &[u32], width: u32, height: u32) {
    let clrtype = image::ColorType::RGBA(8);
    image::save_buffer(path, vec32_to_8(buffer).as_ref(), width, height, clrtype).unwrap();
}

/// Convert screen (-1 to 1) coordinates to image space (0 - screen size) based on image
/// width and height.
pub fn screen_to_image_space(x: f32, y: f32, width: u32, height: u32) -> (u32, u32) {
    assert!(x <= 1.0 && x >= -1.0 && y <= 1.0 && y >= -1.0);
    (((x + 1.0) / 2.0 * (width - 1) as f32) as u32,
     ((y + 1.0) / 2.0 * (height - 1) as f32) as u32)
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
}
