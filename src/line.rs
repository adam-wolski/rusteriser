//! Line creation module.

use std::mem;
use color;
use utils;


/// Draw line using Bresenham algorithm.
pub fn draw(x0: u32,
            y0: u32,
            x1: u32,
            y1: u32,
            color: color::Color,
            buffer: &mut [u32],
            buffer_width: usize) {

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
            buffer[utils::xy(v0 as usize, v1 as usize, buffer_width)] = color.bgra();
            while e >= 0 {
                v0 = v0 + dir0;
                e = e - 2 * d0;
            }
            v1 = v1 + dir1;
            e = e + 2 * d1;
        }
    } else {
        for _ in 0..d0 {
            buffer[utils::xy(v1 as usize, v0 as usize, buffer_width)] = color.bgra();
            while e >= 0 {
                v0 = v0 + dir0;
                e = e - 2 * d0;
            }
            v1 = v1 + dir1;
            e = e + 2 * d1;
        }
    }
}


/// Iterator with each next item being next point in line.
/// Line is created using Bresenham algorithm same as in `line::draw`.
pub struct LineIterator {
    d0: i32,
    d1: i32,
    e: i32,
    v0: i32,
    v1: i32,
    dir0: i32,
    dir1: i32,
    swapped: bool,
    count: i32,
}

impl LineIterator {
    pub fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> LineIterator {
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

        let e = 2 * d1 - d0;

        LineIterator {
            d0: d0,
            d1: d1,
            e: e,
            v0: v0,
            v1: v1,
            dir0: dir0,
            dir1: dir1,
            swapped: swapped,
            count: 0,
        }
    }
}

impl Iterator for LineIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<(usize, usize)> {
        if self.count >= self.d0 {
            return None;
        }
        if self.count == 0 {
            self.count += 1;
            if self.swapped {
                return Some((self.v0 as usize, self.v1 as usize));
            } else {
                return Some((self.v1 as usize, self.v0 as usize));
            }
        }

        while self.e >= 0 {
            self.v0 = self.v0 + self.dir0;
            self.e = self.e - 2 * self.d0
        }

        self.v1 = self.v1 + self.dir1;
        self.e = self.e + 2 * self.d1;

        self.count += 1;

        if self.swapped {
            Some((self.v0 as usize, self.v1 as usize))
        } else {
            Some((self.v1 as usize, self.v0 as usize))
        }
    }
}
