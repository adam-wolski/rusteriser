#![feature(test)]
#![feature(plugin)]
#![plugin(clippy)]

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate sdl2;
extern crate tobj;
extern crate cgmath;
extern crate image;
extern crate test;

pub mod window;
pub mod line;
pub mod framebuffer;
pub mod model;
pub mod color;
pub mod common;
pub mod triangle;

use std::path::Path;


const WINDOW_WIDTH: u32 = 256;
const WINDOW_HEIGHT: u32 = 256;


fn main() {
    env_logger::init().unwrap();

    let mut window = window::Window::new("Rusteriser", WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let testmodelpath = Path::new("./content/monkey.obj");
    let testmodel = model::Model::load(testmodelpath);

    for face in &testmodel.faces {
        let mut newface: Vec<cgmath::Vector2<u32>> = Vec::with_capacity(3);
        for pos in face.iter().take(3) {
            let (x, y) = common::screen_to_image_space(pos.x, pos.y, WINDOW_WIDTH, WINDOW_HEIGHT);
            newface.push(cgmath::Vector2::new(x, y));
        }

        triangle::draw(&newface, color::Color::white(), &mut framebuffer);
    }

    window.backbuffer_fill(framebuffer.data_as_ref());
    window.swap();
    common::save_buffer_as_image(Path::new("./test_output/test.png"),
                                 framebuffer.data_as_ref(),
                                 WINDOW_WIDTH,
                                 WINDOW_HEIGHT);
    while window.is_running() {}
}


#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;
    use std::path::Path;
    use cgmath;

    const WINDOW_WIDTH: u32 = 512;
    const WINDOW_HEIGHT: u32 = 512;

    #[test]
    fn test_lines() {
        let testmodelpath = Path::new("./content/monkey.obj");
        let testmodel = model::Model::load(testmodelpath);

        let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let color = color::Color::white();

        for face in &testmodel.faces {
            for i in 0..3 {
                let (x0, y0) = common::screen_to_image_space(face[i % 3].x,
                                                             face[i % 3].y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let (x1, y1) = common::screen_to_image_space(face[(i + 1) % 3].x,
                                                             face[(i + 1) % 3].y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                line::draw(x0, y0, x1, y1, color, &mut framebuffer);
            }
        }
        common::save_buffer_as_image(Path::new("./test_output/test_lines.png"),
                                     framebuffer.data_as_ref(),
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }

    #[test]
    fn test_lines_iter() {
        let testmodelpath = Path::new("./content/monkey.obj");
        let testmodel = model::Model::load(testmodelpath);

        let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let color = color::Color::white();

        for face in &testmodel.faces {
            for i in 0..3 {
                let (x0, y0) = common::screen_to_image_space(face[i % 3].x,
                                                             face[i % 3].y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let (x1, y1) = common::screen_to_image_space(face[(i + 1) % 3].x,
                                                             face[(i + 1) % 3].y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let line = line::LineIterator::new(x0, y0, x1, y1);
                for point in line {
                    framebuffer.setxy(point.0 as u32, point.1 as u32, color);
                }
            }
        }
        common::save_buffer_as_image(Path::new("./test_output/test_lines_iter.png"),
                                     framebuffer.data_as_ref(),
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }

    #[bench]
    fn bench_line(b: &mut Bencher) {
        let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let color = color::Color::white();
        b.iter(|| {
            line::draw(0,
                       0,
                       WINDOW_WIDTH - 1,
                       WINDOW_HEIGHT - 1,
                       color,
                       &mut framebuffer)
        });
    }

    #[bench]
    fn bench_line_iter(b: &mut Bencher) {
        let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let color = color::Color::white();
        b.iter(|| {
            let line = line::LineIterator::new(0, 0, WINDOW_WIDTH - 1, WINDOW_HEIGHT - 1);
            for point in line {
                framebuffer.setxy(point.0 as u32, point.1 as u32, color);
            }
        })
    }

    #[bench]
    fn bench_triangle(b: &mut Bencher) {
        let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
        let color = color::Color::white();
        let mut tri: Vec<cgmath::Vector2<u32>> = Vec::with_capacity(3);
        tri.push(cgmath::Vector2::<u32>::new(0, 0));
        tri.push(cgmath::Vector2::<u32>::new(0, WINDOW_HEIGHT - 1));
        tri.push(cgmath::Vector2::<u32>::new(WINDOW_WIDTH - 1, WINDOW_HEIGHT - 1));
        b.iter(|| triangle::draw(&tri, color, &mut framebuffer));
    }
}
