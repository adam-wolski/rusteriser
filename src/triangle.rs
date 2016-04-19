use cgmath as cg;
use line;
use color;
use common;

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


// This one isn't that amazing. Should try finding something else.
fn point_in_triangle(point: (usize, usize), triangle: &[cg::Vector2<u32>]) -> bool {
    let p0 = cg::Vector2::<f32>::new(triangle[0].x as f32, triangle[0].y as f32);
    let p1 = cg::Vector2::<f32>::new(triangle[1].x as f32, triangle[1].y as f32);
    let p2 = cg::Vector2::<f32>::new(triangle[2].x as f32, triangle[2].y as f32);
    let p = cg::Vector2::<f32>::new(point.0 as f32, point.1 as f32);

    let a = 1.0 / 2.0 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
    let sign = if a < 0.0 {
        -1.0
    } else {
        1.0
    };
    let s = (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * p.x + (p0.x - p2.x) * p.y) * sign;
    let t = (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * p.x + (p1.x - p0.x) * p.y) * sign;

    s > 0.0 && t > 0.0 && (s + t) < 2.0 * a * sign
}

#[test]
fn test_point_in_triangle() {
    let mut tri: Vec<cg::Vector2<u32>> = Vec::with_capacity(3);
    tri.push(cg::Vector2::<u32>::new(245, 391));
    tri.push(cg::Vector2::<u32>::new(115, 200));
    tri.push(cg::Vector2::<u32>::new(306, 438));

    let mut point = (234, 357);
    assert!(point_in_triangle(point, tri.as_ref()));
    point = (236, 277);
    assert!(!point_in_triangle(point, tri.as_ref()));

    tri.clear();
    tri.push(cg::Vector2::<u32>::new(375, 186));
    tri.push(cg::Vector2::<u32>::new(2, 257));
    tri.push(cg::Vector2::<u32>::new(483, 5));

    point = (340, 110);
    assert!(point_in_triangle(point, tri.as_ref()));
    point = (288, 82);
    assert!(!point_in_triangle(point, tri.as_ref()));
}


/// Draw triangle from given vertex positions.
pub fn draw(triangle: &[cg::Vector2<u32>],
            color: color::Color,
            mut buffer: &mut [u32],
            buffer_width: usize) {

    let (bb_min_x, bb_min_y, bb_max_x, bb_max_y) = bounding_box(triangle);

    for y in bb_min_y..(bb_max_y) {
        let line = line::LineIterator::new(bb_min_x, y, bb_max_x, y);
        for point in line.filter(|p| point_in_triangle(*p, triangle)) {
            buffer[common::xy(point.0, point.1, buffer_width)] = color.bgra();
        }
    }
}


pub struct TriangleIterator<'a> {
    bb_min_x: u32,
    bb_max_x: u32,
    bb_max_y: u32,
    triangle: &'a [cg::Vector2<u32>],
    y: u32,
}

impl<'a> TriangleIterator<'a> {
    pub fn new(triangle: &'a [cg::Vector2<u32>]) -> TriangleIterator {
        let (bb_min_x, bb_min_y, bb_max_x, bb_max_y) = bounding_box(triangle);
        TriangleIterator {
            bb_min_x: bb_min_x,
            bb_max_x: bb_max_x,
            bb_max_y: bb_max_y,
            triangle: triangle,
            y: bb_min_y,
        }
    }
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = Vec<(usize, usize)>;

    fn next(&mut self) -> Option<Vec<(usize, usize)>> {
        if self.y > self.bb_max_y {
            return None;
        }
        self.y += 1;
        Some(line::LineIterator::new(self.bb_min_x, self.y, self.bb_max_x, self.y)
                 .filter(|p| point_in_triangle(*p, self.triangle))
                 .collect())
    }
}
