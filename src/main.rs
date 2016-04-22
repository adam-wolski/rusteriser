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
use std::sync;

use cgmath::*;

const WINDOW_WIDTH: u32 = 256;
const WINDOW_HEIGHT: u32 = 256;


fn simple_shade(face: &model::Face, light_dir: Vector3<f32>) -> color::Color {
    // Get the normal vector.
    let mut normal: Vector3<f32> = (face.verts[2].pos - face.verts[0].pos)
        .cross((face.verts[1].pos - face.verts[0].pos));
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
    pub bi: Vec<usize>, // Buffer index
    pub fbv: Vec<u32>, // Frame buffer values
    pub zbv: Vec<f32>, // Z Buffer values
}


fn main() {
    env_logger::init().unwrap();

    let mut window = window::Window::new("Rusteriser", WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut framebuffer: Vec<u32> = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];
    let framebuffer_width = WINDOW_WIDTH as usize;
    let mut zbuffer: Vec<f32> = vec![0.0; (WINDOW_WIDTH * WINDOW_HEIGHT) as usize];

    let testmodelpath = Path::new("./content/african_head/african_head.obj");
    let testmodel = model::Model::load(testmodelpath);

    let lightdir = Vector3::new(0.0, 0.0, -1.0);

    let (tx, rx) = sync::mpsc::channel();

    for face in testmodel.faces.clone() {
        let tx = tx.clone();
        thread::spawn(move || {
            let mut result = FaceThreadResult {
                bi: Vec::with_capacity(1000),
                fbv: Vec::with_capacity(1000),
                zbv: Vec::with_capacity(1000),
            };
            let mut image_face: Vec<Vector2<u32>> = Vec::with_capacity(3);
            let mut z: f32 = 0.0;
            for vertex in &face.verts {
                let (x, y) = common::screen_to_image_space(vertex.pos.x,
                                                           vertex.pos.y,
                                                           WINDOW_WIDTH,
                                                           WINDOW_HEIGHT);
                image_face.push(Vector2::new(x, y));
                z += vertex.pos.z;
            }
            let color = simple_shade(&face, lightdir);
            let triangle = triangle::TriangleIterator::new(&image_face);
            for line in triangle {
                for point in line {
                    result.bi.push(common::xy(point.0, point.1, framebuffer_width));
                    result.fbv.push(color.bgra());
                    result.zbv.push(z);
                }
            }
            tx.send(result).unwrap();
        });
    }

    for _ in 0..testmodel.faces.len() {
        let result: FaceThreadResult = rx.recv().unwrap();
        for i in 0..result.bi.len() {
            let bi = result.bi[i];
            let z_b_v = result.zbv[i];
            let f_b_v = result.fbv[i];
            if z_b_v > zbuffer[bi] {
                framebuffer[bi] = f_b_v;
                zbuffer[bi] = z_b_v;
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
    use cgmath::*;

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
                let (x0, y0) = common::screen_to_image_space(face.verts[i % 3].pos.x,
                                                             face.verts[i % 3].pos.y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let (x1, y1) = common::screen_to_image_space(face.verts[(i + 1) % 3].pos.x,
                                                             face.verts[(i + 1) % 3].pos.y,
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
                let (x0, y0) = common::screen_to_image_space(face.verts[i % 3].pos.x,
                                                             face.verts[i % 3].pos.y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let (x1, y1) = common::screen_to_image_space(face.verts[(i + 1) % 3].pos.x,
                                                             face.verts[(i + 1) % 3].pos.y,
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
        b.iter(|| line::draw(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT, color, &mut fb, fb_width))
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
        let mut tri: Vec<Vector2<u32>> = Vec::with_capacity(3);
        tri.push(Vector2::<u32>::new(0, 0));
        tri.push(Vector2::<u32>::new(0, WINDOW_HEIGHT));
        tri.push(Vector2::<u32>::new(WINDOW_WIDTH, WINDOW_HEIGHT));
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
        let mut tri: Vec<Vector2<u32>> = Vec::with_capacity(3);
        tri.push(Vector2::<u32>::new(0, 0));
        tri.push(Vector2::<u32>::new(0, WINDOW_HEIGHT - 1));
        tri.push(Vector2::<u32>::new(WINDOW_WIDTH - 1, WINDOW_HEIGHT - 1));

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
