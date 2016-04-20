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
pub mod model;
pub mod color;
pub mod common;
pub mod triangle;


use std::path::Path;
use std::time;
use std::thread;

use cgmath::EuclideanVector;

const WINDOW_WIDTH: u32 = 256;
const WINDOW_HEIGHT: u32 = 256;


fn simple_shade(face: &[cgmath::Vector3<f32>], light_dir: cgmath::Vector3<f32>) -> color::Color {
    // Get the normal vector.
    let mut normal: cgmath::Vector3<f32> = (face[2] - face[0]).cross((face[1] - face[0]));
    normal = normal.normalize();
    let intensity: f32 = normal.dot(light_dir);
    if intensity > 0.0 {
        let clr = (intensity * 255.0).round() as u8;
        color::Color::new(clr, clr, clr, 255)
    } else {
        color::Color::new(0, 0, 0, 255)
    }
}


struct FaceThreadResult {
    bp: usize, // Buffer position
    fbv: u32,  // Frame buffer value
    zbv: f32, // Z Buffer value
}


fn main() {
    env_logger::init().unwrap();

    let mut window = window::Window::new("Rusteriser", WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut framebuffer: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
    let framebuffer_width = WINDOW_WIDTH as usize;
    let mut zbuffer: Vec<f32> = vec![0.0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
    let zbuffer_width = WINDOW_WIDTH as usize;

    let testmodelpath = Path::new("./content/box.obj");
    let testmodel = model::Model::load(testmodelpath);

    let lightdir = cgmath::Vector3::new(0.0, 0.0, -1.0);

    for face in &testmodel.faces {
        let mut image_face: Vec<cgmath::Vector2<u32>> = Vec::with_capacity(3);
        let mut z: f32 = 0.0;
        for pos in face.iter().take(3) {
            let (x, y) = common::screen_to_image_space(pos.x, pos.y, WINDOW_WIDTH, WINDOW_HEIGHT);
            image_face.push(cgmath::Vector2::new(x, y));
            z += pos.z;
        }
        let color = simple_shade(face.as_ref(), lightdir);
        let triangle = triangle::TriangleIterator::new(&image_face);
        for line in triangle {
            for point in line {
                if z > zbuffer[common::xy(point.0, point.1, zbuffer_width)] {
                    framebuffer[common::xy(point.0, point.1, framebuffer_width)] = color.bgra();
                    zbuffer[common::xy(point.0, point.1, zbuffer_width)] = z;
                }
            }
        }
    }

    common::save_buffer_as_image(Path::new("./test_output/test.png"),
                                 framebuffer.as_ref(),
                                 WINDOW_WIDTH,
                                 WINDOW_HEIGHT);
    window.backbuffer_fill(&common::vec32_to_8(&framebuffer));
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
        let mut fb: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
        let fb_width = WINDOW_WIDTH as usize;
        let color = color::Color::red();
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
                line::draw(x0, y0, x1, y1, color, &mut fb, fb_width);
            }
        }
        common::save_buffer_as_image(Path::new("./test_output/test_lines_iter.png"),
                                     &fb,
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }

    #[test]
    fn test_lines_iter() {
        let testmodelpath = Path::new("./content/monkey.obj");
        let testmodel = model::Model::load(testmodelpath);
        let mut fb: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
        let fb_width = WINDOW_WIDTH as usize;
        let color = color::Color::red();
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
                    fb[common::xy(point.0, point.1, fb_width)] = color.bgra();
                }
            }
        }
        common::save_buffer_as_image(Path::new("./test_output/test_lines_iter.png"),
                                     &fb,
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }

    #[bench]
    fn bench_line(b: &mut Bencher) {
        let mut fb: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
        let fb_width = WINDOW_WIDTH as usize;
        let color = color::Color::red();
        b.iter(|| {
            line::draw(0,
                       0,
                       WINDOW_WIDTH,
                       WINDOW_HEIGHT,
                       color,
                       &mut fb,
                       fb_width)
        })
    }

    #[bench]
    fn bench_line_iter(b: &mut Bencher) {
        let mut fb: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
        let fb_width = WINDOW_WIDTH as usize;
        let color = color::Color::red();
        b.iter(|| {
            let line = line::LineIterator::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
            for point in line {
                fb[common::xy(point.0, point.1, fb_width)] = color.bgra();
            }
        })
    }

    #[bench]
    fn bench_triangle(b: &mut Bencher) {
        let mut fb: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
        let fb_width = WINDOW_WIDTH as usize;
        let color = color::Color::red();
        let mut tri: Vec<cgmath::Vector2<u32>> = Vec::with_capacity(3);
        tri.push(cgmath::Vector2::<u32>::new(0, 0));
        tri.push(cgmath::Vector2::<u32>::new(0, WINDOW_HEIGHT));
        tri.push(cgmath::Vector2::<u32>::new(WINDOW_WIDTH, WINDOW_HEIGHT));
        b.iter(|| triangle::draw(&tri, color, &mut fb, fb_width));
        common::save_buffer_as_image(Path::new("./test_output/bench_triangle.png"),
                                     &fb,
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }

    #[bench]
    fn bench_triangle_iter(b: &mut Bencher) {
        let mut fb: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
        let fb_width = WINDOW_WIDTH as usize;
        let color = color::Color::red();
        let mut tri: Vec<cgmath::Vector2<u32>> = Vec::with_capacity(3);
        tri.push(cgmath::Vector2::<u32>::new(0, 0));
        tri.push(cgmath::Vector2::<u32>::new(0, WINDOW_HEIGHT));
        tri.push(cgmath::Vector2::<u32>::new(WINDOW_WIDTH, WINDOW_HEIGHT));

        b.iter(|| {
            let triangle = triangle::TriangleIterator::new(&tri);
            for line in triangle {
                for point in line {
                    fb[common::xy(point.0, point.1, fb_width)] = color.bgra();
                }
            }
        });
        common::save_buffer_as_image(Path::new("./test_output/bench_triangle_iter.png"),
                                     &fb,
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }
}
