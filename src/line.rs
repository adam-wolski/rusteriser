//! Line creation module.

/// Let's be a little bit fancy and create our line as an iterator instead of draw_line function. 
/// So we don't do drawing in the same function. 
/// All the computation of line goes in line but drawing can be anywhere else.
pub struct Line {
    x: i32,
    y: i32,
    d0: i32,
    d1: i32,
    e: i32,
    i: i32,
}

impl Line {
    /// Creates new line iterator in which each next item is a (x, y) tuple with coordinates of 
    /// next point in line.
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Line {
        let d0 = if x1 > x0 {
            x1 - x0
        } else {
            x0 - x1
        };
        let d1 = if y1 > y0 {
            y1 - y0
        } else {
            y0 - y1
        };
        Line {
            x: x0,
            y: y0,
            d0: d0,
            d1: d1,
            e: 2 * d1 - d0,
            i: 0,
        }
    }
}

impl Iterator for Line {
    type Item = (i32, i32);

    /// Return next line point
    fn next(&mut self) -> Option<(i32, i32)> {
        // Return first point
        if self.i == 0 {
            self.i += 1;
            return Some((self.x, self.y)) 
        }
        while self.e >= 0 {
            self.y += 1;
            self.e -= 2 * self.d0;
        }
        self.x += 1;
        self.e += 2 * self.d1;
        if self.i <= self.d0 {
            self.i += 1;
            Some((self.x, self.y))
        } else {
            None
        }
    }
}
