//! Line creation module.

use std::mem;
use framebuffer;
use color;

/// Draw line using Bresenham algorithm.
pub fn draw(x0: u32,
            y0: u32,
            x1: u32,
            y1: u32,
            color: color::Color,
            target: &mut framebuffer::Framebuffer) {

    // Values of current point in line.
    let mut v0 = y0 as i32;
    let mut v1 = x0 as i32;

    let mut dir0: i32 = if y0 > y1 {
        -1
    } else {
        1
    };
    let mut dir1: i32 = if x0 > x1 {
        -1
    } else {
        1
    };

    let mut d0 = if x1 > x0 {
        x1 as i32 - x0 as i32
    } else {
        x0 as i32 - x1 as i32
    };

    let mut d1 = if y1 > y0 {
        y1 as i32 - y0 as i32
    } else {
        y0 as i32 - y1 as i32
    };

    let swapped = if d1 > d0 {
        mem::swap(&mut d1, &mut d0);
        mem::swap(&mut v1, &mut v0);
        mem::swap(&mut dir1, &mut dir0);
        true
    } else {
        false
    };

    let mut e = 2 * d1 - d0;
    if swapped {
        for _ in 0..d0 {
            target.setxy(v0 as u32, v1 as u32, color);
            while e >= 0 {
                v0 = v0 + dir0;
                e = e - 2 * d0;
            }
            v1 = v1 + dir1;
            e = e + 2 * d1;
        }
    } else {
        for _ in 0..d0 {
            target.setxy(v1 as u32, v0 as u32, color);
            while e >= 0 {
                v0 = v0 + dir0;
                e = e - 2 * d0;
            }
            v1 = v1 + dir1;
            e = e + 2 * d1;
        }
    }
}

/// Drawn line using Digital Differential Analyzer algorithm.
/// Just for future reference. Not as efficient as Bresenham algorithm.
fn _draw_dda(x1: i32, y1: i32, x2: i32, y2: i32, target: &mut framebuffer::Framebuffer) {
    let dx = if x1 > x2 {
        x1 - x2
    } else {
        x2 - x1
    };
    let dy = if y1 > y2 {
        y1 - y2
    } else {
        y2 - y1
    };
    let length = if dx > dy {
        dx
    } else {
        dy
    };
    let x_increment = (x2 - x1) as f32 / length as f32;
    let y_increment = (y2 - y1) as f32 / length as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;
    for _ in 0..length {
        target.setxy(x.round() as u32, y.round() as u32, color::Color::red());
        x = x + x_increment;
        y = y + y_increment;
    }
    target.setxy(x2 as u32, y2 as u32, color::Color::green());
}

/// Drawn line using Bresenham algorithm with floats and only with one quadrant working.
/// Just for future reference. Version without floats is more efficient.
fn _draw_bresenham_float(x1: i32,
                         y1: i32,
                         x2: i32,
                         y2: i32,
                         target: &mut framebuffer::Framebuffer) {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let mut e = dy as f32 / dx as f32 - 0.5;

    let mut x = x1;
    let mut y = y1;
    for _ in 0..dx {
        target.setxy(x as u32, y as u32, color::Color::blue());
        while e >= 0.0 {
            y = y + 1;
            e = e - 1.0;
        }
        x = x + 1;
        e = e + dy as f32 / dx as f32;
    }
}
