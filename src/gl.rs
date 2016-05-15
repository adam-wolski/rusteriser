use std::sync;
use std::path;
use std::thread;
use std::time;

use cgmath::*;

use window;
use image;
use model;
use triangle;
use utils;
use color;


// TODO: Give this as parametes somewhere.
const CLIP_FAR: f32 = 99.0;
const CLIP_NEAR: f32 = 0.0;


#[derive(Debug, Clone, Copy)]
pub struct VSInput {
    pub position: Vector4<f32>,
    pub texcoord: Vector2<f32>,
    pub normal: Vector4<f32>,
    pub camera: Vector3<f32>,
    pub camera_target: Vector3<f32>,
    // Space transformation matrices
    pub view: Matrix4<f32>,
    pub projection: Matrix4<f32>,
}

impl Default for VSInput {
    fn default() -> VSInput {
        VSInput {
            position: Vector4::new(0.0, 0.0, 0.0, 1.0),
            normal: Vector4::new(1.0, 1.0, 1.0, 0.0),
            texcoord: Vector2::new(0.0, 0.0),
            camera: Vector3::new(0.0, 0.0, 0.0),
            camera_target: Vector3::new(0.0, 0.0, 0.0),
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VSOutput {
    pub position: Vector4<f32>,
    pub normal: Vector4<f32>,
    pub texcoord: Vector2<f32>,
}

impl Default for VSOutput {
    fn default() -> VSOutput {
        VSOutput {
            position: Vector4::new(0.0, 0.0, 0.0, 1.0),
            normal: Vector4::new(1.0, 1.0, 1.0, 0.0),
            texcoord: Vector2::new(0.0, 0.0),
        }
    }
}

#[derive(Clone)]
pub struct PSInput {
    pub textures: Vec<sync::Arc<image::DynamicImage>>,
    pub light_pos: Vector3<f32>,
    pub cam_dir: Vector3<f32>,
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub texcoord: Vector2<f32>,
}

impl Default for PSInput {
    fn default() -> PSInput {
        PSInput {
            textures: Vec::new(),
            light_pos: Vector3::new(0.0, 0.0, 0.0),
            cam_dir: Vector3::new(0.0, 0.0, 1.0),
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(1.0, 1.0, 1.0),
            texcoord: Vector2::new(0.0, 0.0),
        }
    }
}


/// Construct View matrix which transforms from world space to view space.
pub fn view_matrix(camera: Vector3<f32>,
                   camera_target: Vector3<f32>,
                   up: Vector3<f32>)
                   -> Matrix4<f32> {
    let z_axis = (camera - camera_target).normalize();
    let x_axis = (up.cross(z_axis)).normalize();
    let y_axis = z_axis.cross(x_axis);
    Matrix4::from_cols(Vector4::new(x_axis.x, y_axis.x, z_axis.x, 0.0),
                       Vector4::new(x_axis.y, y_axis.y, z_axis.y, 0.0),
                       Vector4::new(x_axis.z, y_axis.z, z_axis.z, 0.0),
                       Vector4::new(-x_axis.dot(camera),
                                    -y_axis.dot(camera),
                                    -z_axis.dot(camera),
                                    1.0))
}

pub fn projection_matrix(fovy: f32) -> Matrix4<f32> {
    let d = 1.0 / (fovy.tan() / 2.0);
    let aspect_ratio = 1.0;
    let mut projection = Matrix4::identity();
    projection[0][0] = d / aspect_ratio;
    projection[1][1] = d;
    projection[2][2] = (CLIP_NEAR + CLIP_FAR) / (CLIP_NEAR - CLIP_FAR);
    projection[3][2] = 2.0 * CLIP_NEAR * CLIP_FAR / (CLIP_NEAR - CLIP_FAR);
    projection[2][3] = -1.0;
    projection
}

/// Construct viewport transformation matrix which translates ndc to screen/window coordinates.
pub fn viewport_matrix(window_dimensions: (u32, u32),
                       clip_near: f32,
                       clip_far: f32)
                       -> Matrix4<f32> {
    let mut viewport: Matrix4<f32> = Matrix4::identity();
    let (window_width, window_height) = window_dimensions;
    viewport[0][0] = (window_width - 1) as f32 / 2.0;
    viewport[1][1] = (window_height - 1) as f32 / 2.0;
    viewport[2][2] = (clip_far - clip_near) / 2.0;
    viewport[3][0] = (window_width - 1) as f32 / 2.0;
    viewport[3][1] = (window_height - 1) as f32 / 2.0;
    viewport[3][2] = (clip_near + clip_far) / 2.0;
    viewport
}


/// Results returned from threads run per face.
struct FaceThreadResult {
    pub bi: Vec<usize>, // Buffer index
    pub fbv: Vec<u32>, // Frame buffer values
    pub zbv: Vec<f32>, // Z Buffer values
}


pub struct Gl<'a> {
    window: window::Window<'a>,
    fb: Vec<u32>,
    fb_width: usize,
    zb: Vec<f32>,
}

impl<'a> Gl<'a> {
    pub fn new(window_width: u32, window_height: u32) -> Gl<'a> {
        let window = window::Window::new("Rusteriser", window_width, window_height);
        let framebuffer: Vec<u32> = vec![0; (window_width * window_height) as usize];
        let framebuffer_width = window_width as usize;
        let zbuffer: Vec<f32> = vec![-99999999.0; (window_width * window_height) as usize];
        Gl {
            window: window,
            fb: framebuffer,
            fb_width: framebuffer_width,
            zb: zbuffer,
        }
    }

    pub fn draw<V, P>(&mut self,
                      model: &model::Model,
                      vertex_shader: V,
                      vertex_shader_input: VSInput,
                      pixel_shader: P,
                      pixel_shader_input: PSInput)
        where V: Fn(VSInput) -> VSOutput + Send + Copy + 'static,
              P: Fn(PSInput) -> Vector4<f32> + Send + Copy + 'static
    {

        let viewport: Matrix4<f32> = viewport_matrix(self.window.dimensions(), CLIP_NEAR, CLIP_FAR);

        let (tx, rx) = sync::mpsc::channel();
        let fb_width = self.fb_width;

        for face in model.faces.clone() {
            let tx = tx.clone();

            let mut ps_input = pixel_shader_input.clone();
            let mut vs_input = vertex_shader_input;
            thread::spawn(move || {
                let mut result = FaceThreadResult {
                    bi: Vec::with_capacity(1000),
                    fbv: Vec::with_capacity(1000),
                    zbv: Vec::with_capacity(1000),
                };

                let mut face_ss: Vec<Vector3<f32>> = Vec::with_capacity(3);
                let mut face_2d: Vec<Vector2<u32>> = Vec::with_capacity(3);
                let mut texcoords: Vec<Vector2<f32>> = Vec::with_capacity(3);
                let mut normals: Vec<Vector3<f32>> = Vec::with_capacity(3);
                for vertex in &face.verts {
                    vs_input.position = vertex.pos.extend(1.0);
                    vs_input.normal = vertex.normal.extend(0.0);
                    vs_input.texcoord = vertex.texcoord;
                    let vs_out: VSOutput = vertex_shader(vs_input);
                    let vs_pos = vs_out.position;
                    let ndc_v = Vector4::<f32>::new(vs_pos.x / vs_pos.w,
                                                    vs_pos.y / vs_pos.w,
                                                    vs_pos.z / vs_pos.w,
                                                    1.0);
                    let s_s_v = viewport * ndc_v;
                    let mut v3: Vector3<f32> = s_s_v.truncate();
                    v3.x = v3.x.round();
                    v3.y = v3.y.round();
                    face_ss.push(v3);
                    face_2d.push(v3.truncate().cast());
                    normals.push(vs_out.normal.truncate());
                    texcoords.push(vs_out.texcoord);
                }

                let triangle = triangle::TriangleIterator::new(&face_2d);
                for line in triangle {
                    for point in line {
                        let bary = match triangle::barycentric(Vector2::new(point.0 as f32,
                                                                            point.1 as f32),
                                                               &face_ss) {
                            Some(b) => b,
                            None => continue,
                        };
                        result.bi.push(utils::xy(point.0, point.1, fb_width));
                        result.zbv.push(face_ss[0].z * bary.x + face_ss[1].z * bary.y +
                                        face_ss[2].z * bary.z);

                        let texcoord = utils::vector2_interpolate(&texcoords, &bary);
                        let normal = utils::vector3_interpolate(&normals, &bary);
                        let position = utils::vector3_interpolate(&face_ss, &bary);

                        ps_input.texcoord = texcoord;
                        ps_input.normal = normal;
                        ps_input.position = position;

                        // We can't just copy pixel_shader_input as it stores textures as Arc.
                        let pixel_color = pixel_shader(ps_input.clone());
                        result.fbv.push(color::v4_as_value(pixel_color));
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
                if z_b_v >= self.zb[bi] {
                    self.fb[bi] = f_b_v;
                    self.zb[bi] = z_b_v;
                }
            }
        }
    }

    pub fn save_framebuffer_as_image(&self, path: &path::Path) {
        let (window_width, window_height) = self.window.dimensions();
        utils::save_buffer_as_image(path, &self.fb, window_width, window_height);
    }

    pub fn present(&mut self) {
        self.window.backbuffer_fill(&utils::arr32_to_8(&self.fb));
        self.window.swap();

        while self.window.is_running() {
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
