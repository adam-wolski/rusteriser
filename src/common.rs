use std::path::Path;
use image;

pub fn save_buffer_as_image(path: &Path, buffer: &[u8], width: u32, height: u32) {
    let clrtype = image::ColorType::RGBA(8);
    image::save_buffer(path, buffer, width, height, clrtype).unwrap();
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
        assert!(screen_to_image_space(-1.0, 0.0, width, height) == (0, 31));
        assert!(screen_to_image_space(0.0, 1.0, width, height) == (31, 63));
    }
}
