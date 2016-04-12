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
extern crate rand;


pub mod window;
pub mod line;
pub mod framebuffer;
pub mod model;
pub mod color;
pub mod common;
pub mod triangle;


use std::path::Path;
use std::thread;
use std::time;

use cgmath::EuclideanVector;

const WINDOW_WIDTH: u32 = 512;
const WINDOW_HEIGHT: u32 = 512;


fn simple_shade(face: &[cgmath::Vector3<f32>], light_dir: cgmath::Vector3<f32>) -> Option<color::Color> {
    let normal: cgmath::Vector3<f32> = (face[2] - face[0]).cross((face[1] - face[0]));
    normal.normalize();
    let intensity: f32 = normal.dot(light_dir) * 100.0;
    debug!("\nFace: {:?}\nNormal: {:?}\nLightDir: {:?}\nIntensity: {}\n", face, normal, light_dir, intensity);
    if intensity > 0.0 {
        let clr = (intensity * 255.0).round() as u8;
        debug!("\nColor: {}", clr);
        Some(color::Color::new(clr, clr, clr, 255))
    } else {
        None
    }
}


fn main() {
    env_logger::init().unwrap();

    let mut window = window::Window::new("Rusteriser", WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut framebuffer = framebuffer::Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let testmodelpath = Path::new("./content/african_head.obj");
    let testmodel = model::Model::load(testmodelpath);

    let lightdir = cgmath::Vector3::new(0.0, 0.0, -1.0);

    for face in &testmodel.faces {
        let mut newface: Vec<cgmath::Vector2<u32>> = Vec::with_capacity(3);
        for pos in face.iter().take(3) {
            let (x, y) = common::screen_to_image_space(pos.x, pos.y, WINDOW_WIDTH, WINDOW_HEIGHT);
            newface.push(cgmath::Vector2::new(x, y));
        }
        if let Some(color) = simple_shade(face.as_ref(), lightdir) {
            triangle::draw(&newface,
                           color,
                           &mut framebuffer);
        }
    }

    common::save_buffer_as_image(Path::new("./test_output/test.png"),
                                 framebuffer.data_as_ref(),
                                 WINDOW_WIDTH,
                                 WINDOW_HEIGHT);

    window.backbuffer_fill(framebuffer.data_as_ref());
    window.swap();
    while window.is_running() {
        thread::sleep(time::Duration::from_secs(1));
    }
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
