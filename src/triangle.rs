use cgmath as cg;
use line;
use color;
use framebuffer;

/// Returns bounding box as tuple `(min_x, min_y, max_x, max_y)`
/// # Panics
/// * Not being able to find max or min value.
/// * Or anything else really, full of unwrap.
fn bounding_box(positions: &[cg::Vector2<u32>]) -> (u32, u32, u32, u32) {
    let min_x = positions.iter().map(|pos| pos.x).min().unwrap();
    let min_y = positions.iter().map(|pos| pos.y).min().unwrap();
    let max_x = positions.iter().map(|pos| pos.x).max().unwrap();
    let max_y = positions.iter().map(|pos| pos.y).max().unwrap();
    (min_x, min_y, max_x, max_y)
}

/// Draw triangle from given vertex positions.
pub fn draw(positions: &[cg::Vector2<u32>],
            color: color::Color,
            mut framebuffer: &mut framebuffer::Framebuffer) {
    let (bb_min_x, bb_min_y, bb_max_x, bb_max_y) = bounding_box(positions);
    debug!("{} {} {} {}", bb_min_x, bb_min_y, bb_max_x, bb_max_y);

    for y in bb_min_y..bb_max_y {
        line::draw(bb_min_x, y, bb_max_x, y, color, &mut framebuffer);
    }
}
