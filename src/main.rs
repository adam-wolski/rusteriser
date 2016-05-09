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
use image::{Pixel, GenericImage};


const WINDOW_WIDTH: u32 = 256;
const WINDOW_HEIGHT: u32 = 256;


fn pixel_shader(light_dir: Vector3<f32>,
                normal: Vector3<f32>,
                texcoord: Vector2<f32>,
                texture: &image::DynamicImage)
                -> Vector4<f32> {
    let (texwidth, texheight) = texture.dimensions();
    let (tx, ty) = common::texcoord_to_image_space(texcoord.x, texcoord.y, texwidth, texheight);
    let t_clr = color::as_ranges(texture.get_pixel(tx, ty).channels4());

    let n = normal.normalize();
    let l = light_dir.normalize();
    let ndotl = common::saturate(n.dot(l));

    Vector4::new(t_clr.0 * ndotl, t_clr.1 * ndotl, t_clr.2 * ndotl, t_clr.3)
}


/// Results returned from threads run per face.
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

    let camera: Vector3<f32> = Vector3::new(0.0, 0.0, 3.0);

    let mut projection: Matrix4<f32> = Matrix4::identity();

    let front_clip: f32 = 0.0;
    let near_clip: f32 = 1.0;
    let mut view: Matrix4<f32> = Matrix4::identity();
    view[0][0] = (WINDOW_WIDTH - 1) as f32 / 2.0;
    view[1][1] = (WINDOW_HEIGHT - 1) as f32 / 2.0;
    // view[2][2] = (front_clip - near_clip) / 2;
    view[3][0] = (WINDOW_WIDTH - 1) as f32 / 2.0;
    view[3][1] = (WINDOW_HEIGHT - 1) as f32 / 2.0;
    // view[3][2] = (near_clip + front_clip) / 2.0;

    let modelpath = Path::new("./content/african_head/african_head.obj");
    // let modelpath = Path::new("./content/box.obj");
    let model = model::Model::load(modelpath).unwrap();

    let texture_image = image::open("./content/african_head/african_head_diffuse.tga").unwrap();

    let lightdir = Vector3::new(0.0, 0.0, 1.0);

    let (tx, rx) = sync::mpsc::channel();

    let texture = sync::Arc::new(texture_image);

    for face in model.faces.clone() {
        let tx = tx.clone();
        let tex = texture.clone();
        thread::spawn(move || {
            let mut result = FaceThreadResult {
                bi: Vec::with_capacity(1000),
                fbv: Vec::with_capacity(1000),
                zbv: Vec::with_capacity(1000),
            };
            let mut face_cs: Vec<Vector3<f32>> = Vec::with_capacity(3);
            let mut face_img: Vec<Vector2<u32>> = Vec::with_capacity(3);
            for vertex in &face.verts {
                let v = view * vertex.pos.extend(1.0);
                face_cs.push(v.truncate());
                face_img.push(v.truncate().truncate().cast());
            }
            let triangle = triangle::TriangleIterator::new(&face_img);
            for line in triangle {
                for point in line {
                    let bary = match triangle::barycentric(Vector2::new(point.0 as f32,
                                                                        point.1 as f32),
                                                           &face_cs) {
                        Some(b) => b,
                        None => continue,
                    };
                    result.bi.push(common::xy(point.0, point.1, framebuffer_width));
                    result.zbv.push(face_cs[0].z * bary.x + face_cs[1].z * bary.y +
                                    face_cs[2].z * bary.z);
                    let texcoord = Vector2::<f32>::new(face.verts[0].texcoord.x * bary.x +
                                                       face.verts[1].texcoord.x * bary.y +
                                                       face.verts[2].texcoord.x * bary.z,
                                                       face.verts[0].texcoord.y * bary.x +
                                                       face.verts[1].texcoord.y * bary.y +
                                                       face.verts[2].texcoord.y * bary.z);
                    let normal = Vector3::<f32>::new(face.verts[0].normal.x * bary.x +
                                                     face.verts[1].normal.x * bary.y +
                                                     face.verts[2].normal.x * bary.z,
                                                     face.verts[0].normal.y * bary.x +
                                                     face.verts[1].normal.y * bary.y +
                                                     face.verts[2].normal.y * bary.z,
                                                     face.verts[0].normal.z * bary.x +
                                                     face.verts[1].normal.z * bary.y +
                                                     face.verts[2].normal.z * bary.z);
                    let pixel_color = pixel_shader(lightdir, normal, texcoord, &tex);
                    result.fbv.push(color::as_value(pixel_color));
                }
            }
            tx.send(result).unwrap();
        });
    }

    for _ in 0..model.faces.len() {
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
    window.backbuffer_fill(&common::arr32_to_8(&framebuffer));
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
        let testmodel = model::Model::load(testmodelpath).unwrap();
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
        let testmodel = model::Model::load(testmodelpath).unwrap();
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
