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
pub mod utils;
pub mod triangle;
pub mod gl;


use std::path::Path;
use std::sync;

use cgmath::*;
use image::{Pixel, GenericImage};


const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 1024;


fn vertex_shader(inputs: gl::VSInput) -> gl::VSOutput {
    let v4 = inputs.position;
    let v_s_v = inputs.view * v4;
    let c_s_v = inputs.projection * v_s_v;
    
    let mut output: gl::VSOutput = gl::VSOutput::default();
    output.position = c_s_v;
    output.texcoord = inputs.texcoord;
    output.normal = inputs.normal;
    output
}

fn pixel_shader(inputs: gl::PSInput) -> Vector4<f32> {
    let texture = inputs.tex0.unwrap();
    let texcoord = inputs.texcoord;
    let normal = inputs.normal;
    let light_dir = inputs.light_pos;

    let (texwidth, texheight) = texture.dimensions();
    let (tx, ty) = utils::texcoord_to_image_space(texcoord.x, texcoord.y, texwidth, texheight);
    let t_clr = color::tup8_as_ranges(texture.get_pixel(tx, ty).channels4());

    let n = normal.normalize();
    let l = light_dir.normalize();
    let ndotl = utils::saturate(n.dot(l));

    Vector4::new(t_clr.0 * ndotl, t_clr.1 * ndotl, t_clr.2 * ndotl, t_clr.3)
}


fn main() {
    env_logger::init().unwrap();

    let mut graphics: gl::Gl = gl::Gl::new(WINDOW_WIDTH, WINDOW_HEIGHT);

    let camera: Vector3<f32> = Vector3::new(2.0, 0.0, 3.0);
    let camera_target: Vector3<f32> = Vector3::new(0.0, 0.0, 0.0);
    let up: Vector3<f32> = Vector3::new(0.0, 1.0, 0.0);
    let light_pos = Vector3::new(0.0, 0.0, 1.0);

    let view = gl::view_matrix(camera, camera_target, up);
    let mut projection: Matrix4<f32> = Matrix4::identity();
    projection[2][3] = -1.0 / camera.z;

    let modelpath = Path::new("./content/african_head/african_head.obj");
    // let modelpath = Path::new("./content/box.obj");
    let model = model::Model::load(modelpath).unwrap();

    let texture_image = image::open("./content/african_head/african_head_diffuse.tga").unwrap();
    let texture = sync::Arc::new(texture_image);

    let mut vs_in: gl::VSInput = gl::VSInput::default();
    vs_in.view = view;
    vs_in.projection = projection;
    vs_in.camera = camera;
    vs_in.camera_target = camera_target;

    let mut ps_in: gl::PSInput = gl::PSInput::default();
    ps_in.tex0 = Some(texture.clone());
    ps_in.light_pos = light_pos;

    graphics.draw(&model, vertex_shader, vs_in, pixel_shader, ps_in);
    graphics.save_framebuffer_as_image(Path::new("./test_output/test.png"));
    graphics.present();
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
                let (x0, y0) = utils::screen_to_image_space(face.verts[i % 3].pos.x,
                                                             face.verts[i % 3].pos.y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let (x1, y1) = utils::screen_to_image_space(face.verts[(i + 1) % 3].pos.x,
                                                             face.verts[(i + 1) % 3].pos.y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                line::draw(x0, y0, x1, y1, color, &mut fb, fb_width);
            }
        }
        utils::save_buffer_as_image(Path::new("./test_output/test_lines_iter.png"),
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
                let (x0, y0) = utils::screen_to_image_space(face.verts[i % 3].pos.x,
                                                             face.verts[i % 3].pos.y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let (x1, y1) = utils::screen_to_image_space(face.verts[(i + 1) % 3].pos.x,
                                                             face.verts[(i + 1) % 3].pos.y,
                                                             WINDOW_WIDTH,
                                                             WINDOW_HEIGHT);
                let line = line::LineIterator::new(x0, y0, x1, y1);
                for point in line {
                    fb[utils::xy(point.0, point.1, fb_width)] = color.bgra();
                }
            }
        }
        utils::save_buffer_as_image(Path::new("./test_output/test_lines_iter.png"),
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
                fb[utils::xy(point.0, point.1, fb_width)] = color.bgra();
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
        utils::save_buffer_as_image(Path::new("./test_output/bench_triangle.png"),
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
                    fb[utils::xy(point.0, point.1, fb_width)] = color.bgra();
                }
            }
        });
        utils::save_buffer_as_image(Path::new("./test_output/bench_triangle_iter.png"),
                                     &fb,
                                     WINDOW_WIDTH,
                                     WINDOW_HEIGHT);
    }
}
